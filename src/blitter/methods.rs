//! `NcBlitter` methods

pub use crate::{c_api, error, NcBlitter, NcResult, NcVisualOptions};
use core::ffi::c_void;

impl NcBlitter {
    /// The number of `height` subdivisions in a cell using the current blitter.
    ///
    /// Default & Pixel returns `None`.
    pub const fn cell_height(&self) -> Option<u8> {
        // self.cell_size().and_then(|size| Some(size.0) ) // not const
        if let Some(size) = self.cell_size() {
            Some(size.0)
        } else {
            None
        }
    }

    /// The number of `width` subdivisions in a cell using the current blitter.
    ///
    /// Default & Pixel returns `None`.
    pub const fn cell_width(&self) -> Option<u8> {
        // self.cell_size().and_then(|size| Some(size.1) ) // not const
        if let Some(size) = self.cell_size() {
            Some(size.1)
        } else {
            None
        }
    }

    /// The inner Cell's dimensions `(height, width)` using the current blitter.
    ///
    /// Default & Pixel returns `None`.
    pub const fn cell_size(&self) -> Option<(u8, u8)> {
        use NcBlitter::*;
        match self {
            Ascii => Some((1, 1)),
            Half => Some((2, 1)),
            Quadrant => Some((2, 2)),
            Sextant => Some((3, 2)),
            Braille => Some((4, 2)),
            _4x1 => Some((4, 1)),
            _8x1 => Some((8, 1)),
            _ => None,
        }
    }
}

impl NcBlitter {
    /// Blits a flat array `data` of [`NcRgba`] values to the [`NcPlane`] that
    /// must be configured in `vopts`.
    ///
    /// The blit begins at `vopts.y` and `vopts.x` relative to the plane.
    ///
    /// Each source row ought occupy `line_size` bytes (this might be greater
    /// than `vopts.lenx` * 4 due to padding or partial blits).
    ///
    /// A subregion of the input can be specified with the `begy`×`begx` and
    /// `leny`×`lenx` fields from `vopts`.
    ///
    /// Returns the number of pixels blitted on success.
    ///
    /// *C style function: [ncblit_rgba()][c_api::ncblit_rgba].*
    ///
    /// [`NcRgba`]: crate::NcRgba
    /// [`NcPlane`]: crate::NcPlane
    pub fn blit_rgba(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { c_api::ncblit_rgba(data_ptr, line_size as i32, vopts) };
        error![
            res,
            &format!["NcBlitter::blit_rgba(data, {}, {:?})", line_size, vopts],
            res as usize
        ];
    }

    /// Like [`blit_rgba`][NcBlitter#method.blit_rgba], but for BGRx.
    ///
    /// *C style function: [ncblit_bgrx()][c_api::ncblit_bgrx].*
    pub fn blit_bgrx(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { c_api::ncblit_bgrx(data_ptr, line_size as i32, vopts) };
        error![
            res,
            &format!["NcBlitter::blit_bgrx(data, {}, {:?})", line_size, vopts],
            res as usize
        ];
    }

    /// Like [`blit_rgba`][NcBlitter#method.blit_rgba], but for RGB.
    ///
    /// `line_size` must be a multiple of 3 for this RGB data.
    ///
    /// Supply an `alpha` value to be applied throughout.
    ///
    /// *C style function: [ncblit_rgb_packed()][c_api::ncblit_rgb_packed].*
    pub fn blit_rgb_packed(
        data: &[u8],
        line_size: usize,
        vopts: &NcVisualOptions,
        alpha: u8,
    ) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res =
            unsafe { c_api::ncblit_rgb_packed(data_ptr, line_size as i32, vopts, alpha as i32) };
        error![
            res,
            &format![
                "NcBlitter::blit_rgb_packed(data, {}, {:?}, {})",
                line_size, vopts, alpha
            ],
            res as usize
        ];
    }

    /// Like [`blit_rgb_packed`][NcBlitter#method.blit_rgb_packed], but `line_size`
    /// must be a multiple of 4 for this RGBx data.
    ///
    /// Supply an `alpha` value to be applied throughout.
    ///
    /// *C style function: [ncblit_rgb_loose()][c_api::ncblit_rgb_loose].*
    pub fn blit_rgb_loose(
        data: &[u8],
        line_size: usize,
        vopts: &NcVisualOptions,
        alpha: u8,
    ) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res =
            unsafe { c_api::ncblit_rgb_loose(data_ptr, line_size as i32, vopts, alpha as i32) };
        error![
            res,
            &format![
                "NcBlitter::blit_rgb_loose(data, {}, {:?}, {})",
                line_size, vopts, alpha
            ],
            res as usize
        ];
    }
}
