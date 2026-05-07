//! Sequencer track.

use heapless::spsc::Queue;

use crate::params::Pitch;
use crate::step::{Step, StepEvent};

/// Capacity of the event queue.
pub const EVENT_QUEUE_CAPACITY: usize = 16;

/// Event queue.
type EventQueue = Queue<TrackEvent, EVENT_QUEUE_CAPACITY>;

/// Sequencer track.
#[derive(Default, Debug)]
pub struct Track {
    /// Last played step number.
    play_step: Option<u32>,

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
    pub fn update(&mut self, pulse_no: u32, ppq: u32, steps: &[Step], params: &TrackParams) {
        let mut pulse_no = pulse_no as i32 - params.delay;

        // Apply swing value to each 2nd step.
        if pulse_no / (ppq as i32 / 4) % 2 == 1 {
            pulse_no -= params.swing;
        }

        // Make sure pulse no is always positive.
        let pulse_no = pulse_no.max(0) as u32;

        let mut shift = params.shift;
        while shift < 0 {
            shift += steps.len() as i32;
        }

        // Do some calculations to determine where we are.
        let mut play_step = (pulse_no / (ppq / 4) + shift as u32) % steps.len() as u32;

        // Apply reverse option.
        if params.reverse {
            play_step = (steps.len() - 1) as u32 - play_step;
        }

        // Apply re-sort function.
        if let Some(f) = params.resort_fn {
            play_step = f(play_step);
        }

        // Check if a previously started note has reached its length.
        if let Some(scheduled_note_off) = self.scheduled_note_off
            && self.pulse_count == scheduled_note_off.0
        {
            let step_event = StepEvent::NoteOff {
                pitch: scheduled_note_off.1,
            };
            self.event_queue
                .enqueue(TrackEvent::StepEvent(play_step, step_event))
                .ok();
            self.scheduled_note_off = None;
        }

        if params.enable && (self.play_step.is_none() || play_step != self.play_step.unwrap()) {
            let mut step = &steps[play_step as usize];

            if params.repeat
                && !step.enabled()
                && let Some(last_step) = self.play_step()
            {
                step = &steps[last_step as usize];
            }

            self.play_step = Some(play_step);

            // Get the event and emit it.
            if step.enabled()
                && let Some(step_event) = step.event().clone()
            {
                // If a note is still playing, it must be stopped before triggering a new one.
                if let Some(scheduled_note_off) = self.scheduled_note_off {
                    let step_event = StepEvent::NoteOff {
                        pitch: scheduled_note_off.1,
                    };
                    self.event_queue
                        .enqueue(TrackEvent::StepEvent(play_step, step_event))
                        .ok();
                    self.scheduled_note_off = None;
                }

                // Enqueue the event.
                self.event_queue
                    .enqueue(TrackEvent::StepEvent(play_step, step_event.clone()))
                    .ok();

                // If the event is a note on, then a corresponding note off
                // is scheduled for later processing.
                if let StepEvent::NoteOn { pitch, .. } = step_event {
                    // Schedule the note off for a 1/32 note length.
                    let note_off_pulse = self.pulse_count + ppq / 8;
                    self.scheduled_note_off = Some((note_off_pulse, pitch));
                }
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
            let step_event = StepEvent::NoteOff {
                pitch: scheduled_note_off.1,
            };
            self.event_queue
                .enqueue(TrackEvent::StepEvent(play_step, step_event))
                .ok();
            self.scheduled_note_off = None;
        }
    }

    /// Returns the last played step number.
    pub fn play_step(&self) -> Option<u32> {
        self.play_step
    }
}

/// Track playback parameters.
#[derive(Debug, Default, Clone)]
pub struct TrackParams {
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

    /// Repeat the last step once.
    pub repeat: bool,

    /// Function to re-sort the steps.
    pub resort_fn: Option<fn(u32) -> u32>,
}

/// Events generated by track playback.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrackEvent {
    /// Event for a step.
    StepEvent(u32, StepEvent),
}
