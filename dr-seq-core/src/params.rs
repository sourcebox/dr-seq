//! Plugin parameters.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use serde::{Deserialize, Serialize};

use crate::editor;

#[derive(Params)]
pub struct AppParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[persist = "pattern"]
    pub pattern: Pattern,

    /// Flag if pattern was changed in the editor.
    pub pattern_changed: AtomicBool,

    /// Number of the active step.
    pub active_step: AtomicI32,
}

impl Default for AppParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            pattern: Pattern::default(),
            pattern_changed: AtomicBool::new(false),
            active_step: AtomicI32::new(0),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Pattern {
    /// Array of tracks with steps.
    pub steps: [[AtomicBool; 16]; 8],
}

impl<'a> PersistentField<'a, Pattern> for Pattern {
    fn set(&self, new_value: Pattern) {
        for (step, new_step) in self.steps.iter().zip(new_value.steps) {
            for s in step.iter().zip(new_step) {
                s.0.store(s.1.load(Ordering::Relaxed), Ordering::Relaxed)
            }
        }
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&Pattern) -> R,
    {
        f(self)
    }
}
