//!

use core::ptr::null_mut;

use crate::{
    NcBlitter, NcDim, NcOffset, NcPlane, NcRgba, NcScale, NcVisualOptions, NcVisualOptionsBuilder,
};

/// # NcisualOptions Constructors
impl<'ncplane> NcVisualOptions {
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
    ///   If [`NcBlitter::PIXEL`] is used the bitmap will be drawn offset from
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
    /// [`NcBlitter::PIXEL`]: crate::NcBlitter#associatedconstant.PIXEL
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
    pub fn new(
        plane: Option<&mut NcPlane>,
        scale: NcScale,
        y: NcOffset,
        x: NcOffset,
        section_yx_lenyx: Option<(NcDim, NcDim, NcDim, NcDim)>,
        cell_offset_yx: Option<(NcDim, NcDim)>,
        blitter: NcBlitter,
        flags: u32,
        transcolor: NcRgba,
    ) -> Self {
        let plane_ptr = if let Some(p) = plane { p } else { null_mut() };
        let (begy, begx, leny, lenx) = if let Some(s) = section_yx_lenyx {
            (s.0, s.1, s.2, s.3)
        } else {
            (0, 0, 0, 0)
        };
        let (pxoffy, pxoffx) = if let Some(o) = cell_offset_yx {
            (o.0, o.1)
        } else {
            (0, 0)
        };

        Self {
            n: plane_ptr,
            scaling: scale,

            y,
            x,

            begy,
            begx,
            leny,
            lenx,

            blitter,

            flags: flags as u64,

            transcolor,

            pxoffy,
            pxoffx,
        }
    }

    /// Returns a builder object for `NcVisualOptions`.
    pub fn builder() -> NcVisualOptionsBuilder<'ncplane> {
        NcVisualOptionsBuilder::default()
    }
}
