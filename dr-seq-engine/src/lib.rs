#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub mod event;
pub mod params;

mod pattern;
mod step;
mod track;

use array_init::array_init;

use crate::event::Event;
use crate::track::Track;

/// Number of tracks.
const NUM_TRACKS: usize = 8;

/// Clock pulses per quarter note.
pub const CLOCK_PPQ: u32 = 384;

/// Sequencer engine.
#[derive(Default)]
pub struct Engine {
    /// Individual tracks.
    tracks: [Track; NUM_TRACKS],
}

impl Engine {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self {
            tracks: array_init(|_| Track::new()),
        }
    }

    /// Process a clock pulse.
    pub fn clock(&mut self, pulse_no: i32) {
        for track in self.tracks.as_mut() {
            track.clock(pulse_no);
        }
    }

    /// Return next event.
    pub fn next_event(&mut self) -> Option<Event> {
        for track in self.tracks.as_mut() {
            if let Some(event) = track.next_event() {
                return Some(event);
            }
        }

        None
    }

    /// Return a mutable reference to the tracks.
    pub fn tracks(&mut self) -> &mut [Track] {
        &mut self.tracks
    }

    /// Return a mutable reference to a specific track.
    pub fn track(&mut self, track_no: usize) -> &mut Track {
        &mut self.tracks[track_no]
    }
}
