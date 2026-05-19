//! Crate containing the core functionality of the plugin.

mod clock;
mod config;
mod editor;
mod params;
mod presets;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

use nice_plug::prelude::*;

use dr_seq_engine::{Pattern, Pitch, StepEvent, Track, TrackEvent, TrackParams, Velocity};

use clock::Clock;
use config::*;
use editor::EditorEvent;
use params::{AppParams, StepState};
use presets::load_preset;

/// Main plugin struct.
pub struct App {
    /// Parameters shared with host.
    params: Arc<AppParams>,

    /// Sender part of channel for events from editor to the engine.
    editor_event_sender: mpsc::SyncSender<EditorEvent>,

    /// Channel for receiving events from the editor.
    editor_event_receiver: mpsc::Receiver<EditorEvent>,

    /// Flag if transport is playing.
    playing: bool,

    /// Individual tracks.
    tracks: [Track; TRACKS],

    /// Patterns for the tracks.
    patterns: [Pattern<16>; TRACKS],

    /// Step repeats for the tracks.
    step_repeats: [bool; TRACKS],
}

impl Default for App {
    fn default() -> Self {
        let update_engine = Arc::new(AtomicBool::new(false));
        let editor_channel: (mpsc::SyncSender<EditorEvent>, mpsc::Receiver<EditorEvent>) =
            mpsc::sync_channel(64);
        Self {
            params: Arc::new(AppParams::new(update_engine.clone())),
            editor_event_sender: editor_channel.0,
            editor_event_receiver: editor_channel.1,
            playing: false,
            tracks: core::array::from_fn(|_| Track::new()),
            patterns: core::array::from_fn(|_| Pattern::<16>::new()),
            step_repeats: [false; TRACKS],
        }
    }
}

impl Plugin for App {
    const NAME: &'static str = NAME;
    const VENDOR: &'static str = VENDOR;
    const URL: &'static str = URL;
    const EMAIL: &'static str = EMAIL;

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    #[cfg(not(feature = "dummy-audio"))]
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    #[cfg(feature = "dummy-audio")]
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();
    type SysExMessage = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.editor_event_sender.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.update_engine();

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if let Ok(event) = self.editor_event_receiver.try_recv() {
            match event {
                EditorEvent::UpdateEngine => {
                    self.update_engine();
                }
                EditorEvent::LoadPreset(preset_no) => {
                    load_preset(preset_no, self.params.clone());
                    self.update_engine();
                }
            }
        }

        let playing = context.transport().playing;

