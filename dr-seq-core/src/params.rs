//! Plugin parameters.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use serde::{Deserialize, Serialize};

use crate::config::{BARS, CLOCK_PPQ, TRACKS};
use crate::editor::{self, EditorEvent};

#[derive(Params)]
pub struct AppParams {
    /// State of the editor.
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    /// State of the pattern in the grid.
    #[persist = "pattern"]
    pub pattern: Pattern,

    /// Sender part of channel for events from editor to the engine.
    pub editor_event_sender: Mutex<mpsc::Sender<EditorEvent>>,

    /// Number of the current step.
    pub current_step: AtomicI32,

    /// Swing.
    #[id = "swing"]
    pub swing: IntParam,

    /// Track 1 enable.
    #[id = "track1-enable"]
    pub track1_enable: BoolParam,

    /// Track 2 enable.
    #[id = "track2-enable"]
    pub track2_enable: BoolParam,

    /// Track 3 enable.
    #[id = "track3-enable"]
    pub track3_enable: BoolParam,

    /// Track 4 enable.
    #[id = "track4-enable"]
    pub track4_enable: BoolParam,

    /// Track 5 enable.
    #[id = "track5-enable"]
    pub track5_enable: BoolParam,

    /// Track 6 enable.
    #[id = "track6-enable"]
    pub track6_enable: BoolParam,

    /// Track 7 enable.
    #[id = "track7-enable"]
    pub track7_enable: BoolParam,

    /// Track 8 enable.
    #[id = "track8-enable"]
    pub track8_enable: BoolParam,

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

    /// Accent velocity.
    #[id = "accent-velocity"]
    pub accent_velocity: IntParam,
}

impl AppParams {
    /// Returns a new instance.
    pub fn new(
        update_engine: Arc<AtomicBool>,
        editor_event_sender: mpsc::Sender<EditorEvent>,
    ) -> Self {
        let delay_range = IntRange::Linear {
            min: -(CLOCK_PPQ as i32) / 8,
            max: (CLOCK_PPQ as i32) / 8,
        };

        Self {
            editor_state: editor::default_state(),
            pattern: Pattern::default(),
            editor_event_sender: Mutex::new(editor_event_sender),
            current_step: AtomicI32::new(0),
            swing: IntParam::new("Swing", 0, IntRange::Linear { min: 0, max: 100 }).with_callback(
                {
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                },
            ),

            // Track enables
            track1_enable: BoolParam::new("Track 1 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track2_enable: BoolParam::new("Track 2 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track3_enable: BoolParam::new("Track 3 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track4_enable: BoolParam::new("Track 4 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track5_enable: BoolParam::new("Track 5 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track6_enable: BoolParam::new("Track 6 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track7_enable: BoolParam::new("Track 7 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),
            track8_enable: BoolParam::new("Track 8 Enable", true)
                .with_callback({
                    let update_engine = update_engine.clone();
                    Arc::new(move |_| update_engine.store(true, Ordering::Release))
                })
                .with_value_to_string(Arc::new(|value| {
                    String::from(if value { "on" } else { "off" })
                })),

            // Track delays
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

            accent_velocity: IntParam::new(
                "Accent Velocity",
                127,
                IntRange::Linear { min: 0, max: 127 },
            ),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Pattern {
    /// Array of tracks with steps.
    pub steps: [[[AtomicBool; 16]; BARS]; TRACKS],
}

impl<'a> PersistentField<'a, Pattern> for Pattern {
    fn set(&self, new_value: Pattern) {
        for (track, new_track) in self.steps.iter().zip(new_value.steps) {
            for (step, new_step) in track.iter().zip(new_track) {
                for s in step.iter().zip(new_step) {
                    s.0.store(s.1.load(Ordering::Relaxed), Ordering::Relaxed)
                }
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
