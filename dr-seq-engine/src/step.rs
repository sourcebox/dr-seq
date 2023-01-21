//! Sequencer step module.

use crate::params::{Pitch, Velocity};

/// Sequencer step.
#[derive(Debug, Default, Clone)]
pub struct Step {
    /// Flag if step is enabled for playing.
    enabled: bool,

    /// Pitch of the step.
    pitch: Pitch,

    /// Velocity of the step.
    velocity: Velocity,
}

impl Step {
    /// Returns a new instance.
    pub fn new(pitch: Pitch, velocity: Velocity) -> Self {
        Self {
            pitch,
            velocity,
            ..Default::default()
        }
    }

    /// Enable the step.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the step.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns if the step is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
