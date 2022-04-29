//!

use crate::{
    c_api::{self, NcChannels_u64},
    error, NcAlpha, NcChannel, NcPaletteIndex, NcResult, NcRgb,
};

/// 64 bits containing a foreground and background [`NcChannel`]
///
/// At render time, both 24-bit [`NcRgb`] values are quantized down to terminal
/// capabilities, if necessary. There's a clear path to 10-bit support should
/// we one day need it.
///
/// ## Default Color
///
/// The "default color" is best explained by [color(3NCURSES)][0] and
/// [default_colors(3NCURSES)][1]. Ours is the same concept.
///
/// [0]: https://manpages.debian.org/stretch/ncurses-doc/color.3ncurses.en.html
/// [1]: https://manpages.debian.org/stretch/ncurses-doc/default_colors.3ncurses.en.html
///
/// **Until the "not default color" bit is set, any color you load will be ignored.**
///
/// ## Diagram
///
/// ```txt
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB║~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ↑↑↑↑↑↑↑↑↑↑↑↑ foreground ↑↑↑↑↑↑↑↑↑↑↑║↑↑↑↑↑↑↑↑↑↑↑↑ background ↑↑↑↑↑↑↑↑↑↑↑
/// ```
///
/// Detailed info (specially on the context-dependent bits on each
/// [`NcChannel`]'s 4th byte):
///
/// ```txt
///                             ~foreground channel~
/// reserved, must be 0                                  ↓bits view↓               ↓hex mask↓
/// 0·······|········|········|········║········|········|········|········  =  8·······|········
///
/// NcChannels::FG_DEFAULT_MASK: foreground is NOT "default color"
/// ·1······|········|········|········║········|········|········|········  =  4·······|········
///
/// NcChannels::FG_ALPHA_MASK: foreground alpha (2bits)
/// ··11····|········|········|········║········|········|········|········  =  3·······|········
///
/// NcChannels::FG_PALETTE: foreground uses palette index
/// ····1···|········|········|········║········|········|········|········  =  ·8······|········
///
/// NcChannels::NOBACKGROUND_MASK: glyph is entirely foreground
/// ·····1··|········|········|········║········|········|········|········  =  ·4······|········
///
/// reserved, must be 0
/// ······00|········|········|········║········|········|········|········  =  ·3······|········
///
/// NcChannels::FG_RGB_MASK: foreground in 3x8 RGB (rrggbb)
/// ········|11111111|11111111|11111111║········|········|········|········  =  ··FFFFFF|········
/// ```
///
/// ```txt
///                             ~background channel~
/// reserved, must be 0                                  ↓bits view↓               ↓hex mask↓
/// ········|········|········|········║0·······|········|········|········  =  ········|8·······
///
/// NcChannels::BGDEFAULT_MASK: background is NOT "default color"
/// ········|········|········|········║·1······|········|········|········  =  ········|4·······
///
/// NcChannels::BG_ALPHA_MASK: background alpha (2 bits)
/// ········|········|········|········║··11····|········|········|········  =  ········|3·······
///
/// NcChannels::BG_PALETTE: background uses palette index
/// ········|········|········|········║····1···|········|········|········  =  ········|·8······
///
/// reserved, must be 0
/// ········|········|········|········║·····000|········|········|········  =  ········|·7······
///
/// NcChannels::BG_RGB_MASK: background in 3x8 RGB (rrggbb)
/// ········|········|········|········║········|11111111|11111111|11111111  =  ········|··FFFFFF
/// ```
/// `type in C: channels (uint64_t)`
///
/// ## `NcChannels` Mask Flags
///
/// - [`NcChannels::BG_DEFAULT_MASK`][NcChannels#associatedconstant.BGDEFAULT_MASK]
/// - [`NcChannels::BG_ALPHA_MASK`][NcChannels#associatedconstant.BG_ALPHA_MASK]
/// - [`NcChannels::BG_PALETTE`][NcChannels#associatedconstant.BG_PALETTE]
/// - [`NcChannels::BG_RGB_MASK`][NcChannels#associatedconstant.BG_RGB_MASK]
/// - [`NcChannels::FG_DEFAULT_MASK`][NcChannels#associatedconstant.FGDEFAULT_MASK]
/// - [`NcChannels::FG_ALPHA_MASK`][NcChannels#associatedconstant.FG_ALPHA_MASK]
/// - [`NcChannels::FG_PALETTE`][NcChannels#associatedconstant.FG_PALETTE]
/// - [`NcChannels::FG_RGB_MASK`][NcChannels#associatedconstant.FG_RGB_MASK]
///
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NcChannels(pub NcChannels_u64);

