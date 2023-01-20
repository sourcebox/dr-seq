//! Parameter types.

/// Pitch options.
#[derive(Debug, Default)]
pub enum Pitch {
    /// Default setting.
    #[default]
    Default,

    /// Individual setting.
    Custom(u8),
}

/// Velocity options.
#[derive(Debug, Default)]
pub enum Velocity {
    /// Normal velocity.
    #[default]
    Normal,

    /// Accented step.
    Strong,

    /// Quiet step.
    Weak,

    /// Individual setting.
    Custom(u8),
}
