//! `NcScale`

use std::fmt;

/// Indicates how to scale an [`NcVisual`][crate::NcVisual] during rendering.
///
/// Default: *`None`*.
///
/// The scaling preferences are applied only for the context of
/// [`NcVisual.render`][crate::NcVisual#method.render].
///
/// You can think of it as a pipeline:
/// ```txt
/// NcVisual::from_file() → frame → NcVisual.render() → scaling → output frame → blit
/// ```
/// where you still have the original frame.
///
/// Whereas
/// [`NcVisual.resize`][crate::NcVisual#method.resize] and
/// [`NcVisual.resize_noninterpolative`][crate::NcVisual#method.resize_noninterpolative]
/// are changing that original frame.
///
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcScale {
    /// Maintains the original size. Will Apply no scaling.
    None = c_api::NCSCALE_NONE,

    /// Maintains the aspect ratio.
    ///
    /// Scales an `NcVisual` to the `NcPlane`'s size without stretching.
    Scale = c_api::NCSCALE_SCALE,

    /// Like `None`, maintains the original size, while admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    NoneHires = c_api::NCSCALE_NONE_HIRES,

    /// Like `Scale`, maintains the aspect ratio, while admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    ScaleHires = c_api::NCSCALE_SCALE_HIRES,

    /// Throws away aspect ratio.
    ///
    /// Stretches and scales the `NcVisual` in an attempt to fill the entirety
    /// of the `NcPlane`.
    Stretch = c_api::NCSCALE_STRETCH,
}

impl Default for NcScale {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for NcScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NcScale::*;
        write!(
            f,
            "{}",
            match self {
                None => "None",
                Scale => "Scale",
                NoneHires => "NoneHires",
                ScaleHires => "ScaleHires",
                Stretch => "Stretch",
            }
        )
    }
}

impl From<c_api::NcScale_u32> for NcScale {
    fn from(scale: c_api::NcScale_u32) -> Self {
        use {c_api::*, NcScale::*};
        match scale {
            NCSCALE_NONE => None,
            NCSCALE_SCALE => Scale,
            NCSCALE_NONE_HIRES => NoneHires,
            NCSCALE_SCALE_HIRES => ScaleHires,
            NCSCALE_STRETCH => Stretch,
            _ => None, // invalid values default to `None`
        }
    }
}

impl From<NcScale> for c_api::NcScale_u32 {
    fn from(scale: NcScale) -> Self {
        use {c_api::*, NcScale::*};
        match scale {
            None => NCSCALE_NONE,
            Scale => NCSCALE_SCALE,
            NoneHires => NCSCALE_NONE_HIRES,
            ScaleHires => NCSCALE_SCALE_HIRES,
            Stretch => NCSCALE_STRETCH,
        }
    }
}

pub(crate) mod c_api {
    use crate::bindings::ffi;

    /// Indicates how to scale an [`NcVisual`][crate::NcVisual] during rendering.
    ///
    /// It's recommended to use [`NcScale`][crate::NcScale] instead.
    ///
    /// # Associated `c_api` constants
    ///
    /// - [`NCSCALE_NONE`] will apply no scaling.
    /// - [`NCSCALE_SCALE`] scales a visual to the plane's size,
    ///   maintaining aspect ratio.
    /// - [`NCSCALE_STRETCH`] stretches and scales the image in an
    ///   attempt to fill the entirety of the plane.
    /// - [`NCSCALE_NONE_HIRES`] like `NONE` admitting high-res blitters.
    /// - [`NCSCALE_SCALE_HIRES`] like `CALE` admitting high-res blitters.
    ///
    /// The `NCSCALE_*` preferences are applied only for the context of
    /// [`NcVisual.render`][crate::NcVisual#method.render].
    /// You can think of it as a pipeline:
    ///
    /// ```txt
    /// NcVisual::fromfile() → frame → NcVisual.render() → scaling → output frame → blit
    /// ```
    ///
    /// where you still have the original frame. Whereas
    /// [`NcVisual.resize`][crate::NcVisual#method.resize] and
    /// [`NcVisual.resize_noninterpolative`][crate::NcVisual#method.resize_noninterpolative]
    /// are changing that original frame.
    ///
    pub type NcScale_u32 = u32; // crate::bindings::ffi::ncscale_e;

    /// [`NcScale_u32`] mode that maintains the original size.
    pub const NCSCALE_NONE: NcScale_u32 = ffi::ncscale_e_NCSCALE_NONE;

    /// [`NcScale_u32`] mode that maintains the aspect ratio.
    pub const NCSCALE_SCALE: NcScale_u32 = ffi::ncscale_e_NCSCALE_SCALE;

    /// [`NcScale_u32`] mode that throws away the aspect ratio.
    pub const NCSCALE_STRETCH: NcScale_u32 = ffi::ncscale_e_NCSCALE_STRETCH;

    /// [`NcScale_u32`] mode that maintains the original size, admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    pub const NCSCALE_NONE_HIRES: NcScale_u32 = ffi::ncscale_e_NCSCALE_NONE_HIRES;

    /// [`NcScale_u32`] mode that maintains the aspect ratio, admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    pub const NCSCALE_SCALE_HIRES: NcScale_u32 = ffi::ncscale_e_NCSCALE_SCALE_HIRES;
}
