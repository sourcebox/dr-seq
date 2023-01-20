mod clock;
mod editor;

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use atomic_float::AtomicF32;
use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use serde::{Deserialize, Serialize};

use clock::Clock;
use dr_seq_engine::{Engine, CLOCK_PPQ};

pub struct DrSeq {
    /// Sequencer engine.
    engine: Engine,

    /// Parameters shared with host.
    params: Arc<DrSeqParams>,
}

#[derive(Params)]
struct DrSeqParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[persist = "pattern"]
    pattern: Pattern,

    /// Flag if pattern was changed in the editor.
    pattern_changed: AtomicBool,

    /// Number of the active step.
    active_step: AtomicI32,
}

impl Default for DrSeq {
    fn default() -> Self {
        Self {
            engine: Engine::new(),
            params: Arc::new(DrSeqParams::default()),
        }
    }
}

impl Default for DrSeqParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            pattern: Pattern::default(),
            pattern_changed: AtomicBool::new(false),
            active_step: AtomicI32::new(0),
        }
    }
}

impl Plugin for DrSeq {
    const NAME: &'static str = "Dr. Seq";
    const VENDOR: &'static str = "sourcebox";
    const URL: &'static str = "https://sourcebox.de";
    const EMAIL: &'static str = "info@sourcebox.de";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 0;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 0;

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if self.params.pattern_changed.load(Ordering::Relaxed) {
            self.params.pattern_changed.store(false, Ordering::Relaxed);
        }

        // Pulses per quarter note.
        let ppq = CLOCK_PPQ as f64;

        let clock = Clock::new(buffer, context.transport(), ppq);

        for (pulse_no, timing) in clock {
            self.params
                .active_step
                .store(pulse_no / (ppq / 4.0) as i32 % 16, Ordering::Relaxed);

            if pulse_no % ppq as i32 == 0 {
                let event = NoteEvent::NoteOn {
                    timing,
                    voice_id: None,
                    channel: 0,
                    note: 60,
                    velocity: 1.0,
                };
                context.send_event(event);
            }
            if pulse_no % ppq as i32 == ppq as i32 / 2 {
                let event = NoteEvent::NoteOff {
                    timing,
                    voice_id: None,
                    channel: 0,
                    note: 60,
                    velocity: 0.0,
                };
                context.send_event(event);
            }
        }

        while let Some(event) = context.next_event() {
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for DrSeq {
    const CLAP_ID: &'static str = "de.sourcebox.dr-seq";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("TR-style drum sequencer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for DrSeq {
    const VST3_CLASS_ID: [u8; 16] = *b"sb-dr-seq-plugin";
    const VST3_CATEGORIES: &'static str = "Instrument|Tools";
}

nih_export_clap!(DrSeq);

#[derive(Default, Serialize, Deserialize)]
struct Pattern {
    /// Array of tracks with steps.
    steps: [[AtomicBool; 32]; 8],
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
