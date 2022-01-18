//!

mod builder;
pub(crate) mod c_api;
pub use builder::NcVisualOptionsBuilder;

use core::ptr::null_mut;

use crate::{NcBlitter, NcPlane, NcRgba, NcScale};

/// Options struct for [`NcVisual`][crate::NcVisual].
///
/// It is recommended to construct it via [`NcVisualOptionsBuilder`]
/// by calling [`NcVisualOptions::builder()`].
///
/// # Usage
///
/// If a plane is not provided, one will be created, having the exact size
/// necessary to display the visual (this might be smaller or larger than
/// the rendering area). if [`NcVisualOptions::CHILDPLANE`] is provided,
/// this will be interpreted as the parent.
///
/// A subregion of the visual can be rendered using `beg_y`, `beg_x`, `len_y`,
/// and `len_x`.
///
/// # Fields
///
/// * [`n`]: an optional mutable pointer to an [`NcPlane`].
///
/// * [`scaling`]: how the source will be stretched/scaled relative to the
///   `NcPlane` ([`NOSCALE`], [`SCALE`], [`STRETCH`], [`NONE_HIRES`],
///   [`SCALE_HIRES`]).
///
/// * [`y`]: if an `NcPlane` is provided in `n` then this specifies where the
///   `NcVisual` will be on that plane.
///
///   Otherwise it specifies where the created `NcPlane` will be placed relative
///   to the standard plane's origin.
///
///   If [`VERALIGNED`] is set, this will be interpreted as an [`NcAlign`] value.
///
/// * [`x`]: if an `NcPlane` is provided in `n` then this specifies where the
///   `NcVisual` will be on that plane.
///
///   Otherwise it specifies where the created `NcPlane` will be placed relative
///   to the standard plane's origin.
///
///   If [`HORALIGNED`] is set, this will be interpreted as an [`NcAlign`] value.
///
/// * [`begy`]: origin of rendered section in the *y* axis.
/// * [`begx`]: origin of rendered section in the *x* axis.
/// * [`leny`]: length of rendered section in the *y* axis.
/// * [`lenx`]: length of rendered section in the *x* axis.
///
/// * [`blitter`]: [`NcBlitter`] glyph set to use for blitting.
///
/// * [`flags`]: bitmask of options: [`ADDALPHA`], [`BLEND`], [`CHILDPLANE`],
///   [`NODEGRADE`], [`VERALIGNED`], [`HORALIGNED`], [`NOINTERPOLATE`].
///
/// * [`transcolor`]: treats this color as transparent when the [`ADDALPHA`] flag
///   is active.
///
/// * [`pxoffy`]: pixel offset within the cell in the *y* axis.
///
///   If [`NcBlitter::Pixel`] is used the bitmap will be drawn offset from the
///   upper-left cell’s origin by these amounts, otherwise this will be ignored.
///
///   It is an error if either number exceeds the cell-pixel geometry in any
///   dimension (see [`NcPixelGeometry.cell_y`], [`NcVisualGeometry.cdim_yx`]).
///
/// * [`pxoffx`]: pixel offset within the cell in the *x* axis.
///
///   If [`NcBlitter::Pixel`] is used, the bitmap will be drawn offset from the
///   upper-left cell’s origin by these amounts, otherwise this will be ignored.
///
///   It is an error if either number exceeds the cell-pixel geometry in any
///   dimension (see [`NcPixelGeometry.cell_x`], [`NcVisualGeometry.cdim_yx`]).
///
/// [`NcVisualOptions::builder()`]: NcVisualOptions#method.builder
/// [`NcAlign`]: crate::NcAlign
/// [`NcPixelGeometry.cell_y`]: crate::NcPixelGeometry#structfield.cell_y
/// [`NcPixelGeometry.cell_x`]: crate::NcPixelGeometry#structfield.cell_x
/// [`NcVisualGeometry.cdim_yx`]: crate::NcVisualGeometry#structfield.cdim_yx
/// [`n`]: crate::c_api::ffi::ncvisual_options#structfield.n
/// [`scaling`]: crate::c_api::ffi::ncvisual_options#structfield.scaling
/// [`y`]: crate::c_api::ffi::ncvisual_options#structfield.y
/// [`x`]: crate::c_api::ffi::ncvisual_options#structfield.x
/// [`begy`]: crate::c_api::ffi::ncvisual_options#structfield.begy
/// [`begx`]: crate::c_api::ffi::ncvisual_options#structfield.begx
/// [`leny`]: crate::c_api::ffi::ncvisual_options#structfield.leny
/// [`lenx`]: crate::c_api::ffi::ncvisual_options#structfield.lenx
/// [`blitter`]: crate::c_api::ffi::ncvisual_options#structfield.blitter
/// [`flags`]: crate::c_api::ffi::ncvisual_options#structfield.flags
/// [`transcolor`]: crate::c_api::ffi::ncvisual_options#structfield.transcolor
/// [`pxoffy`]: crate::c_api::ffi::ncvisual_options#structfield.pxoffy
/// [`pxoffx`]: crate::c_api::ffi::ncvisual_options#structfield.pxoffx
/// [`NOSCALE`]: crate::NcScale#associatedconstant.NOSCALE
/// [`SCALE`]: crate::NcScale#associatedconstant.SCALE
/// [`STRETCH`]: crate::NcScale#associatedconstant.STRETCH
/// [`NONE_HIRES`]: crate::NcScale#associatedconstant.NONE_HIRES
/// [`SCALE_HIRES`]: crate::NcScale#associatedconstant.SCALE_HIRES
/// [`ADDALPHA`]: NcVisualOptions#associatedconstant.ADDALPHA
/// [`BLEND`]: NcVisualOptions#associatedconstant.BLEND
/// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
/// [`NcVisualOptions::CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
/// [`NODEGRADE`]: NcVisualOptions#associatedconstant.NODEGRADE
/// [`VERALIGNED`]:NcVisualOptions#associatedconstant.VERALIGNED
/// [`HORALIGNED`]: NcVisualOptions#associatedconstant.HORALIGNED
/// [`NOINTERPOLATE`]: NcVisualOptions#associatedconstant.NOINTERPOLATE
pub type NcVisualOptions = crate::bindings::ffi::ncvisual_options;

