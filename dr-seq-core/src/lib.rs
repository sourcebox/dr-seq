mod clock;
mod config;
mod editor;
mod params;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nih_plug::prelude::*;

use clock::Clock;
use config::*;
use dr_seq_engine::{
    event::TrackEvent,
    params::{Pitch, Velocity},
    Engine,
};
use params::AppParams;

pub struct App {
    /// Parameters shared with host.
    params: Arc<AppParams>,

    /// Sequencer engine.
    engine: Engine<TRACKS, BARS, CLOCK_PPQ>,

    /// Flag to update the engine after a parameter has been changed.
    update_engine: Arc<AtomicBool>,
}

impl Default for App {
    fn default() -> Self {
        let update_engine = Arc::new(AtomicBool::new(false));
        Self {
            params: Arc::new(AppParams::new(update_engine.clone())),
            engine: Engine::new(),
            update_engine,
        }
    }
}

impl Plugin for App {
    const NAME: &'static str = NAME;
    const VENDOR: &'static str = VENDOR;
    const URL: &'static str = URL;
    const EMAIL: &'static str = EMAIL;

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
        self.update_engine();

        // The accent track is always disabled for playing and only used to store the steps.
        self.engine.track(ACCENT_TRACK).disable();

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

        if self.update_engine.load(Ordering::Relaxed) {
            self.update_engine();
            self.params.pattern_changed.store(false, Ordering::Relaxed);
        }

        // Pulses per quarter note.
        let ppq = CLOCK_PPQ as f64;

        let clock = Clock::new(buffer, context.transport(), ppq);

        let accent_velocity = self.params.accent_velocity.value() as f32 / 127.0;

        for (pulse_no, timing) in clock {
            self.params
                .active_step
                .store(pulse_no / (ppq / 4.0) as i32 % 16, Ordering::Relaxed);

            self.engine.clock(pulse_no);

            while let Some(event) = self.engine.next_event() {
                let note = (36 + event.0) as u8;
                match event.1 {
                    TrackEvent::NoteOn {
                        bar,
                        step,
                        pitch,
                        vel,
                    } => {
                        let accent = self
                            .engine
                            .track(ACCENT_TRACK)
                            .pattern()
                            .bar(bar)
                            .step(step)
                            .enabled();
                        let event = NoteEvent::NoteOn {
                            timing,
                            voice_id: None,
                            channel: 0,
                            note: match pitch {
                                Pitch::Default => note,
                                Pitch::Custom(pitch) => pitch as u8,
                                _ => note,
                            },
                            velocity: match vel {
                                Velocity::Strong => 1.0,
                                Velocity::Weak => 50.0 / 127.0,
                                _ => {
                                    if accent {
                                        accent_velocity
                                    } else {
                                        100.0 / 127.0
                                    }
                                }
                            },
                        };
                        context.send_event(event);
                    }
                    TrackEvent::NoteOff {
                        bar: _,
                        step: _,
                        pitch,
                    } => {
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
        for (t, track) in self.engine.tracks()[0..TRACKS].iter_mut().enumerate() {
            for (s, step) in track.pattern().bar(0).steps().iter_mut().enumerate() {
                let state = self.params.pattern.steps[t][s].load(Ordering::Relaxed);
                if state {
                    step.enable();
                } else {
                    step.disable();
                }
            }
        }

        self.engine.set_swing(self.params.swing.value() * 48 / 100);

        self.engine
            .track(0)
            .set_enabled(self.params.track1_enable.value());
        self.engine
            .track(1)
            .set_enabled(self.params.track2_enable.value());
        self.engine
            .track(2)
            .set_enabled(self.params.track3_enable.value());
        self.engine
            .track(3)
            .set_enabled(self.params.track4_enable.value());
        self.engine
            .track(4)
            .set_enabled(self.params.track5_enable.value());
        self.engine
            .track(5)
            .set_enabled(self.params.track6_enable.value());
        self.engine
            .track(6)
            .set_enabled(self.params.track7_enable.value());
        self.engine
            .track(7)
            .set_enabled(self.params.track8_enable.value());

        self.engine
            .track(0)
            .set_delay(self.params.track1_delay.value());
        self.engine
            .track(1)
            .set_delay(self.params.track2_delay.value());
        self.engine
            .track(2)
            .set_delay(self.params.track3_delay.value());
        self.engine
            .track(3)
            .set_delay(self.params.track4_delay.value());
        self.engine
            .track(4)
            .set_delay(self.params.track5_delay.value());
        self.engine
            .track(5)
            .set_delay(self.params.track6_delay.value());
        self.engine
            .track(6)
            .set_delay(self.params.track7_delay.value());
        self.engine
            .track(7)
            .set_delay(self.params.track8_delay.value());
    }
}

impl ClapPlugin for App {
    const CLAP_ID: &'static str = CLAP_ID;
    const CLAP_DESCRIPTION: Option<&'static str> = CLAP_DESCRIPTION;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for App {
    const VST3_CLASS_ID: [u8; 16] = VST3_CLASS_ID;
    const VST3_CATEGORIES: &'static str = VST3_CATEGORIES;
}

nih_export_clap!(App);
