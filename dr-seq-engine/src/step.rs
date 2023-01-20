//! Sequencer step module.

use crate::params::{Pitch, Velocity};

/// Sequencer step.
#[derive(Debug, Default)]
pub struct Step {
    pitch: Pitch,
    velocity: Velocity,
}

impl Step {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }
}
