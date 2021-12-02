//! `NcBlitter` methods

use super::constants;
pub use crate::{c_api, error, NcBlitter, NcResult, NcVisualOptions};
use core::ffi::c_void;

crate::impl_api![
    NcBlitter,
    NcBlitterApi,
    /// [`NcBlitter`] mode where the blitter is automatically chosen.
    const DEFAULT: NcBlitter = constants::NCBLIT_DEFAULT;,
    /// [`NcBlitter`] mode using: space, compatible with ASCII.
    const ASCII: NcBlitter = constants::NCBLIT_1x1;,
    /// [`NcBlitter`] mode using: halves + 1x1 (space).
    /// ▄▀
    const HALF: NcBlitter = constants::NCBLIT_2x1;,
    /// [`NcBlitter`] mode using: quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    const QUADRANT: NcBlitter = constants::NCBLIT_2x2;,
    /// [`NcBlitter`] mode using: sextants
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    const SEXTANT: NcBlitter = constants::NCBLIT_3x2;,
    /// [`NcBlitter`] mode using: 4 rows, 2 cols (braille).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    const BRAILLE: NcBlitter = constants::NCBLIT_BRAILLE;,
    /// Sixel/Pixel mode.
    ///
    /// See [Sixel in Wikipedia](https://en.wikipedia.org/wiki/Sixel).
    const PIXEL: NcBlitter = constants::NCBLIT_PIXEL;,
    /// Alias of [`ASCII`][NcBlitter#associatedconstant.ASCII]
    const _1x1: NcBlitter = constants::NCBLIT_1x1;,
    /// Alias of [`HALF`][NcBlitter#associatedconstant.HALF]
    const _2x1: NcBlitter = constants::NCBLIT_2x1;,
    /// Alias of [`QUADRANT`][NcBlitter#associatedconstant.QUADRANT]
    const _2x2: NcBlitter = constants::NCBLIT_2x2;,
    /// Alias of [`SEXTANT`][NcBlitter#associatedconstant.SEXTANT]
    const _3x2: NcBlitter = constants::NCBLIT_3x2;,
    /// [`NcBlitter`] mode using: four vertical levels.
    /// █▆▄▂
    const _4x1: NcBlitter = constants::NCBLIT_4x1;,
    /// [`NcBlitter`] mode using: eight vertical levels.
    /// █▇▆▅▄▃▂▁
    const _8x1: NcBlitter = constants::NCBLIT_8x1;,
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
    },
    /// Returns the name of the blitter.
    ///
    /// *(No equivalent C style function)*
    fn name(self) -> String
    where
        Self: Sized,
        u32: From<Self>,
    {
        #[allow(clippy::useless_conversion)] // TODO: send clippy issue?
        crate::Nc::str_blitter(self.into())
    }
];
