//!

use crate::{c_api, error, NcResult, NcVisualOptions};
use core::ffi::c_void;

pub(crate) mod geometry;

/// The blitter mode to use for rasterizing an [`NcVisual`][crate::NcVisual]
/// (alias of [`u32`]).
///
/// We never blit full blocks, but instead spaces (more efficient) with the
/// background set to the desired foreground.
///
/// There is a mechanism of graceful degradation, that works as follows:
/// - without braille support, [`NcBlitter::BRAILLE`] decays to [`NcBlitter::_3x2`].
/// - without bitmap support, [`NcBlitter::PIXEL`] decays to [`NcBlitter::_3x2`].
/// - without sextant support, [`NcBlitter::_3x2`] decays to [`NcBlitter::_2x2`].
/// - without quadrant support, [`NcBlitter::_2x2`] decays to [`NcBlitter::_2x1`].
/// - the only viable blitters in ASCII are [`NCBlitter::_1x1`] and [`NcBlitter::PIXEL`].
///
/// [`NCBlitter::BRAILLE`]: NcBlitter#associatedconstant.BRAILLE
/// [`NCBlitter::PIXEL`]: NcBlitter#associatedconstant.PIXEL
/// [`NCBlitter::_1x1`]: NcBlitter#associatedconstant._1x1
/// [`NCBlitter::_2x1`]: NcBlitter#associatedconstant._2x1
/// [`NCBlitter::_2x2`]: NcBlitter#associatedconstant._2x2
/// [`NCBlitter::_3x2`]: NcBlitter#associatedconstant._3x2
///
/// If you don't want this behaviour you have to use
/// [`NcVisualOptions::NODEGRADE`][crate::NcVisualOptions#associatedconstant.NODEGRADE]
pub type NcBlitter = u32;

crate::impl_api![
    NcBlitter,
    NcBlitterApi,
    /// [`NcBlitter`] mode where the blitter is automatically chosen.
    const DEFAULT: NcBlitter = constants::NCBLIT_DEFAULT;,
    /// [`NcBlitter`] mode using: space, compatible with ASCII.
    const _1x1: NcBlitter = constants::NCBLIT_1x1;,
    /// [`NcBlitter`] mode using: halves + 1x1 (space).
    /// ▄▀
    const _2x1: NcBlitter = constants::NCBLIT_2x1;,
    /// [`NcBlitter`] mode using: quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    const _2x2: NcBlitter = constants::NCBLIT_2x2;,
    /// [`NcBlitter`] mode using: sextants
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    const _3x2: NcBlitter = constants::NCBLIT_3x2;,
    /// [`NcBlitter`] mode using: four vertical levels.
    /// █▆▄▂
    const _4x1: NcBlitter = constants::NCBLIT_4x1;,
    /// [`NcBlitter`] mode using: eight vertical levels.
    /// █▇▆▅▄▃▂▁
    const _8x1: NcBlitter = constants::NCBLIT_8x1;,
    /// [`NcBlitter`] mode using: 4 rows, 2 cols (braille).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    const BRAILLE: NcBlitter = constants::NCBLIT_BRAILLE;,
    /// Sixel/Pixel mode.
    ///
    /// See [Sixel in Wikipedia](https://en.wikipedia.org/wiki/Sixel).
    const PIXEL: NcBlitter = constants::NCBLIT_PIXEL;,
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
    /// *C style function: [ncblit_rgba()][c_api::ncblit_rgba].*
    fn blit_rgba(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { c_api::ncblit_rgba(data_ptr, line_size as i32, vopts) };
        error![
            res,
            &format!["NcBlitter::blit_rgba(data, {}, {:?})", line_size, vopts],
            res as usize
        ];
    },
    /// Like [`blit_rgba`][NcBlitter#method.blit_rgba], but for BGRx.
    ///
    /// *C style function: [ncblit_bgrx()][c_api::ncblit_bgrx].*
    fn blit_bgrx(data: &[u8], line_size: usize, vopts: &NcVisualOptions) -> NcResult<usize> {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        let res = unsafe { c_api::ncblit_bgrx(data_ptr, line_size as i32, vopts) };
        error![
            res,
            &format!["NcBlitter::blit_bgrx(data, {}, {:?})", line_size, vopts],
            res as usize
        ];
    },
    /// Like [`blit_rgba`][NcBlitter#method.blit_rgba], but for RGB.
    ///
    /// `line_size` must be a multiple of 3 for this RGB data.
    ///
    /// Supply an `alpha` value to be applied throughout.
    ///
    /// *C style function: [ncblit_rgb_packed()][c_api::ncblit_rgb_packed].*
    fn blit_rgb_packed(
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
    },
    /// Like [`blit_rgb_packed`][NcBlitter#method.blit_rgb_packed], but `line_size`
    /// must be a multiple of 4 for this RGBx data.
    ///
    /// Supply an `alpha` value to be applied throughout.
    ///
    /// *C style function: [ncblit_rgb_loose()][c_api::ncblit_rgb_loose].*
    fn blit_rgb_loose(
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
];

pub(crate) mod constants {
    use crate::NcBlitter;

    /// [`NcBlitter`] mode where the blitter is automatically chosen.
    pub const NCBLIT_DEFAULT: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_DEFAULT;
    /// [`NcBlitter`] mode using: space, compatible with ASCII.
    pub const NCBLIT_1x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_1x1;
    /// [`NcBlitter`] mode using: halves + 1x1 (space).
    /// ▄▀
    pub const NCBLIT_2x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_2x1;
    /// [`NcBlitter`] mode using: quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    pub const NCBLIT_2x2: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_2x2;
    /// [`NcBlitter`] mode using: sextants.
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    pub const NCBLIT_3x2: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_3x2;
    /// [`NcBlitter`] mode using: four vertical levels.
    /// █▆▄▂
    pub const NCBLIT_4x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_4x1;
    /// [`NcBlitter`] mode using: eight vertical levels.
    /// █▇▆▅▄▃▂▁
    pub const NCBLIT_8x1: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_8x1;
    /// [`NcBlitter`] mode using: 4 rows, 2 cols (braille).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    pub const NCBLIT_BRAILLE: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_BRAILLE;
    /// Sixel/Pixel mode.
    ///
    /// See [Sixel in Wikipedia](https://en.wikipedia.org/wiki/Sixel).
    pub const NCBLIT_PIXEL: NcBlitter = crate::bindings::ffi::ncblitter_e_NCBLIT_PIXEL;
}
