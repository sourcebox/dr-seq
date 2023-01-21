//! Sequencer pattern module.

use array_init::array_init;

use crate::step::Step;

/// Number of steps inside a bar.
const NUM_STEPS: usize = 16;

/// Sequencer pattern with length of `BARS`.
#[derive(Debug, Clone)]
pub struct Pattern<const BARS: usize> {
    /// Array of bars.
    bars: [Bar; BARS],

    /// Length in steps.
    length_steps: u32,
}

impl<const BARS: usize> Default for Pattern<BARS> {
    fn default() -> Self {
        Self {
            bars: array_init(|_| Bar::default()),
            length_steps: 16,
        }
    }
}

impl<const BARS: usize> Pattern<BARS> {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the length in bars.
    pub fn length_bars(&self) -> u32 {
        self.length_steps / 16
    }

    /// Sets the length in bars.
    pub fn set_length_bars(&mut self, bars: u32) {
        self.length_steps = bars * 16;
    }

    /// Returns the length in steps.
    pub fn length_steps(&self) -> u32 {
        self.length_steps
    }

    /// Sets the length in steps.
    pub fn set_length_steps(&mut self, steps: u32) {
        self.length_steps = steps;
    }

    /// Returns a mutable reference to a specific bar.
    pub fn bar(&mut self, bar_no: u32) -> &mut Bar {
        &mut self.bars[bar_no as usize]
    }
}

#[derive(Debug, Default, Clone)]
pub struct Bar {
    /// Array of steps.
    steps: [Step; NUM_STEPS],
}

impl Bar {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self {
            steps: array_init(|_| Step::default()),
        }
    }

    /// Returns a mutable reference to the steps.
    pub fn steps(&mut self) -> &mut [Step] {
        &mut self.steps
    }

    /// Returns a mutable reference to a specific step.
    pub fn step(&mut self, step_no: u32) -> &mut Step {
        &mut self.steps[step_no as usize]
    }
}