mod std_impls {
    use super::{NcChannels, NcChannels_u64};

    impl Default for NcChannels {
        fn default() -> Self {
            Self::with_default()
        }
    }

    crate::from_primitive![NcChannels, NcChannels_u64];
    crate::unit_impl_from![NcChannels, NcChannels_u64];
    crate::unit_impl_fmt![bases+display; NcChannels];

    // Different background and foreground:

    impl From<NcChannels> for [u8; 6] {
        #[inline]
        fn from(rgb: NcChannels) -> Self {
            let fg: (u8, u8, u8) = rgb.fg_rgb().into();
            let bg: (u8, u8, u8) = rgb.bg_rgb().into();
            [fg.0, fg.1, fg.2, bg.0, bg.1, bg.2]
        }
    }
    impl From<[u8; 6]> for NcChannels {
        #[inline]
        fn from(a: [u8; 6]) -> Self {
            Self::from_rgb((a[0], a[1], a[2]), (a[3], a[4], a[5]))
        }
    }

    impl From<NcChannels> for (u8, u8, u8, u8, u8, u8) {
        #[inline]
        fn from(rgb: NcChannels) -> Self {
            let fg: (u8, u8, u8) = rgb.fg_rgb().into();
            let bg: (u8, u8, u8) = rgb.bg_rgb().into();
            (fg.0, fg.1, fg.2, bg.0, bg.1, bg.2)
        }
    }
    impl From<(u8, u8, u8, u8, u8, u8)> for NcChannels {
        fn from(t: (u8, u8, u8, u8, u8, u8)) -> Self {
            Self::from_rgb([t.0, t.1, t.2], [t.3, t.4, t.5])
        }
    }
}

/// # NcChannels constants
impl NcChannels {
    /// If this bit is set, we are *not* using the default background color.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    pub const BG_DEFAULT_MASK: u32 = super::c_api::NC_BGDEFAULT_MASK;

    /// Extract these bits to get the background [`NcAlpha`] mask.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    pub const BG_ALPHA_MASK: u32 = super::c_api::NC_BG_ALPHA_MASK;

    /// If this bit *and*
    /// [`BG_DEFAULT_MASK`][NcChannels#associatedconstant.BG_DEFAULT_MASK]
    /// are set, we're using a palette-indexed background color.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    pub const BG_PALETTE_MASK: u32 = super::c_api::NC_BG_PALETTE;

    /// Extract these bits to get the background [`NcRgb`][crate::NcRgb] value.
    pub const BG_RGB_MASK: u32 = super::c_api::NC_BG_RGB_MASK;

    /// Does this glyph completely obscure the background? If so, there's no need
    /// to emit a background when rasterizing, a small optimization. These are
    /// also used to track regions into which we must not cellblit.
    pub const NOBACKGROUND_MASK: u64 = c_api::NC_NOBACKGROUND_MASK;
}

/// # NcChannels constructors
impl NcChannels {
    /// New `NcChannels`, set to black and NOT using the "default color".
    pub fn new() -> Self {
        Self::combine(NcChannel::new(), NcChannel::new())
    }

    /// New `NcChannels`, set to black and using the "default color".
    pub fn with_default() -> Self {
        Self::combine(NcChannel::with_default(), NcChannel::with_default())
    }

