//! Events

use crate::params::{Pitch, Velocity};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrackEvent {
    /// Start a note with pitch and velocity.
    NoteOn {
        bar: u32,
        step: u32,
        pitch: Pitch,
        vel: Velocity,
    },

    /// Stop a note with pitch.
    NoteOff { bar: u32, step: u32, pitch: Pitch },
}

/// Engine event with track number.
pub struct EngineEvent(pub u32, pub TrackEvent);
