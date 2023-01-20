//! Events

use crate::params::{Pitch, Velocity};

pub enum Event {
    /// Start a note with pitch and velocity.
    NoteOn(Pitch, Velocity),

    /// Stop a note with pitch.
    NoteOff(Pitch),
}