    /// New `NcChannels`, expects two separate [`NcRgb`]s for the foreground
    /// and background channels.
    pub fn from_rgb(fg_rgb: impl Into<NcRgb>, bg_rgb: impl Into<NcRgb>) -> Self {
        Self::combine(NcChannel::from_rgb(fg_rgb), NcChannel::from_rgb(bg_rgb))
    }

    /// New `NcChannels`, expects a single [`NcRgb`] for both foreground
    /// and background channels.
    pub fn from_rgb_both(rgb: impl Into<NcRgb>) -> Self {
        let channel = NcChannel::new().set(rgb.into());
        Self::combine(channel, channel)
    }

    /// New `NcChannels`, expects two separate [`NcRgb`] & [`NcAlpha`] for the
    /// foreground and background channels.
    pub fn from_rgb_alpha(
        fg_rgb: impl Into<NcRgb>,
        fg_alpha: impl Into<NcAlpha>,
        bg_rgb: impl Into<NcRgb>,
        bg_alpha: impl Into<NcAlpha>,
    ) -> Self {
        Self::combine(
            NcChannel::from_rgb(fg_rgb).set_alpha(fg_alpha),
            NcChannel::from_rgb(bg_rgb).set_alpha(bg_alpha),
        )
    }

    /// New `NcChannels`, expects [`NcRgb`] & [`NcAlpha`] for both
    /// channels.
    pub fn from_rgb_alpha_both(rgb: impl Into<NcRgb>, alpha: impl Into<NcAlpha>) -> Self {
        let channel = NcChannel::new().set(rgb.into()).set_alpha(alpha.into());
        Self::combine(channel, channel)
    }

    // Combine & Reverse

    /// Combines two [`NcChannel`]s into an [`NcChannels`].
    ///
    /// *C style function: [channels_combine()][c_api::ncchannels_combine].*
    pub fn combine(fchannel: impl Into<NcChannel>, bchannel: impl Into<NcChannel>) -> Self {
        c_api::ncchannels_combine(fchannel.into().0, bchannel.into().0).into()
    }

    /// Returns the `NcChannels` with the fore- and background's color
    /// information swapped, but without touching housekeeping bits.
    ///
    /// Alpha is retained unless it would lead to an illegal state:
    /// `HIGHCONTRAST`, `TRANSPARENT` and `BLEND` are taken to `OPAQUE`
    /// unless the new value is RGB.
    ///
    /// [`HIGHCONTRAST`][NcAlpha#associatedconstant.HIGHCONTRAST]
    /// [`TRANSPARENT`][NcAlpha#associatedconstant.TRANSPARENT]
    /// [`BLEND`][NcAlpha#associatedconstant.BLEND]
    /// [`OPAQUE`][NcAlpha#associatedconstant.OPAQUE]
    ///
    /// *C style function: [ncchannels_reverse()][c_api::ncchannels_reverse].*
    pub fn reverse(&mut self) -> Self {
        *self = c_api::ncchannels_reverse(self.0).into();
        *self
    }
}

/// # NcChannels methods
impl NcChannels {
    // NcChannel

    /// Gets the foreground alpha and coloring bits as an [`NcChannel`].
    ///
    /// *C style function: [ncchannels_fchannel()][c_api::ncchannels_fchannel].*
    pub fn fchannel(&self) -> NcChannel {
        c_api::ncchannels_fchannel(self.0).into()
    }

    /// Gets the background alpha and coloring bits as an [`NcChannel`].
    ///
    /// *C style function: [ncchannels_bchannel()][c_api::ncchannels_bchannel].*
    pub fn bchannel(&self) -> NcChannel {
        c_api::ncchannels_bchannel(self.0).into()
    }

    /// Sets the foreground alpha and coloring bits from an [`NcChannel`].
    ///
    /// *C style function: [ncchannels_set_fchannel()][c_api::ncchannels_set_fchannel].*
    pub fn set_fchannel(&mut self, fchannel: impl Into<NcChannel>) -> Self {
        c_api::ncchannels_set_fchannel(&mut self.0, fchannel.into().0).into()
    }

