/// Indicates how to scale an [`NcVisual`][crate::NcVisual] during rendering
/// (alias of `u32`).
///
/// - `NOSCALE` will apply no scaling.
/// - `SCALE` scales a visual to the plane's size,
///   maintaining aspect ratio.
/// - `STRETCH` stretches and scales the image in an
///   attempt to fill the entirety of the plane.
/// - `NONE_HIRES` like `NONE` admitting high-res blitters.
/// - `SCALE_HIRES` like `CALE` admitting high-res blitters.
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
pub type NcScale = u32; // crate::bindings::ffi::ncscale_e;

crate::impl_api![
    NcScale,
    NcScaleApi,
    /// Maintains original size.
    const NOSCALE: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_NONE;,
    /// Maintains aspect ratio.
    const SCALE: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_SCALE;,
    /// Throws away aspect ratio.
    const STRETCH: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_STRETCH;,
    /// Maintains original size, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    const NONE_HIRES: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_NONE_HIRES;,
    /// Maintains aspect ratio, admitting high-resolution blitters
    /// that don't preserve aspect ratio.
    const SCALE_HIRES: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_SCALE_HIRES;
];

pub(crate) mod constants {
    use crate::NcScale;

    /// [`NcScale`] mode that maintains the original size.
    pub const NCSCALE_NONE: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_NONE;

    /// [`NcScale`] mode that maintains the aspect ratio.
    pub const NCSCALE_SCALE: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_SCALE;

    /// [`NcScale`] mode that throws away the aspect ratio.
    pub const NCSCALE_STRETCH: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_STRETCH;

    /// [`NcScale`] mode that maintains the original size, admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    pub const NCSCALE_NONE_HIRES: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_NONE_HIRES;

    /// [`NcScale`] mode that maintains the aspect ratio, admitting
    /// high-resolution blitters that don't preserve the aspect ratio.
    pub const NCSCALE_SCALE_HIRES: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_SCALE_HIRES;
}
