//! Configuration settings.

/// Application name.
pub const NAME: &'static str = "Dr. Seq";

/// Application vendor.
pub const VENDOR: &'static str = "sourcebox";

/// Homepage URL.
pub const URL: &'static str = "https://sourcebox.de";

/// Email address.
pub const EMAIL: &'static str = "info@sourcebox.de";

/// Total number of tracks. Last track is used for global accent.
pub const TRACKS: usize = 9;

/// Number of the accent track.
pub const ACCENT_TRACK: u32 = (TRACKS - 1) as u32;

/// Number of bars per track.
pub const BARS: usize = 1;

/// Clock pulses per quarter note.
pub const CLOCK_PPQ: u32 = 384;

/// CLAP plugin id.
pub const CLAP_ID: &'static str = "de.sourcebox.dr-seq";

/// CLAP plugin description.
pub const CLAP_DESCRIPTION: Option<&'static str> = Some("Grid-based drum sequencer");

/// VST3 plugin class id.
pub const VST3_CLASS_ID: [u8; 16] = *b"sb-dr-seq-plugin";

/// VST3 plugin categories.
pub const VST3_CATEGORIES: &'static str = "Instrument|Tools";
