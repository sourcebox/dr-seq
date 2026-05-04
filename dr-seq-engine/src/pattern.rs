//! Sequencer pattern module.

use crate::step::Step;

/// Sequencer pattern with length of `NUM_STEPS`.
#[derive(Debug, Clone)]
pub struct Pattern<const NUM_STEPS: usize> {
    /// Array of steps.
    steps: [Step; NUM_STEPS],

    /// Length in steps.
    length_steps: u32,
}

impl<const NUM_STEPS: usize> Default for Pattern<NUM_STEPS> {
    fn default() -> Self {
        Self {
            steps: core::array::from_fn(|_| Step::default()),
            length_steps: 16,
        }
    }
}

impl<const NUM_STEPS: usize> Pattern<NUM_STEPS> {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the length in steps.
    pub fn length_steps(&self) -> u32 {
        self.length_steps
    }

    /// Sets the length in steps.
    pub fn set_length_steps(&mut self, steps: u32) {
        self.length_steps = steps;
    }

    /// Returns a reference to the steps.
    pub fn steps(&mut self) -> &[Step] {
        &self.steps
    }

    /// Returns a mutable reference to the steps.
    pub fn steps_mut(&mut self) -> &mut [Step] {
        &mut self.steps
    }

    /// Returns a reference to a specific step.
    pub fn step(&self, step_no: u32) -> &Step {
        &self.steps[step_no as usize]
    }
}
