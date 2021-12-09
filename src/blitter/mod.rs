//! `NcBlitter`

// functions already exported by bindgen: 4
// ------------------------------------------
// (#) test:  0
// (W) wrap: 4
// ------------------------------------------
//W+ ncblit_bgrx
//W+ ncblit_rgb_loose
//W+ ncblit_rgb_packed
//W+ ncblit_rgba

use std::fmt;

mod methods;

/// The blitter mode to use for rasterizing an [`NcVisual`][crate::NcVisual].
///
/// We never blit full blocks, but instead spaces (more efficient) with the
/// background set to the desired foreground.
///
/// # Degradation
///
/// There is a mechanism of graceful degradation, that works as follows:
/// - without braille support, [`Braille`] decays to [`Sextant`].
/// - without bitmap support, [`Pixel`] decays to [`Sextant`].
/// - without sextant support, [`Sextant`] decays to [`Quadrant`].
/// - without quadrant support, [`Quadrant`] decays to [`Half`].
/// - the only viable blitters in ASCII are [`Ascii`] and [`Pixel`].
///
/// If you don't want this behaviour you have to set the
/// *[`NcVisualOptions::NODEGRADE`]* flag on [`NcVisualOptions`] or call
/// *[`degrade(false)`]* on [`NcVisualOptionsBuilder`].
///
/// [`Braille`]: NcBlitter::Braille
/// [`Pixel`]: NcBlitter::Pixel
/// [`Ascii`]: NcBlitter::Ascii
/// [`Half`]: NcBlitter::Half
/// [`Quadrant`]: NcBlitter::Quadrant
/// [`Sextant`]: NcBlitter::Sextant
/// [`NcVisualOptions::NODEGRADE`]: crate::NcVisualOptions#associatedconstant.NODEGRADE
/// [`NcVisualOptions`]: crate::NcVisualOptions
/// [`degrade(false)`]: crate::NcVisualOptionsBuilder#method.degrade
/// [`NcVisualOptionsBuilder`]: crate::NcVisualOptionsBuilder
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcBlitter {
    Default = c_api::NCBLIT_DEFAULT,

    /// Blitter mode using only spaces, compatible with ASCII (1x1).
    Ascii = c_api::NCBLIT_1x1,

    /// Blitter mode using halves + `Ascii` (2x1).
    /// ▄▀
    Half = c_api::NCBLIT_2x1,

    /// Blitter mode using quadrants + `Half` (2x2).
    /// ▗▐ ▖▀▟▌▙
    Quadrant = c_api::NCBLIT_2x2,

    /// Blitter mode using sextants + `Quadrant` (3x2).
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    Sextant = c_api::NCBLIT_3x2,

    /// Blitter mode using braille (4x2).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    Braille = c_api::NCBLIT_BRAILLE,

    /// Blitter mode using Pixels/Sixels.
    Pixel = c_api::NCBLIT_PIXEL,

    /// [`NcBlitter`] mode using: four vertical levels (4x1).
    /// █▆▄▂
    _4x1 = c_api::NCBLIT_4x1,

    /// [`NcBlitter`] mode using: eight vertical levels (8x1).
    /// █▇▆▅▄▃▂▁
    _8x1 = c_api::NCBLIT_8x1,
}

/// # Aliases
impl NcBlitter {
    pub const _1x1: NcBlitter = NcBlitter::Ascii;
    pub const _2x1: NcBlitter = NcBlitter::Half;
    pub const _2x2: NcBlitter = NcBlitter::Quadrant;
    pub const _3x2: NcBlitter = NcBlitter::Sextant;
}

impl Default for NcBlitter {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for NcBlitter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NcBlitter::*;
        write!(
            f,
            "{}",
            match self {
                Default => "Default",
                Ascii => "Ascii",
                Half => "Half",
                Quadrant => "Quadrant",
                Sextant => "Sextant",
                Braille => "Braille",
                Pixel => "Pixel",
                _4x1 => "4x1",
                _8x1 => "8x1",
                // _ => "?" // WIP
            }
        )
    }
}

