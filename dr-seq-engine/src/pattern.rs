//! Sequencer pattern.

use crate::step::Step;

/// Sequencer pattern with capacity of `CAPACITY`.
#[derive(Debug, Clone)]
pub struct Pattern<const CAPACITY: usize> {
    /// Array of steps.
    steps: [Step; CAPACITY],

    /// Active length in steps.
    length: usize,
}

impl<const NUM_STEPS: usize> Default for Pattern<NUM_STEPS> {
    fn default() -> Self {
        Self {
            steps: core::array::from_fn(|_| Step::default()),
            length: 16,
        }
    }
}

impl<const NUM_STEPS: usize> Pattern<NUM_STEPS> {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the active length in steps.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Sets the active length in steps.
    pub fn set_len(&mut self, steps: usize) {
        self.length = steps;
    }

    /// Returns the capacity in steps.
    pub fn capacity(&self) -> usize {
        NUM_STEPS
    }

    /// Returns a reference to the steps.
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    /// Returns a mutable reference to the steps.
    pub fn steps_mut(&mut self) -> &mut [Step] {
        &mut self.steps
    }

    /// Returns a reference to a specific step.
    pub fn step(&self, step_no: usize) -> &Step {
        &self.steps[step_no]
    }
}