    /// Sets the background alpha and coloring bits from an [`NcChannel`].
    ///
    /// *C style function: [ncchannels_set_bchannel()][c_api::ncchannels_set_bchannel].*
    pub fn set_bchannel(&mut self, bchannel: impl Into<NcChannel>) -> Self {
        c_api::ncchannels_set_bchannel(&mut self.0, bchannel.into().0).into()
    }

    /// Gets the alpha and coloring bits as an [`NcChannels`].
    ///
    /// *C style function: [ncchannels_bchannel()][c_api::ncchannels_bchannel].*
    pub fn channels(&self) -> NcChannels {
        c_api::ncchannels_channels(self.0).into()
    }

    /// Sets the foreground alpha and coloring bits as an [`NcChannels`],
    /// from another [`NcChannels`].
    ///
    /// *C style function: [ncchannels_set_fchannel()][c_api::ncchannels_set_fchannel].*
    pub fn set_channels(&mut self, from_channels: impl Into<NcChannels>) -> Self {
        c_api::ncchannels_set_channels(&mut self.0, from_channels.into().0).into()
    }

    // Alpha

    /// Gets the foreground [`NcAlpha`].
    ///
    /// *C style function: [ncchannels_fg_alpha()][c_api::ncchannels_fg_alpha].*
    pub fn fg_alpha(&self) -> NcAlpha {
        c_api::ncchannels_fg_alpha(self.0).into()
    }

    /// Gets the background [`NcAlpha`].
    ///
    /// *C style function: [ncchannels_bg_alpha()][c_api::ncchannels_bg_alpha].*
    pub fn bg_alpha(&self) -> NcAlpha {
        c_api::ncchannels_bg_alpha(self.0).into()
    }

    /// Sets the foreground [`NcAlpha`].
    ///
    /// *C style function: [ncchannels_set_fg_alpha()][c_api::ncchannels_set_fg_alpha].*
    pub fn set_fg_alpha(&mut self, alpha: impl Into<NcAlpha>) -> NcResult<()> {
        error![c_api::ncchannels_set_fg_alpha(&mut self.0, alpha.into())]
    }

    /// Sets the background [`NcAlpha`].
    ///
    /// *C style function: [ncchannels_set_bg_alpha()][c_api::ncchannels_set_bg_alpha].*
    pub fn set_bg_alpha(&mut self, alpha: impl Into<NcAlpha>) -> NcResult<()> {
        error![c_api::ncchannels_set_bg_alpha(&mut self.0, alpha.into())]
    }

    // NcRgb

    /// Returns true if the foreground channel is set to RGB color.
    ///
    /// *C style function: [ncchannels_fg_rgb_p()][c_api::ncchannels_fg_rgb_p].*
    pub fn fg_rgb_p(&self) -> bool {
        c_api::ncchannels_fg_rgb_p(self.0)
    }

    /// Returns true if the background channel is set to RGB color.
    ///
    /// *C style function: [ncchannels_bg_rgb_p()][c_api::ncchannels_bg_rgb_p].*
    pub fn bg_rgb_p(&self) -> bool {
        c_api::ncchannels_bg_rgb_p(self.0)
    }

    /// Gets the foreground [`NcRgb`].
    ///
    /// *C style function: [ncchannels_fg_rgb()][c_api::ncchannels_fg_rgb].*
    pub fn fg_rgb(&self) -> NcRgb {
        c_api::ncchannels_fg_rgb(self.0).into()
    }

    /// Gets the background [`NcRgb`].
    ///
    /// *C style function: [ncchannels_bg_rgb()][c_api::ncchannels_bg_rgb].*
    pub fn bg_rgb(&self) -> NcRgb {
        c_api::ncchannels_bg_rgb(self.0).into()
    }

    /// Sets the foreground [`NcRgb`].
    ///
    /// *C style function: [channels_set_fg_rgb()][c_api::ncchannels_set_fg_rgb].*
    pub fn set_fg_rgb(&mut self, rgb: impl Into<NcRgb>) -> Self {
        c_api::ncchannels_set_fg_rgb(&mut self.0, rgb.into().0);
        *self
    }

