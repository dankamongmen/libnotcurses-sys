//!

use crate::{error, fns, NcDim, NcResult, NcVisualOptions};
use core::ffi::c_void;

/// The blitter mode to use for rasterizing an [`NcVisual`][crate::NcVisual].
///
/// We never blit full blocks, but instead spaces (more efficient) with the
/// background set to the desired foreground.
///
/// ## Modes
///
/// - [`NCBLIT_1x1`]
/// - [`NCBLIT_2x1`]
/// - [`NCBLIT_2x2`]
/// - [`NCBLIT_3x2`]
/// - [`NCBLIT_4x1`]
/// - [`NCBLIT_8x1`]
/// - [`NCBLIT_BRAILLE`]
/// - [`NCBLIT_DEFAULT`]
/// - [`NCBLIT_PIXEL`]
///
/// There is a mechanism of graceful degradation, that works as follows:
/// - without braille support, NCBLIT_BRAILLE decays to NCBLIT_3x2
/// - without bitmap support, NCBLIT_PIXEL decays to NCBLIT_3x2
/// - without sextant support, NCBLIT_3x2 decays to NCBLIT_2x2
/// - without quadrant support, NCBLIT_2x2 decays to NCBLIT_2x1
/// - the only viable blitters in ASCII are NCBLIT_1x1 and NCBLIT_PIXEL
///
/// If you don't want this behaviour you have to use
/// [`NCVISUAL_OPTION_NODEGRADE`][crate::NCVISUAL_OPTION_NODEGRADE]
///
pub type NcBlitter = u32;

/// [`NcBlitter`] mode using: space, compatible with ASCII
pub const NCBLIT_1x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_1x1;

/// [`NcBlitter`] mode using: halves + 1x1 (space)
/// ▄▀
pub const NCBLIT_2x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_2x1;

/// [`NcBlitter`] mode using: quadrants + 2x1
/// ▗▐ ▖▀▟▌▙
pub const NCBLIT_2x2: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_2x2;

/// [`NcBlitter`] mode using: sextants
/// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
pub const NCBLIT_3x2: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_3x2;

/// [`NcBlitter`] mode using: four vertical levels
/// █▆▄▂
pub const NCBLIT_4x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_4x1;

/// [`NcBlitter`] mode using: eight vertical levels
/// █▇▆▅▄▃▂▁
pub const NCBLIT_8x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_8x1;

/// [`NcBlitter`] mode using: 4 rows, 2 cols (braille)
/// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
pub const NCBLIT_BRAILLE: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_BRAILLE;

/// [`NcBlitter`] mode where the blitter is automatically chosen
pub const NCBLIT_DEFAULT: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_DEFAULT;

/// Sixel/Pixel mode
///
/// NCBLIT_PIXEL
///
/// See [Sixel in Wikipedia](https://en.wikipedia.org/wiki/Sixel).
pub const NCBLIT_PIXEL: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_PIXEL;

/// Contains the blitter geometry information as returned by the
/// NcPlane.[blitter_geom()][crate::NcPlane#method.blitter_geom] method.
///
/// - `y`, `x`: the input size in pixels.
/// - `scale_y`, `scale_x`: the scaling
/// - `blitter` The blitter that will be used
///
#[derive(Clone, Debug)]
pub struct NcBlitterGeometry {
    ///
    pub y: NcDim,
    ///
    pub x: NcDim,
    ///
    pub scale_y: NcDim,
    ///
    pub scale_x: NcDim,
    /// The blitter that will be used.
    pub blitter: NcBlitter,
}

/// Enables the [`NcBlitter`] methods.
pub trait NcBlitterMethods {
    fn blit_bgrx(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize>;
    fn blit_rgba(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize>;
    fn blit_rgb_loose(
        data: &[u8],
        line_size: usize,
        vopts: &NcVisualOptions,
        alpha: u8,
    ) -> NcResult<usize>;
    fn blit_rgb_packed(
        data: &[u8],
        line_size: usize,
        vopts: &NcVisualOptions,
        alpha: u8,
    ) -> NcResult<usize>;
}

impl NcBlitterMethods for NcBlitter {
    /// Blit a flat array `data` of RGBA 32-bit values to the plane configured
    /// in `vopts`, which mustn’t be NULL.
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
    /// *C style function: [ncblit_rgba()][fns::ncblit_rgba].*
    fn blit_rgba(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { fns::ncblit_rgba(data_ptr, line_size as i32, vopts) };
        error![
            res,
            &format!["NcBlitter::blit_rgba(data, {}, {:?})", line_size, vopts],
            res as usize
        ];
    }

    /// Like [`blit_rgba`][NcBlitter#method.blit_rgba], but for BGRx.
    ///
    /// *C style function: [ncblit_bgrx()][fns::ncblit_bgrx].*
    fn blit_bgrx(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { fns::ncblit_bgrx(data_ptr, line_size as i32, vopts) };
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
    /// *C style function: [ncblit_rgb_packed()][fns::ncblit_rgb_packed].*
    fn blit_rgb_packed(
        data: &[u8],
        line_size: usize,
        vopts: &NcVisualOptions,
        alpha: u8,
    ) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res =
            unsafe { fns::ncblit_rgb_packed(data_ptr, line_size as i32, vopts, alpha as i32) };
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
    /// *C style function: [ncblit_rgb_loose()][fns::ncblit_rgb_loose].*
    fn blit_rgb_loose(
        data: &[u8],
        line_size: usize,
        vopts: &NcVisualOptions,
        alpha: u8,
    ) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { fns::ncblit_rgb_loose(data_ptr, line_size as i32, vopts, alpha as i32) };
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
