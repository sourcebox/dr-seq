//! Configuration settings.

use nih_plug::wrapper::clap::features::ClapFeature;
use nih_plug::wrapper::vst3::subcategories::Vst3SubCategory;

/// Application name.
pub const NAME: &str = "Dr. Seq";

/// Application vendor.
pub const VENDOR: &str = "sourcebox";

/// Homepage URL.
pub const URL: &str = "https://sourcebox.de";

/// Email address.
pub const EMAIL: &str = "info@sourcebox.de";

/// Total number of tracks. Last track is used for global accent.
pub const TRACKS: usize = 9;

/// Labels for the tracks.
pub const TRACK_LABELS: [&str; TRACKS] =
    ["BD", "SD", "CHH", "PHH", "OHH", "WD", "MT", "SFX", "ACC"];

/// Notes send by the tracks.
pub const TRACK_NOTES: [u8; TRACKS - 1] = [36, 38, 42, 44, 46, 48, 51, 49];

/// Number of the accent track.
pub const ACCENT_TRACK: u32 = (TRACKS - 1) as u32;

/// Clock pulses per quarter note.
pub const CLOCK_PPQ: u32 = 384;

/// CLAP plugin id.
pub const CLAP_ID: &str = "de.sourcebox.dr-seq";

/// CLAP plugin description.
pub const CLAP_DESCRIPTION: Option<&str> = Some("Grid-based drum sequencer");

/// CLAP features list.
pub const CLAP_FEATURES: &[ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];

/// VST3 plugin class id.
pub const VST3_CLASS_ID: [u8; 16] = *b"sb-dr-seq-plugin";

/// VST3 plugin sub categories.
pub const VST3_SUBCATEGORIES: &[Vst3SubCategory] =
    &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