    /// Sets the background [`NcRgb`].
    ///
    /// *C style function: [channels_set_bg_rgb()][c_api::ncchannels_set_bg_rgb].*
    pub fn set_bg_rgb(&mut self, rgb: impl Into<NcRgb>) -> Self {
        c_api::ncchannels_set_bg_rgb(&mut self.0, rgb.into().0);
        *self
    }

    /// Gets the foreground red component.
    ///
    /// *(No equivalent C style function)*
    pub fn fg_r(&self) -> u8 {
        c_api::ncchannel_r(c_api::ncchannels_fchannel(self.0))
    }

    /// Gets the foreground green component.
    ///
    /// *(No equivalent C style function)*
    pub fn fg_g(&self) -> u8 {
        c_api::ncchannel_g(c_api::ncchannels_fchannel(self.0))
    }

    /// Gets the foreground blue component.
    ///
    /// *(No equivalent C style function)*
    pub fn fg_b(&self) -> u8 {
        c_api::ncchannel_b(c_api::ncchannels_fchannel(self.0))
    }

    /// Gets the background red component.
    ///
    /// *(No equivalent C style function)*
    pub fn bg_r(&self) -> u8 {
        c_api::ncchannel_r(c_api::ncchannels_bchannel(self.0))
    }

    /// Gets the background green component.
    ///
    /// *(No equivalent C style function)*
    pub fn bg_g(&self) -> u8 {
        c_api::ncchannel_g(c_api::ncchannels_bchannel(self.0))
    }

    /// Gets the background blue component.
    ///
    /// *(No equivalent C style function)*
    pub fn bg_b(&self) -> u8 {
        c_api::ncchannel_b(c_api::ncchannels_bchannel(self.0))
    }

