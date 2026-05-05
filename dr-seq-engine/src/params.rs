//! Parameters.

use serde::{Deserialize, Serialize};

/// Pitch variants.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Pitch {
    /// Default setting.
    #[default]
    Default,

    /// Individual setting.
    Custom(i32),
}

/// Velocity variants.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Velocity {
    /// Normal velocity.
    #[default]
    Default,

    /// Accented velocity.
    Accent,

    /// Quiet velocity.
    Weak,

    /// Ghost velocity.
    Ghost,

    /// Individual setting.
    Custom(u8),
}
