//!

use crate::{c_api::ffi, NcVisualOptions};

/// Treats as transparent the color specified in the `transcolor` field.
pub const NCVISUAL_OPTION_ADDALPHA: u32 = ffi::NCVISUAL_OPTION_ADDALPHA;

/// Uses [`NcAlpha::Blend`][crate::NcAlpha#associatedconstant.BLEND] with
/// the `NcVisual`.
pub const NCVISUAL_OPTION_BLEND: u32 = ffi::NCVISUAL_OPTION_BLEND;

/// allows you to indicate that the n field of ncvisual_options refers not to
/// the plane onto which you'd like to blit, but the parent of a new plane.
///
/// A plane will be created using the other parameters in the ncvisual_options,
/// as a child of this parent. This means things like, say, vertically centering
/// a sprixel relative to the standard plane can be done in one step.
pub const NCVISUAL_OPTION_CHILDPLANE: u32 = ffi::NCVISUAL_OPTION_CHILDPLANE;

/// Fails rather than gracefully degrade. See [`NcBlitter`][crate::NcBlitter].
pub const NCVISUAL_OPTION_NODEGRADE: u32 = ffi::NCVISUAL_OPTION_NODEGRADE;

/// Y is an alignment, not absolute.
pub const NCVISUAL_OPTION_VERALIGNED: u32 = ffi::NCVISUAL_OPTION_VERALIGNED;

/// X is an alignment, not absolute.
pub const NCVISUAL_OPTION_HORALIGNED: u32 = ffi::NCVISUAL_OPTION_HORALIGNED;

/// Uses non-interpolative scaling.
pub const NCVISUAL_OPTION_NOINTERPOLATE: u32 = ffi::NCVISUAL_OPTION_NOINTERPOLATE;

impl NcVisualOptions {
    /// Treats as transparent the color specified in the `transcolor` field.
    pub const ADDALPHA: u32 = NCVISUAL_OPTION_ADDALPHA;

    /// Uses [`NcAlpha::Blend`][crate::NcAlpha#associatedconstant.BLEND] with
    /// the `NcVisual`.
    pub const BLEND: u32 = NCVISUAL_OPTION_BLEND;

    /// allows you to indicate that the n field of ncvisual_options refers not to
    /// the plane onto which you'd like to blit, but the parent of a new plane.
    ///
    /// A plane will be created using the other parameters in the ncvisual_options,
    /// as a child of this parent. This means things like, say, vertically centering
    /// a sprixel relative to the standard plane can be done in one step.
    pub const CHILDPLANE: u32 = NCVISUAL_OPTION_CHILDPLANE;

    /// Fails rather than gracefully degrade. See [`NcBlitter`][crate::NcBlitter].
    pub const NODEGRADE: u32 = NCVISUAL_OPTION_NODEGRADE;

    /// Y is an alignment, not absolute.
    pub const VERALIGNED: u32 = NCVISUAL_OPTION_VERALIGNED;

    /// X is an alignment, not absolute.
    pub const HORALIGNED: u32 = NCVISUAL_OPTION_HORALIGNED;

    /// Uses non-interpolative scaling.
    pub const NOINTERPOLATE: u32 = NCVISUAL_OPTION_NOINTERPOLATE;
}
