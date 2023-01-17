#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

mod pattern;
mod step;
mod track;

use array_init::array_init;

use crate::track::Track;

/// Number of tracks.
const NUM_TRACKS: usize = 8;

/// Clock pulses per quarter note.
const CLOCK_PPQ: u32 = 384;

/// Sequencer engine.
#[derive(Debug, Default)]
pub struct Engine {
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
    pub fn clock(&mut self, clock_no: u32) {
        for track in self.tracks.as_mut() {
            track.clock(clock_no);
        }
    }
}
