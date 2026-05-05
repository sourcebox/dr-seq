//! Sequencer track.

use heapless::spsc::Queue;

use crate::params::{Pitch, Velocity};
use crate::step::Step;

/// Capacity of the event queue.
pub const EVENT_QUEUE_CAPACITY: usize = 16;

/// Event queue.
type EventQueue = Queue<TrackEvent, EVENT_QUEUE_CAPACITY>;

/// Sequencer track.
#[derive(Default, Debug)]
pub struct Track {
    /// Last played step number.
    play_step: Option<usize>,

    /// Monotonic pulse count. Used for note off scheduling.
    pulse_count: u32,

    /// Pulse number and pitch for next note off event.
    scheduled_note_off: Option<(u32, Pitch)>,

    /// Queue for generated events.
    event_queue: EventQueue,
}

impl Clone for Track {
    /// Clones the track data but creates a new event queue.
    fn clone(&self) -> Self {
        Self {
            play_step: None,
            pulse_count: 0,
            scheduled_note_off: None,
            event_queue: EventQueue::new(),
        }
    }
}

impl Track {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the track when a clock pulse occurs.
    pub fn update(&mut self, pulse_no: u32, ppq: u32, steps: &[Step], options: &TrackOptions) {
        let mut pulse_no = pulse_no as i32 - options.delay;

        // Apply swing value to each 2nd step.
        if pulse_no / (ppq as i32 / 4) % 2 == 1 {
            pulse_no -= options.swing;
        }

        // Make sure pulse no is always positive.
        let pulse_no = pulse_no.max(0) as usize;

        let mut shift = options.shift;
        while shift < 0 {
            shift += steps.len() as i32;
        }

        // Do some calculations to determine where we are.
        let mut play_step = (pulse_no / (ppq as usize / 4) + shift as usize) % steps.len();

        // Apply reverse option.
        if options.reverse {
            play_step = steps.len() - 1 - play_step;
        }

        // Apply re-sort function.
        if let Some(f) = options.resort_fn {
            play_step = f(play_step);
        }

        // Check if a previously started note has reached its length.
        if let Some(scheduled_note_off) = self.scheduled_note_off {
            if self.pulse_count == scheduled_note_off.0 {
                self.event_queue
                    .enqueue(TrackEvent::NoteOff {
                        step: play_step,
                        pitch: scheduled_note_off.1,
                    })
                    .ok();
                self.scheduled_note_off = None;
            }
        }

        if options.enable && (self.play_step.is_none() || play_step != self.play_step.unwrap()) {
            self.play_step = Some(play_step);

            let step = &steps[play_step];

            if step.enabled() {
                // If a note is still playing, it must be stopped before triggering a new one.
                if let Some(scheduled_note_off) = self.scheduled_note_off {
                    self.event_queue
                        .enqueue(TrackEvent::NoteOff {
                            step: play_step,
                            pitch: scheduled_note_off.1,
                        })
                        .ok();
                    self.scheduled_note_off = None;
                }

                // Start a new note.
                self.event_queue
                    .enqueue(TrackEvent::NoteOn {
                        step: play_step,
                        pitch: step.pitch(),
                        vel: step.velocity(),
                    })
                    .ok();

                // Schedule the note off for a 1/32 note length.
                let note_off_pulse = self.pulse_count + ppq / 8;
                self.scheduled_note_off = Some((note_off_pulse, step.pitch()));
            }
        }

        // Pulse count is allowed to overflow explicitly.
        // But it can take a while until that happens.
        self.pulse_count = self.pulse_count.wrapping_add(1);
    }

    /// Returns the next event.
    pub fn next_event(&mut self) -> Option<TrackEvent> {
        self.event_queue.dequeue()
    }

    /// Flushes sustained notes.
    pub fn flush(&mut self) {
        if let Some(scheduled_note_off) = self.scheduled_note_off {
            let play_step = self.play_step.unwrap_or_default();
            self.event_queue
                .enqueue(TrackEvent::NoteOff {
                    step: play_step,
                    pitch: scheduled_note_off.1,
                })
                .ok();
            self.scheduled_note_off = None;
        }
    }

    /// Returns the last played step number.
    pub fn play_step(&self) -> Option<usize> {
        self.play_step
    }
}

/// Options for playback.
#[derive(Debug, Default, Clone)]
pub struct TrackOptions {
    /// Enable the playback.
    pub enable: bool,

    /// Swing offset in pulses.
    pub swing: i32,

    /// Time delay in pulses.
    pub delay: i32,

    /// Steps shift.
    pub shift: i32,

    /// Reverse the playback direction.
    pub reverse: bool,

    /// Function to re-sort the steps.
    pub resort_fn: Option<fn(usize) -> usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrackEvent {
    /// Start a note with pitch and velocity.
    NoteOn {
        step: usize,
        pitch: Pitch,
        vel: Velocity,
    },

    /// Stop a note with pitch.
    NoteOff { step: usize, pitch: Pitch },
}
