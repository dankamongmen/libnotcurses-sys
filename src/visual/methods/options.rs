//!

use core::ptr::null_mut;

use crate::{NcBlitter, NcBlitterApi, NcDim, NcPlane, NcRgba, NcScale, NcVisualOptions};

/// # NcVisualOptions Constructors
impl NcVisualOptions {
    /// New default `NcVisualOptions`.
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn new_aligned() -> Self {
    //     Self::with_flags_aligned()
    // }

    // TODO:
    // - horizontally aligned
    // - copy from NcPlaneOptions (with_flags_aligned & with_flags,)
    // y is an ncalign_e if NCVISUAL_OPTION_VERALIGNED is provided.
    // x is an ncalign_e value if NCVISUAL_OPTION_HORALIGNED is provided.

    /// Specify an existing plane.
    ///
    /// If [`NcVisualOptions::CHILDPLANE`][NcVisualOptions#associatedconstant.CHILDPLANE]
    /// is used in `flags` then the `plane` is interpreted as the parent
    /// [`NcPlane`] of the new plane created for this [`NcVisual`][crate::NcVisual].
    pub fn with_plane(
        plane: &mut NcPlane,
        scale: NcScale,
        y: NcDim,
        x: NcDim,
        beg_y: NcDim,
        beg_x: NcDim,
        len_y: NcDim,
        len_x: NcDim,
        // pxoff_y: NcDim,
        // pxoff_x: NcDim,
        blitter: NcBlitter,
        flags: u32,
        transcolor: NcRgba,
    ) -> Self {
        Self {
            // provided plane
            n: plane,
            // the source is stretched/scaled relative to the provided ncplane
            scaling: scale,
            y: y as i32,
            x: x as i32,
            // origin of rendered section
            begy: beg_y,
            begx: beg_x,
            // size of rendered section
            leny: len_y,
            lenx: len_x,
            // glyph set to use
            blitter,
            // bitmask over NCVISUAL_OPTION_*
            flags: flags as u64,
            transcolor,
            // WIP
            pxoffy: 0,
            pxoffx: 0,
        }
    }

    // TODO: use Option<> groups for coords
    pub fn without_plane(
        y: NcDim,
        x: NcDim,
        beg_y: NcDim,
        beg_x: NcDim,
        len_y: NcDim,
        len_x: NcDim,
        // pxoff_y: NcDim,
        // pxoff_x: NcDim,
        blitter: NcBlitter,
        flags: u32,
        transcolor: u32,
    ) -> Self {
        Self {
            n: null_mut(),
            scaling: crate::c_api::NCSCALE_NONE,
            // where the created plane will be placed relative to stdplane's origin
            y: y as i32,
            x: x as i32,
            // origin of rendered section
            begy: beg_y,
            begx: beg_x,
            // size of rendered section
            leny: len_y,
            lenx: len_x,
            // glyph set to use
            blitter,
            // bitmask over NCVISUAL_OPTION_*
            flags: flags as u64,
            // This color will be treated as transparent with flag [NCVISUAL_OPTION_ADDALPHA].
            transcolor,
            // pixel offsets within the cell.
            // if NCBLIT_PIXEL is used, the bitmap will be drawn offset from the
            // upper-left cell's origin by these amounts. it is an error if
            // either number exceeds the cell-pixel geometry in its dimension.
            // if NCBLIT_PIXEL is not used, these fields are ignored.
            // this functionality can be used for smooth bitmap movement.
            // WIP
            pxoffy: 0,
            pxoffx: 0,
            // pxoffy: pxoff_y,
            // pxoffx: pxoff_x,
        }
    }

    pub fn fullsize_pixel_without_plane(y: NcDim, x: NcDim, len_y: NcDim, len_x: NcDim) -> Self {
        Self::without_plane(y, x, 0, 0, len_y, len_x, NcBlitter::PIXEL, 0, 0)
    }
}
