//! Plugin parameters.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use serde::{Deserialize, Serialize};

use crate::config::{CLOCK_PPQ, TRACKS};
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

    /// Swing.
    #[id = "swing"]
    pub swing: IntParam,

    /// Track 1 delay.
    #[id = "track1-delay"]
    pub track1_delay: IntParam,

    /// Track 2 delay.
    #[id = "track2-delay"]
    pub track2_delay: IntParam,

    /// Track 3 delay.
    #[id = "track3-delay"]
    pub track3_delay: IntParam,

    /// Track 4 delay.
    #[id = "track4-delay"]
    pub track4_delay: IntParam,

    /// Track 5 delay.
    #[id = "track5-delay"]
    pub track5_delay: IntParam,

    /// Track 6 delay.
    #[id = "track6-delay"]
    pub track6_delay: IntParam,

    /// Track 7 delay.
    #[id = "track7-delay"]
    pub track7_delay: IntParam,

    /// Track 8 delay.
    #[id = "track8-delay"]
    pub track8_delay: IntParam,
}

impl AppParams {
    /// Returns a new instance.
    pub fn new(update_engine: Arc<AtomicBool>) -> Self {
        let delay_range = IntRange::Linear {
            min: -(CLOCK_PPQ as i32) / 8,
            max: (CLOCK_PPQ as i32) / 8,
        };

        Self {
            editor_state: editor::default_state(),
            pattern: Pattern::default(),
            pattern_changed: AtomicBool::new(false),
            active_step: AtomicI32::new(0),
            swing: IntParam::new("Swing", 0, IntRange::Linear { min: 0, max: 100 }).with_callback(
                {
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                },
            ),
            track1_delay: IntParam::new("Track 1 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track2_delay: IntParam::new("Track 2 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track3_delay: IntParam::new("Track 3 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track4_delay: IntParam::new("Track 4 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track5_delay: IntParam::new("Track 5 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track6_delay: IntParam::new("Track 6 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track7_delay: IntParam::new("Track 7 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
            track8_delay: IntParam::new("Track 8 Delay", 0, delay_range).with_callback({
                let update_engine = update_engine.clone();
                Arc::new(move |_| update_engine.store(true, Ordering::Release))
            }),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Pattern {
    /// Array of tracks with steps.
    pub steps: [[AtomicBool; 16]; TRACKS],
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