/// # Constructors
impl<'ncplane> NcVisualOptions {
    /// Returns a builder object for `NcVisualOptions`.
    pub fn builder() -> NcVisualOptionsBuilder<'ncplane> {
        NcVisualOptionsBuilder::default()
    }

    /// New `NcVisualOptions`.
    ///
    /// # Arguments
    ///
    /// * `plane` - an optional mutable pointer to an [`NcPlane`].
    ///
    /// * `scale` - how the source will be stretched/scaled relative to the
    ///   `NcPlane` ([`NOSCALE`], [`SCALE`], [`STRETCH`], [`NONE_HIRES`],
    ///   [`SCALE_HIRES`]).
    ///
    /// * `y` - if an `NcPlane` is provided in `plane` then this specifies where
    ///   the `NcVisual` will be on that plane in the *y* axis.
    ///
    ///   Otherwise it specifies where the created `NcPlane` will be placed
    ///   in the *y* axis, relative to the standard plane's origin.
    ///
    ///   If [`VERALIGNED`] is set, this will be interpreted as an [`NcAlign`]
    ///   value.
    ///
    /// * `x` - if an `NcPlane` is provided in `plane` then this specifies where
    ///   the `NcVisual` will be on that plane, in the *x* axis.
    ///
    ///   Otherwise it specifies where the created `NcPlane` will be placed,
    ///   in the *y* axis, relative to the standard plane's origin.
    ///
    ///   If [`HORALIGNED`] is set, this will be interpreted as an [`NcAlign`]
    ///   value.
    ///
    /// * `section_yx_lenyx` - The size of the rendered section.
    ///
    ///   `None` renders the entire visual, otherwise the provided tuple
    ///   (`y`, `x`, `len_y`, `len_x`) sets `[yx]` as the origin of the section
    ///   and `len_[yx]` as the its length on each respective dimension.
    ///
    /// * `cell_offset_yx` - Pixel offsets within the cell.
    ///
    ///   If [`NcBlitter::Pixel`] is used the bitmap will be drawn offset from
    ///   the upper-left cell’s origin by these amounts, otherwise this will be
    ///   ignored.
    ///
    ///   It is an error if either number exceeds the cell-pixel geometry in any
    ///   dimension (see [`NcVisualGeometry.cdim_yx`]).
    ///
    /// * `blitter` - [`NcBlitter`] glyph set to use for blitting.
    ///
    /// * `flags` - bitmask of options: ([`ADDALPHA`], [`BLEND`], [`CHILDPLANE`],
    ///   [`NODEGRADE`], [`VERALIGNED`], [`HORALIGNED`], [`NOINTERPOLATE`]).
    ///
    /// * `transcolor` - treats this color as transparent when the [`ADDALPHA`]
    ///   flag is active
    ///
    /// # Notes
    ///
    /// If the [`NcVisualOptions::CHILDPLANE`] flag is used then the `plane` is
    /// interpreted as the parent `NcPlane` of the new plane created for this
    /// [`NcVisual`][crate::NcVisual].
    ///
    /// [`NcAlign`]: crate::NcAlign
    /// [`NcPixelGeometry.cell_y`]: crate::NcPixelGeometry#structfield.cell_y
    /// [`NcPixelGeometry.cell_x`]: crate::NcPixelGeometry#structfield.cell_x
    /// [`NcVisualGeometry.cdim_yx`]: crate::NcVisualGeometry#structfield.cdim_yx
    /// [`NOSCALE`]: NcScale#associatedconstant.NOSCALE
    /// [`SCALE`]: NcScale#associatedconstant.SCALE
    /// [`STRETCH`]: NcScale#associatedconstant.STRETCH
    /// [`NONE_HIRES`]: NcScale#associatedconstant.NONE_HIRES
    /// [`SCALE_HIRES`]: NcScale#associatedconstant.SCALE_HIRES
    /// [`ADDALPHA`]: NcVisualOptions#associatedconstant.ADDALPHA
    /// [`BLEND`]: NcVisualOptions#associatedconstant.BLEND
    /// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
    /// [`NODEGRADE`]: NcVisualOptions#associatedconstant.NODEGRADE
    /// [`VERALIGNED`]:NcVisualOptions#associatedconstant.VERALIGNED
    /// [`HORALIGNED`]: NcVisualOptions#associatedconstant.HORALIGNED
    /// [`NOINTERPOLATE`]: NcVisualOptions#associatedconstant.NOINTERPOLATE
    pub fn new<RGBA: Into<NcRgba>>(
        plane: Option<&mut NcPlane>,
        scale: NcScale,
        y: i32,
        x: i32,
        section_yx_lenyx: Option<(u32, u32, u32, u32)>,
        cell_offset_yx: Option<(u32, u32)>,
        blitter: NcBlitter,
        flags: u32,
        transcolor: RGBA,
    ) -> Self {
        let plane_ptr = if let Some(p) = plane { p } else { null_mut() };
        let (begy, begx, leny, lenx) =
            if let Some(s) = section_yx_lenyx { (s.0, s.1, s.2, s.3) } else { (0, 0, 0, 0) };
        let (pxoffy, pxoffx) = if let Some(o) = cell_offset_yx { (o.0, o.1) } else { (0, 0) };

        Self {
            n: plane_ptr,
            scaling: scale.into(),

            y,
            x,

            begy,
            begx,
            leny,
            lenx,

            blitter: blitter.into(),

            flags: flags as u64,

            transcolor: transcolor.into().into(),

            pxoffy,
            pxoffx,
        }
    }
}
