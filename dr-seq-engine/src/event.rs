//! Events

use crate::params::{Pitch, Velocity};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrackEvent {
    /// Start a note with pitch and velocity.
    NoteOn(Pitch, Velocity),

    /// Stop a note with pitch.
    NoteOff(Pitch),
}

/// Engine event with track number.
pub struct EngineEvent(pub u32, pub TrackEvent);