    /// Sets the foreground red component, and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    pub fn fg_set_r(&mut self, r: impl Into<u8>) -> Self {
        let (_, g, b) = self.bg_rgb().into();
        c_api::ncchannels_set_fg_rgb8(&mut self.0, r.into(), g, b).into()
    }

    /// Sets the foreground green component, and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    pub fn fg_set_g(&mut self, g: impl Into<u8>) -> Self {
        let (r, _, b) = self.bg_rgb().into();
        c_api::ncchannels_set_fg_rgb8(&mut self.0, r, g.into(), b).into()
    }

    /// Sets the foreground blue component, and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    pub fn fg_set_b(&mut self, b: impl Into<u8>) -> Self {
        let (r, g, _) = self.bg_rgb().into();
        c_api::ncchannels_set_fg_rgb8(&mut self.0, r, g, b.into()).into()
    }

    /// Sets the background red component, and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    pub fn bg_set_r(&mut self, r: impl Into<u8>) -> Self {
        let (_, g, b) = self.bg_rgb().into();
        c_api::ncchannels_set_bg_rgb8(&mut self.0, r.into(), g, b).into()
    }

    /// Sets the background green component, and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    pub fn bg_set_g(&mut self, g: impl Into<u8>) -> Self {
        let (r, _, b) = self.bg_rgb().into();
        c_api::ncchannels_set_bg_rgb8(&mut self.0, r, g.into(), b).into()
    }

    /// Sets the background blue component, and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    pub fn bg_set_b(&mut self, b: impl Into<u8>) -> Self {
        let (r, g, _) = self.bg_rgb().into();
        c_api::ncchannels_set_bg_rgb8(&mut self.0, r, g, b.into()).into()
    }

    // default color

    /// Is the background using the "default background color"?
    ///
    /// *C style function: [channels_fg_default_p()][c_api::ncchannels_fg_default_p].*
    pub fn fg_default_p(&self) -> bool {
        c_api::ncchannels_fg_default_p(self.0)
    }

    /// Is the background using the "default background color"?
    ///
    /// The "default background color" must generally be used to take advantage
    /// of terminal-effected transparency.
    ///
    /// *C style function: [channels_bg_default_p()][c_api::ncchannels_bg_default_p].*
    pub fn bg_default_p(&self) -> bool {
        c_api::ncchannels_bg_default_p(self.0)
    }

    /// Marks the foreground as using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_fg_default()][c_api::ncchannels_set_fg_default].*
    pub fn set_fg_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_default(&mut self.0).into()
    }

    /// Marks the background as using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_bg_default()][c_api::ncchannels_set_bg_default].*
    pub fn set_bg_default(&mut self) -> Self {
        c_api::ncchannels_set_bg_default(&mut self.0).into()
    }

    /// Marks the foreground as NOT using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_fg_default()][c_api::ncchannels_set_fg_default].*
    //
    // Not in the C API
    pub fn set_fg_not_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_not_default(&mut self.0).into()
    }

    /// Marks the background as NOT using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_bg_not_default()][c_api::ncchannels_set_bg_not_default].*
    //
    // Not in the C API
    pub fn set_bg_not_default(&mut self) -> Self {
        c_api::ncchannels_set_bg_not_default(&mut self.0).into()
    }

    /// Marks both the foreground and background as using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    //
    // Not in the C API
    pub fn set_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_default(&mut c_api::ncchannels_set_bg_default(&mut self.0)).into()
    }

    /// Marks both the foreground and background as NOT using its "default color",
    /// and returns the new [`NcChannels`].
    ///
    //
    // Not in the C API
    pub fn set_not_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_not_default(&mut c_api::ncchannels_set_bg_not_default(&mut self.0))
            .into()
    }

    // NcPaletteIndex

    /// Gets the [`NcPaletteIndex`] from the foreground [`NcChannel`].
    ///
    /// *C style function: [channels_fg_palindex()][c_api::ncchannels_fg_palindex].*
    pub fn fg_palindex(&self) -> NcPaletteIndex {
        c_api::ncchannels_fg_palindex(self.0)
    }

    /// Gets the [`NcPaletteIndex`] from the background [`NcChannel`].
    ///
    /// *C style function: [channels_bg_palindex()][c_api::ncchannels_bg_palindex].*
    pub fn bg_palindex(&self) -> NcPaletteIndex {
        c_api::ncchannels_bg_palindex(self.0)
    }

    /// Is the foreground of using an [*indexed*][NcPaletteIndex]
    /// [`NcPalette`][crate::NcPalette] color?
    ///
    /// *C style function: [channels_fg_palindex_p()][c_api::ncchannels_fg_palindex_p].*
    pub fn fg_palindex_p(&self) -> bool {
        c_api::ncchannels_fg_palindex_p(self.0)
    }

    /// Is the background of using an [*indexed*][NcPaletteIndex]
    /// [`NcPalette`][crate::NcPalette] color?
    ///
    /// *C style function: [channels_bg_palindex_p()][c_api::ncchannels_bg_palindex_p].*
    pub fn bg_palindex_p(&self) -> bool {
        c_api::ncchannels_bg_palindex_p(self.0)
    }

    /// Sets the foreground of an [`NcChannels`] as using an
    /// [*indexed*][NcPaletteIndex] [`NcPalette`][crate::NcPalette] color.
    ///
    /// *C style function: [channels_set_fg_palindex()][c_api::ncchannels_set_fg_palindex].*
    pub fn set_fg_palindex(&mut self, index: impl Into<NcPaletteIndex>) -> Self {
        c_api::ncchannels_set_fg_palindex(&mut self.0, index.into());
        *self
    }

    /// Sets the background of an [`NcChannels`] as using an
    /// [*indexed*][NcPaletteIndex] [`NcPalette`][crate::NcPalette] color.
    ///
    /// *C style function: [channels_set_bg_palindex()][c_api::ncchannels_set_bg_palindex].*
    pub fn set_bg_palindex(&mut self, index: impl Into<NcPaletteIndex>) -> Self {
        c_api::ncchannels_set_bg_palindex(&mut self.0, index.into());
        *self
    }
}
