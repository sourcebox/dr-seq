#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub mod event;
pub mod params;
pub mod pattern;
pub mod step;
pub mod track;

use array_init::array_init;

use crate::event::EngineEvent;
use crate::track::Track;

/// Sequencer engine.
/// - `TRACKS` is the total number of tracks.
/// - `BARS` is the max number of bars in each pattern.
/// - `PPQ` is the resolution in pulses per quarter note.
#[derive(Debug)]
pub struct Engine<const TRACKS: usize, const BARS: usize, const PPQ: u32> {
    /// Individual tracks.
    tracks: [Track<BARS, PPQ>; TRACKS],
}

impl<const TRACKS: usize, const BARS: usize, const PPQ: u32> Default for Engine<TRACKS, BARS, PPQ> {
    fn default() -> Self {
        Self {
            tracks: array_init(|_| Track::default()),
        }
    }
}

impl<const TRACKS: usize, const BARS: usize, const PPQ: u32> Engine<TRACKS, BARS, PPQ> {
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
    pub fn next_event(&mut self) -> Option<EngineEvent> {
        for (n, track) in self.tracks.iter_mut().enumerate() {
            if let Some(event) = track.next_event() {
                return Some(EngineEvent(n as u32, event));
            }
        }

        None
    }

    /// Returns a mutable reference to the tracks.
    pub fn tracks(&mut self) -> &mut [Track<BARS, PPQ>] {
        &mut self.tracks
    }

    /// Returns a mutable reference to a specific track.
    pub fn track(&mut self, track_no: u32) -> &mut Track<BARS, PPQ> {
        &mut self.tracks[track_no as usize]
    }
}
