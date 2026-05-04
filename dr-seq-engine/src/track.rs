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
    /// Flag if track is enabled for playing.
    enabled: bool,

    /// Last play position as step number.
    play_pos: Option<usize>,

    /// Swing offset in pulses.
    swing: i32,

    /// Delay in pulses.
    delay: i32,

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
            enabled: false,
            play_pos: None,
            swing: self.swing,
            delay: self.delay,
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
    pub fn update(&mut self, pulse_no: u32, ppq: u32, steps: &[Step]) {
        let mut pulse_no = pulse_no as i32 - self.delay;

        // Apply swing value to each 2nd step.
        if pulse_no / (ppq as i32 / 4) % 2 == 1 {
            pulse_no -= self.swing;
        }

        // Do some calculations to determine where we are.
        let play_pos = (pulse_no / (ppq as i32 / 4) % steps.len() as i32) as usize;
        let play_step = play_pos % steps.len();

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

        if self.enabled && (self.play_pos.is_none() || play_pos != self.play_pos.unwrap()) {
            self.play_pos = Some(play_pos);

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
            let play_pos = self.play_pos.unwrap_or_default();
            self.event_queue
                .enqueue(TrackEvent::NoteOff {
                    step: play_pos,
                    pitch: scheduled_note_off.1,
                })
                .ok();
            self.scheduled_note_off = None;
        }
    }

    /// Enable the track.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the track.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Sets the enabled state.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns if the track is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Sets the swing in pulses.
    pub fn set_swing(&mut self, swing: i32) {
        self.swing = swing;
    }

    /// Returns the swing in pulses.
    pub fn swing(&mut self) -> i32 {
        self.swing
    }

    /// Sets the track delay in pulses.
    pub fn set_delay(&mut self, delay: i32) {
        self.delay = delay;
    }

    /// Returns the track delay in pulses.
    pub fn delay(&self) -> i32 {
        self.delay
    }

    /// Returns the last play position as step number.
    pub fn play_pos(&self) -> Option<usize> {
        self.play_pos
    }
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
