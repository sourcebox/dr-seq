//! Sequencer track module.

use crate::pattern::Pattern;

/// Sequencer track.
#[derive(Debug, Default)]
pub struct Track {
    pattern: Pattern,
}

impl Track {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self {
            pattern: Pattern::new(),
        }
    }

    /// Process a clock pulse.
    pub fn clock(&mut self, pulse_no: i32) {}
}
