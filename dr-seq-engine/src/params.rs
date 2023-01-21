//! Parameter types.

/// Pitch options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Pitch {
    /// Default setting.
    #[default]
    Default,

    /// Individual setting.
    Custom(i32),
}

/// Velocity options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Velocity {
    /// Normal velocity.
    #[default]
    Default,

    /// Accented step.
    Strong,

    /// Quiet step.
    Weak,

    /// Individual setting.
    Custom(i32),
}
