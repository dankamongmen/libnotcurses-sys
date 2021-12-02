//! `NcBlitter`

mod methods;
pub use methods::NcBlitterApi;

/// The blitter mode to use for rasterizing an [`NcVisual`][crate::NcVisual]
/// (alias of `u32`).
///
/// We never blit full blocks, but instead spaces (more efficient) with the
/// background set to the desired foreground.
///
/// # Degradation
///
/// There is a mechanism of graceful degradation, that works as follows:
/// - without braille support, [`NcBlitter::BRAILLE`] decays to [`NcBlitter::SEXTANT`].
/// - without bitmap support, [`NcBlitter::PIXEL`] decays to [`NcBlitter::SEXTANT`].
/// - without sextant support, [`NcBlitter::SEXTANT`] decays to [`NcBlitter::QUADRANT`].
/// - without quadrant support, [`NcBlitter::QUADRANT`] decays to [`NcBlitter::HALF`].
/// - the only viable blitters in ASCII are [`NCBlitter::ASCII`] and [`NcBlitter::PIXEL`].
///
/// If you don't want this behaviour you have to set the
/// *[`NcVisualOptions::NODEGRADE`]* flag on [`NcVisualOptions`] or call
/// *[`degrade(false)`]* on [`NcVisualOptionsBuilder`].
///
/// [`NCBlitter::BRAILLE`]: NcBlitter#associatedconstant.BRAILLE
/// [`NCBlitter::PIXEL`]: NcBlitter#associatedconstant.PIXEL
/// [`NCBlitter::ASCII`]: NcBlitter#associatedconstant.ASCII
/// [`NCBlitter::HALF`]: NcBlitter#associatedconstant.HALF
/// [`NCBlitter::QUADRANT`]: NcBlitter#associatedconstant.QUADRANT
/// [`NCBlitter::SEXTANT`]: NcBlitter#associatedconstant.SEXTANT
/// [`NcVisualOptions::NODEGRADE`]: crate::NcVisualOptions#associatedconstant.NODEGRADE
/// [`degrade(false)`]: crate::NcVisualOptionsBuilder#method.degrade
/// [`NcVisualOptionsBuilder`]: crate::NcVisualOptionsBuilder
pub type NcBlitter = u32;

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
