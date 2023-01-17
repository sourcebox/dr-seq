//! Sequencer pattern module.

use array_init::array_init;

use crate::step::Step;

/// Number of steps.
const NUM_STEPS: usize = 16;

/// Sequencer pattern.
#[derive(Debug, Default)]
pub struct Pattern {
    /// Steps inside the pattern.
    steps: [Step; NUM_STEPS],
}

impl Pattern {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self {
            steps: array_init(|_| Step::new()),
        }
    }
}
