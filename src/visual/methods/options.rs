//!

use core::ptr::null_mut;

use crate::{NcBlitter, NcDim, NcPlane, NcRgba, NcScale, NcVisualOptions};

/// # NcisualOptions Constructors
impl NcVisualOptions {
    /// New `NcVisualOptions`.
    ///
    /// # Arguments
    ///
    /// * `plane` -
    /// * `scale` -
    /// * `y` -
    /// * `x` -
    /// * `section_yx_lenyx` - The desired rendered section.
    ///  `None` would render the entire visual, otherwise the provided tuple
    ///  (`y`, `x`, `len_y`, `len_x`) sets `[yx]` as the origin of the section
    ///  and `len_[yx]` as the size on each respective dimension.
    ///
    /// * `cell_offset_yx` - Pixel offsets within the cell.
    ///   It is an error if either number exceeds the cell-pixel geometry in any
    ///   dimension (see [`NcVisualGeometry.cdim_yx`]).
    /// * `blitter` -
    /// * `flags` -
    /// * `transcolor` -
    ///
    /// # Notes
    ///
    /// If the [`NcVisualOptions::CHILDPLANE`] flag is used then the `plane` is
    /// interpreted as the parent `NcPlane` of the new plane created for this
    /// [`NcVisual`][crate::NcVisual].
    ///
    /// [`NcPixelGeometry.cell_y`]: crate::NcPixelGeometry#structfield.cell_y
    /// [`NcPixelGeometry.cell_x`]: crate::NcPixelGeometry#structfield.cell_x
    /// [`NcVisualGeometry.cdim_yx`]: crate::NcVisualGeometry#structfield.cdim_yx
    pub fn new(
        plane: Option<&mut NcPlane>,
        scale: NcScale,
        y: NcDim,
        x: NcDim,
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
            // optionally provided plane
            n: plane_ptr,
            // the source is stretched/scaled relative to the provided ncplane
            scaling: scale,

            y: y as i32,
            x: x as i32,

            // origin of rendered section
            begy,
            begx,
            // size of rendered section
            leny,
            lenx,

            blitter,

            // bitmask over NCVISUAL_OPTION_*
            flags: flags as u64,
            transcolor,

            // pixel offset, up to the cell geometry
            pxoffy,
            pxoffx,
        }
    }
}