impl From<c_api::NcBlitter_u32> for NcBlitter {
    fn from(blitter: c_api::NcBlitter_u32) -> Self {
        use {c_api::*, NcBlitter::*};
        match blitter {
            NCBLIT_DEFAULT => Default,
            NCBLIT_1x1 => Ascii,
            NCBLIT_2x1 => Half,
            NCBLIT_2x2 => Quadrant,
            NCBLIT_3x2 => Sextant,
            NCBLIT_BRAILLE => Braille,
            NCBLIT_PIXEL => Pixel,
            NCBLIT_4x1 => _4x1,
            NCBLIT_8x1 => _8x1,
            _ => Ascii, // invalid values default to Ascii
        }
    }
}

impl From<NcBlitter> for c_api::NcBlitter_u32 {
    fn from(blitter: NcBlitter) -> Self {
        use {c_api::*, NcBlitter::*};
        match blitter {
            Default => NCBLIT_DEFAULT,
            Ascii => NCBLIT_1x1,
            Half => NCBLIT_2x1,
            Quadrant => NCBLIT_2x2,
            Sextant => NCBLIT_3x2,
            Braille => NCBLIT_BRAILLE,
            Pixel => NCBLIT_PIXEL,
            _4x1 => NCBLIT_4x1,
            _8x1 => NCBLIT_8x1,
        }
    }
}

pub(crate) mod c_api {
    use crate::bindings::ffi;

    /// The blitter mode to use for rasterizing an [`NcVisual`][crate::NcVisual].
    ///
    /// It's recommended to use [`NcBlitter`][crate::NcBlitter] instead.
    ///
    /// # Associated `c_api` constants:
    ///
    /// - [`NCBLIT_DEFAULT`]
    /// - [`NCBLIT_1x1`]
    /// - [`NCBLIT_2x1`]
    /// - [`NCBLIT_2x2`]
    /// - [`NCBLIT_3x2`]
    /// - [`NCBLIT_4x1`]
    /// - [`NCBLIT_8x1`]
    /// - [`NCBLIT_BRAILLE`]
    /// - [`NCBLIT_PIXEL`]
    pub type NcBlitter_u32 = u32;

    /// [`NcBlitter_u32`] mode where the blitter is automatically chosen.
    pub const NCBLIT_DEFAULT: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_DEFAULT;
    /// [`NcBlitter_u32`] mode using: space, compatible with ASCII.
    pub const NCBLIT_1x1: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_1x1;
    /// [`NcBlitter_u32`] mode using: halves + 1x1 (space).
    /// ▄▀
    pub const NCBLIT_2x1: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_2x1;
    /// [`NcBlitter_u32`] mode using: quadrants + 2x1.
    /// ▗▐ ▖▀▟▌▙
    pub const NCBLIT_2x2: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_2x2;
    /// [`NcBlitter_u32`] mode using: sextants.
    /// 🬀🬁🬂🬃🬄🬅🬆🬇🬈🬉🬊🬋🬌🬍🬎🬏🬐🬑🬒🬓🬔🬕🬖🬗🬘🬙🬚🬛🬜🬝🬞🬟🬠🬡🬢🬣🬤🬥🬦🬧🬨🬩🬪🬫🬬🬭🬮🬯🬰🬱🬲🬳🬴🬵🬶🬷🬸🬹🬺🬻
    pub const NCBLIT_3x2: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_3x2;
    /// [`NcBlitter_u32`] mode using: four vertical levels.
    /// █▆▄▂
    pub const NCBLIT_4x1: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_4x1;
    /// [`NcBlitter_u32`] mode using: eight vertical levels.
    /// █▇▆▅▄▃▂▁
    pub const NCBLIT_8x1: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_8x1;
    /// [`NcBlitter_u32`] mode using: 4 rows, 2 cols (braille).
    /// ⡀⡄⡆⡇⢀⣀⣄⣆⣇⢠⣠⣤⣦⣧⢰⣰⣴⣶⣷⢸⣸⣼⣾⣿
    pub const NCBLIT_BRAILLE: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_BRAILLE;
    /// Sixel/Pixel mode.
    ///
    /// See [Sixel in Wikipedia](https://en.wikipedia.org/wiki/Sixel).
    pub const NCBLIT_PIXEL: NcBlitter_u32 = ffi::ncblitter_e_NCBLIT_PIXEL;
}