        if playing != self.playing {
            self.playing = playing;
            if !playing {
                // When transport stops, any scheduled note offs should be sent immediately.
                for track in self.tracks.as_mut() {
                    track.flush();
                }
                for (n, track) in self.tracks.as_mut().iter_mut().enumerate() {
                    while let Some(event) = track.next_event() {
                        let note = TRACK_NOTES[n];
                        if let TrackEvent::StepEvent(_, StepEvent::NoteOff { pitch }) = event {
                            let event = NoteEvent::NoteOff {
                                timing: 0,
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
                    }
                }
            }
        }

        let ppq = CLOCK_PPQ as f64;
        let clock = Clock::new(buffer, context.transport(), ppq);

        // Convert the velocity values from 0-127 into 0.0-1.0 range.
        let default_velocity = self.params.normal_velocity.value() as f32 / 127.0;
        let accent_velocity = if self.params.accent_vel_mode.value() {
            self.params.accent_velocity.value() as f32 / 127.0
        } else {
            (default_velocity + self.params.accent_velocity.value() as f32 / 127.0).clamp(0.0, 1.0)
        };
        let weak_velocity = self.params.weak_velocity.value() as f32 / 127.0;
        let ghost_velocity = self.params.ghost_velocity.value() as f32 / 127.0;

        // Iterate over pulses generated by the clock.
        for (pulse_no, timing) in clock {
            // Get the current step from the pulse number and convert it into range 0-15
            // for showing it in the editor.
            let current_step = (pulse_no / (ppq / 4.0) as u32 % 16) as usize;
            if current_step != self.params.current_step.load(Ordering::Relaxed) {
                self.params
                    .current_step
                    .store(current_step, Ordering::Relaxed);
            }

            let mut track_ppq = CLOCK_PPQ;

            // The FAST mangler doubles the speed by halving the ppq.
            if self.params.mangler_fast.value() {
                track_ppq /= 2;
            }

            // The SLOW mangler halves the speed by doubling the ppq.
            if self.params.mangler_slow.value() {
                track_ppq *= 2;
            }

            let mut track_params = TrackParams {
                swing: self.params.swing.value() * CLOCK_PPQ as i32 / 8 / 100,
                shift: if self.params.mangler_swag.value() {
                    1
                } else {
                    0
                },
                reverse: self.params.mangler_mirror.value(),
                triplet: self.params.triplet.value(),
                ..Default::default()
            };

            // The HACK mangler re-sorts the order of the steps.
            if self.params.mangler_hack.value() {
                track_params.resort_fn = Some(|step| {
                    let base = step / 8 * 8;
                    let sub = step % 8;
                    base + match sub {
                        0 => 0,
                        1 => 3,
                        2 => 1,
                        3 => 7,
                        4 => 2,
                        5 => 6,
                        6 => 4,
                        7 => 5,
                        _ => 0,
                    }
                });
            }

            let sole_enabled = self.params.mangler_sole.value();
            let flame_enabled = self.params.mangler_flame.value();
            let mut skip_notes = false;

            for (n, track) in self.tracks.iter_mut().enumerate() {
                track_params.enable = match n {
                    0 => self.params.track1_enable.value(),
                    1 => self.params.track2_enable.value(),
                    2 => self.params.track3_enable.value(),
                    3 => self.params.track4_enable.value(),
                    4 => self.params.track5_enable.value(),
                    5 => self.params.track6_enable.value(),
                    6 => self.params.track7_enable.value(),
                    7 => self.params.track8_enable.value(),
                    _ => false,
                };
                track_params.delay = match n {
                    0 => self.params.track1_delay.value(),
                    1 => self.params.track2_delay.value(),
                    2 => self.params.track3_delay.value(),
                    3 => self.params.track4_delay.value(),
                    4 => self.params.track5_delay.value(),
                    5 => self.params.track6_delay.value(),
                    6 => self.params.track7_delay.value(),
                    7 => self.params.track8_delay.value(),
                    _ => 0,
                };
                track_params.repeat = flame_enabled && self.step_repeats[n];

                if !track_params.enable {
                    // Clear step repeats on disabled tracks.
                    self.step_repeats[n] = false;
                }

                track.update(pulse_no, track_ppq, self.patterns[n].steps(), &track_params);

                while let Some(event) = track.next_event() {
                    // Turn track events into corresponding MIDI messages.
                    let note = TRACK_NOTES[n];
                    match event {
                        TrackEvent::StepEvent(step, StepEvent::NoteOn { pitch, vel }) => {
                            let accent = self.patterns[ACCENT_TRACK as usize].step(step).enabled();
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
                                    Velocity::Accent => accent_velocity,
                                    Velocity::Weak => weak_velocity,
                                    Velocity::Ghost => ghost_velocity,
                                    _ => {
                                        if accent {
                                            accent_velocity
                                        } else {
                                            default_velocity
                                        }
                                    }
                                },
                            };
                            if !skip_notes {
                                context.send_event(event);
                                if sole_enabled {
                                    skip_notes = true;
                                }
                                if flame_enabled {
                                    self.step_repeats[n] = true;
                                }
                            }
                        }
                        TrackEvent::StepEvent(_, StepEvent::NoteOff { pitch }) => {
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

            // Make sure only one track has repeat enabled.
            let repeat_priority = [1, 2, 3, 4, 5, 6, 0, 7];
            let mut disable_repeat = false;
            for n in repeat_priority {
                if disable_repeat {
                    self.step_repeats[n] = false;
                } else if self.step_repeats[n] {
                    disable_repeat = true;
                }
            }
        }

        // Pass thru any incoming MIDI message.
        while let Some(event) = context.next_event() {
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl App {
    /// Update the engine with the parameters from the editor or host.
    fn update_engine(&mut self) {
        for t in 0..TRACKS {
            for (s, step) in self.patterns[t].steps_mut().iter_mut().enumerate() {
                let state =
                    StepState::from(self.params.pattern.steps[t][s].load(Ordering::Relaxed));
                if state != StepState::Off {
                    step.enable();
                    step.set_event(Some(StepEvent::NoteOn {
                        pitch: Pitch::Default,
                        vel: match state {
                            StepState::Accent => Velocity::Accent,
                            StepState::Weak => Velocity::Weak,
                            StepState::Ghost => Velocity::Ghost,
                            _ => Velocity::Default,
                        },
                    }));
                } else {
                    step.disable();
                }
            }
        }
    }
}

impl ClapPlugin for App {
    const CLAP_ID: &'static str = CLAP_ID;
    const CLAP_DESCRIPTION: Option<&'static str> = CLAP_DESCRIPTION;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = CLAP_FEATURES;
}

impl Vst3Plugin for App {
    const VST3_CLASS_ID: [u8; 16] = VST3_CLASS_ID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = VST3_SUBCATEGORIES;
}

nice_export_clap!(App);
