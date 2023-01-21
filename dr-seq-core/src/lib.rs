mod clock;
mod editor;
mod params;

use std::sync::atomic::Ordering;
use std::sync::Arc;

use nih_plug::prelude::*;

use clock::Clock;
use dr_seq_engine::{
    event::TrackEvent,
    params::{Pitch, Velocity},
    Engine,
};
use params::AppParams;

/// Number of tracks.
const TRACKS: usize = 8;

/// Number of bars per track.
const BARS: usize = 1;

/// Clock pulses per quarter note.
const CLOCK_PPQ: u32 = 384;

pub struct App {
    /// Sequencer engine.
    engine: Engine<TRACKS, BARS, CLOCK_PPQ>,

    /// Parameters shared with host.
    params: Arc<AppParams>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            engine: Engine::new(),
            params: Arc::new(AppParams::default()),
        }
    }
}

impl Plugin for App {
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
        for track in self.engine.tracks() {
            track.enable();
        }

        self.update_engine();

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if self.params.pattern_changed.load(Ordering::Relaxed) {
            self.update_engine();
            self.params.pattern_changed.store(false, Ordering::Relaxed);
        }

        // Pulses per quarter note.
        let ppq = CLOCK_PPQ as f64;

        let clock = Clock::new(buffer, context.transport(), ppq);

        for (pulse_no, timing) in clock {
            self.params
                .active_step
                .store(pulse_no / (ppq / 4.0) as i32 % 16, Ordering::Relaxed);

            self.engine.clock(pulse_no);

            while let Some(event) = self.engine.next_event() {
                let note = (36 + event.0) as u8;
                match event.1 {
                    TrackEvent::NoteOn(pitch, velocity) => {
                        let event = NoteEvent::NoteOn {
                            timing,
                            voice_id: None,
                            channel: 0,
                            note: match pitch {
                                Pitch::Default => note,
                                Pitch::Custom(pitch) => pitch as u8,
                                _ => note,
                            },
                            velocity: match velocity {
                                Velocity::Strong => 1.0,
                                Velocity::Weak => 0.4,
                                _ => 0.7,
                            },
                        };
                        context.send_event(event);
                    }
                    TrackEvent::NoteOff(pitch) => {
                        let event = NoteEvent::NoteOff {
                            timing,
                            voice_id: None,
                            channel: 0,
                            note: match pitch {
                                Pitch::Default => note,
                                Pitch::Custom(pitch) => pitch as u8,
                                _ => note,
                            },
                            velocity: 0.0,
                        };
                        context.send_event(event)
                    }
                    _ => {}
                }
            }
        }

        while let Some(event) = context.next_event() {
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl App {
    fn update_engine(&mut self) {
        for (t, track) in self.engine.tracks().iter_mut().enumerate() {
            for (s, step) in track.pattern().bar(0).steps().iter_mut().enumerate() {
                let state = self.params.pattern.steps[t][s].load(Ordering::Relaxed);
                if state {
                    step.enable();
                } else {
                    step.disable();
                }
            }
        }
    }
}

impl ClapPlugin for App {
    const CLAP_ID: &'static str = "de.sourcebox.dr-seq";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("TR-style drum sequencer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for App {
    const VST3_CLASS_ID: [u8; 16] = *b"sb-dr-seq-plugin";
    const VST3_CATEGORIES: &'static str = "Instrument|Tools";
}

nih_export_clap!(App);
