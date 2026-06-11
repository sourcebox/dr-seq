//! Preset patterns.
//!
//! Regular tracks:
//!     _: off
//!     X: normal
//!     x: weak
//!     .: ghost
//!
//! Hihat:
//!     - x / X: normal
//!     - o / O: open
//!     - h / H: pedal

use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::{
    config::TRACKS,
    params::{AppParams, StepState},
};

/// Definition of a preset pattern.
pub struct PresetPattern {
    /// Steps
    pub steps: [&'static str; 6],
}

pub static PRESET_PATTERN_0: PresetPattern = PresetPattern {
    steps: [
        "X___X___X___X___",
        "____X_______X___",
        "__X___X___X___X_",
        "________________",
        "________________",
        "________________",
    ],
};

pub static PRESET_PATTERN_1: PresetPattern = PresetPattern {
    steps: [
        "X___X___X___X___",
        "____X_______X___",
        "__O___O___O___O_",
        "________________",
        "________________",
        "________________",
    ],
};

pub static PRESET_PATTERN_2: PresetPattern = PresetPattern {
    steps: [
        "X___X___X___X___",
        "____X_______X___",
        "__X__HX___XH_HX_",
        "________________",
        "________________",
        "________________",
    ],
};

pub static PRESET_PATTERN_3: PresetPattern = PresetPattern {
    steps: [
        "X___X___X___X___",
        "____X_______X___",
        "XXXXXXXXXXXXXXXX",
        "________________",
        "________________",
        "________________",
    ],
};

pub static PRESET_PATTERN_4: PresetPattern = PresetPattern {
    steps: [
        "X___X___X___X___",
        "____X_______X___",
        "__X___XH__X___X_",
        "_XX___________X_",
        "________________",
        "________________",
    ],
};

pub static PRESET_PATTERN_5: PresetPattern = PresetPattern {
    steps: [
        "X__X__X____X_XX_",
        "____X__X_X__X___",
        "XXO_X_XHX_H_XH__",
        "__X__X_X___X____",
        "________________",
        "________________",
    ],
};

/// DnB main.
pub static PRESET_PATTERN_6: PresetPattern = PresetPattern {
    steps: [
        "X_________X_____",
        "____X__x_x__X___",
        "X_X_X_X_X_X_X_X_",
        "________________",
        "________________",
        "________________",
    ],
};

/// DnB variation.
pub static PRESET_PATTERN_7: PresetPattern = PresetPattern {
    steps: [
        "X_____X_________",
        "____X_____X_____",
        "X_X_X_X_X_X_X_X_",
        "________________",
        "________________",
        "________________",
    ],
};

/// Loads a preset into the parameters.
pub fn load_preset(preset_no: u32, params: Arc<AppParams>) {
    let preset = match preset_no {
        1 => &PRESET_PATTERN_1,
        2 => &PRESET_PATTERN_2,
        3 => &PRESET_PATTERN_3,
        4 => &PRESET_PATTERN_4,
        5 => &PRESET_PATTERN_5,
        6 => &PRESET_PATTERN_6,
        7 => &PRESET_PATTERN_7,
        _ => &PRESET_PATTERN_0,
    };

    // Clear pattern.
    for t in 0..TRACKS {
        for s in 0..16 {
            params.pattern.steps[t][s].store(0, Ordering::Relaxed);
        }
    }

    for (t, steps) in preset.steps.iter().enumerate() {
        for (s, step) in steps.chars().enumerate() {
            match t {
                2 => {
                    // Hihat
                    let (state, offset) = match step {
                        'X' => (StepState::Normal, 0),
                        'x' => (StepState::Weak, 0),
                        'O' => (StepState::Normal, 2),
                        'o' => (StepState::Weak, 2),
                        'H' => (StepState::Normal, 1),
                        'h' => (StepState::Weak, 1),
                        _ => (StepState::Off, 0),
                    };
                    params.pattern.steps[t + offset][s].store(state.into(), Ordering::Relaxed);
                }
                0..2 => {
                    // Regular track 1 or 2
                    let state = match step {
                        'X' => StepState::Normal,
                        'x' => StepState::Weak,
                        '.' => StepState::Ghost,
                        _ => StepState::Off,
                    };
                    params.pattern.steps[t][s].store(state.into(), Ordering::Relaxed);
                }
                _ => {
                    // Regular track above hihat
                    let state = match step {
                        'X' => StepState::Normal,
                        'x' => StepState::Weak,
                        '.' => StepState::Ghost,
                        _ => StepState::Off,
                    };
                    params.pattern.steps[t + 2][s].store(state.into(), Ordering::Relaxed);
                }
            }
        }
    }
}
