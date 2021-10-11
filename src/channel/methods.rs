//! `NcChannel*` methods and associated functions.
#![allow(clippy::unnecessary_cast)]

use crate::{c_api, NcAlpha, NcChannel, NcChannels, NcComponent, NcPaletteIndex, NcRgb};

/// Enables the [`NcChannel`] associated methods and constants.
pub trait NcChannelApi {
    // constants
    const DEFAULT_MASK: u32 = super::constants::NC_BGDEFAULT_MASK;
    const ALPHA_MASK: u32 = super::constants::NC_BG_ALPHA_MASK;
    const PALETTE_MASK: u32 = super::constants::NC_BG_PALETTE;
    const RGB_MASK: u32 = super::constants::NC_BG_RGB_MASK;

    // constructors
    fn new() -> Self;
    fn default() -> Self;
    fn from_rgb(rgb: NcRgb) -> Self;
    fn from_rgb_alpha(rgb: NcRgb, alpha: NcAlpha) -> Self;
    fn from_rgb8(r: NcComponent, g: NcComponent, b: NcComponent) -> Self;
    fn from_rgb8_alpha(r: NcComponent, g: NcComponent, b: NcComponent, alpha: NcAlpha) -> Self;

    // methods
    fn fcombine(&self, bchannel: NcChannel) -> NcChannels;
    fn bcombine(&self, fchannel: NcChannel) -> NcChannels;

    fn alpha(&self) -> NcAlpha;
    fn set_alpha(&mut self, alpha: NcAlpha) -> Self;

    fn rgb(&self) -> NcRgb;
    fn set(&mut self, rgb: NcRgb) -> Self;

    fn rgb8(&self) -> (NcComponent, NcComponent, NcComponent);
    fn set_rgb8(&mut self, r: NcComponent, g: NcComponent, b: NcComponent) -> Self;
    fn r(&self) -> NcComponent;
    fn g(&self) -> NcComponent;
    fn b(&self) -> NcComponent;
    fn set_r(&mut self, r: NcComponent) -> Self;
    fn set_g(&mut self, g: NcComponent) -> Self;
    fn set_b(&mut self, b: NcComponent) -> Self;

    fn default_p(&self) -> bool;
    fn set_default(&mut self) -> Self;
    fn set_not_default(&mut self) -> Self;

    fn palindex_p(&self) -> bool;
}

/// Enables the [`NcChannels`] associated methods and constants.
pub trait NcChannelsApi {
    // constants
    const BG_DEFAULT_MASK: u32 = super::constants::NC_BGDEFAULT_MASK;
    const BG_ALPHA_MASK: u32 = super::constants::NC_BG_ALPHA_MASK;
    const BG_PALETTE_MASK: u32 = super::constants::NC_BG_PALETTE;
    const BG_RGB_MASK: u32 = super::constants::NC_BG_RGB_MASK;
    const FG_DEFAULT_MASK: u64 = super::constants::NC_FGDEFAULT_MASK;
    const FG_ALPHA_MASK: u64 = super::constants::NC_FG_ALPHA_MASK;
    const FG_PALETTE_MASK: u64 = super::constants::NC_FG_PALETTE;
    const FG_RGB_MASK: u64 = super::constants::NC_FG_RGB_MASK;
    const NOBACKGROUND_MASK: u64 = super::constants::NC_NOBACKGROUND_MASK;

    // constructors
    fn new() -> Self;
    fn with_default() -> Self;
    fn from_rgb(fg_rgb: NcRgb, bg_rgb: NcRgb) -> Self;
    fn from_rgb_both(rgb: NcRgb) -> Self;
    fn from_rgb_alpha(fg_rgb: NcRgb, fg_alpha: NcAlpha, bg_rgb: NcRgb, bg_alpha: NcAlpha) -> Self;
    fn from_rgb_alpha_both(rgb: NcRgb, alpha: NcAlpha) -> Self;
    fn from_rgb8(
        fg_r: NcComponent,
        fg_g: NcComponent,
        fg_b: NcComponent,
        bg_r: NcComponent,
        bg_g: NcComponent,
        bg_b: NcComponent,
    ) -> Self;
    fn from_rgb8_both(r: NcComponent, g: NcComponent, b: NcComponent) -> Self;
    fn from_rgb8_alpha(
        fg_r: NcComponent,
        fg_g: NcComponent,
        fg_b: NcComponent,
        fg_alpha: NcAlpha,
        bg_r: NcComponent,
        bg_g: NcComponent,
        bg_b: NcComponent,
        bg_alpha: NcAlpha,
    ) -> Self;
    fn from_rgb8_alpha_both(r: NcComponent, g: NcComponent, b: NcComponent, alpha: NcAlpha)
        -> Self;

    // methods
    fn combine(fchannel: NcChannel, bchannel: NcChannel) -> Self;

    fn fchannel(&self) -> NcChannel;
    fn bchannel(&self) -> NcChannel;
    fn set_fchannel(&mut self, fchannel: NcChannel) -> Self;
    fn set_bchannel(&mut self, bchannel: NcChannel) -> Self;

    fn fg_alpha(&self) -> NcAlpha;
    fn bg_alpha(&self) -> NcAlpha;
    fn set_fg_alpha(&mut self, alpha: NcAlpha);
    fn set_bg_alpha(&mut self, alpha: NcAlpha);

    fn fg_rgb(&self) -> NcRgb;
    fn bg_rgb(&self) -> NcRgb;
    fn set_fg_rgb(&mut self, alpha: NcAlpha) -> Self;
    fn set_bg_rgb(&mut self, alpha: NcAlpha) -> Self;

    fn fg_rgb8(&self) -> (NcComponent, NcComponent, NcComponent);
    fn bg_rgb8(&self) -> (NcComponent, NcComponent, NcComponent);
    fn set_fg_rgb8(&mut self, r: NcComponent, g: NcComponent, b: NcComponent) -> Self;
    fn set_bg_rgb8(&mut self, r: NcComponent, g: NcComponent, b: NcComponent) -> Self;
    fn fg_r(&self) -> NcComponent;
    fn fg_g(&self) -> NcComponent;
    fn fg_b(&self) -> NcComponent;
    fn bg_r(&self) -> NcComponent;
    fn bg_g(&self) -> NcComponent;
    fn bg_b(&self) -> NcComponent;
    fn fg_set_r(&mut self, r: NcComponent) -> Self;
    fn fg_set_g(&mut self, g: NcComponent) -> Self;
    fn fg_set_b(&mut self, b: NcComponent) -> Self;
    fn bg_set_r(&mut self, r: NcComponent) -> Self;
    fn bg_set_g(&mut self, g: NcComponent) -> Self;
    fn bg_set_b(&mut self, b: NcComponent) -> Self;

    fn fg_default_p(&self) -> bool;
    fn bg_default_p(&self) -> bool;
    fn set_fg_default(&mut self) -> Self;
    fn set_fg_not_default(&mut self) -> Self;
    fn set_bg_default(&mut self) -> Self;
    fn set_bg_not_default(&mut self) -> Self;
    fn set_default(&mut self) -> Self;
    fn set_not_default(&mut self) -> Self;

    fn fg_palindex_p(&self) -> bool;
    fn bg_palindex_p(&self) -> bool;
    fn set_fg_palindex(&mut self, index: NcPaletteIndex) -> Self;
    fn set_bg_palindex(&mut self, index: NcPaletteIndex) -> Self;
}

// NcChannel -------------------------------------------------------------------

/// # NcChannel Methods
impl NcChannelApi for NcChannel {
    // Constants

    /// If this bit is set, we are *not* using the default color.
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_DEFAULT_MASK`][NcChannels#associatedconstant.BG_DEFAULT_MASK]
    const DEFAULT_MASK: u32 = super::constants::NC_BGDEFAULT_MASK;

    /// Extract these bits to get the (background) [`NcAlpha`] mask.
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_ALPHA_MASK`][NcChannels#associatedconstant.BG_ALPHA_MASK]
    const ALPHA_MASK: u32 = super::constants::NC_BG_ALPHA_MASK;

    /// If this bit *and*
    /// [`DEFAULT_MASK`][NcChannel#associatedconstant.DEFAULT_MASK] are set,
    /// we're using a palette-indexed background color
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_PALETTE_MASK`][NcChannels#associatedconstant.BG_PALETTE_MASK]
    const PALETTE_MASK: u32 = super::constants::NC_BG_PALETTE;

    /// Extract these bits to get the background [`NcRgb`][crate::NcRgb] value.
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_RGB_MASK`][NcChannels#associatedconstant.BG_RGB_MASK]
    const RGB_MASK: u32 = super::constants::NC_BG_RGB_MASK;

    // Constructors

    /// New `NcChannel`, set to black and NOT using the "default color".
    fn new() -> Self {
        0 as NcChannel | c_api::NC_BGDEFAULT_MASK
    }

    /// New `NcChannel`, set to black and using the "default color".
    fn default() -> Self {
        0 as NcChannel
    }

    /// New `NcChannel`, expects [`NcRgb`].
    fn from_rgb(rgb: NcRgb) -> Self {
        Self::new().set(rgb)
    }

    /// New `NcChannel`, expects [`NcRgb`] & [`NcAlpha`].
    fn from_rgb_alpha(rgb: NcRgb, alpha: NcAlpha) -> Self {
        Self::new().set(rgb).set_alpha(alpha)
    }

    /// New `NcChannel`, expects three RGB [`NcComponent`] components.
    fn from_rgb8(r: NcComponent, g: NcComponent, b: NcComponent) -> Self {
        Self::new().set_rgb8(r, g, b)
    }

    /// New `NcChannel`, expects three RGB [`NcComponent`] components & [`NcAlpha`].
    fn from_rgb8_alpha(r: NcComponent, g: NcComponent, b: NcComponent, alpha: NcAlpha) -> Self {
        Self::new().set_rgb8(r, g, b).set_alpha(alpha)
    }

    // Combine

    /// Combines this [`NcChannel`] as foreground, with another as background
    /// into an [`NcChannels`].
    ///
    /// *C style function: [channels_combine()][c_api::ncchannels_combine].*
    //
    // Not in the C API
    fn fcombine(&self, bchannel: NcChannel) -> NcChannels {
        c_api::ncchannels_combine(*self, bchannel)
    }

    /// Combines this [`NcChannel`] as background, with another as foreground
    /// into an [`NcChannels`].
    ///
    /// *C style function: [channels_combine()][c_api::ncchannels_combine].*
    //
    // Not in the C API
    fn bcombine(&self, fchannel: NcChannel) -> NcChannels {
        c_api::ncchannels_combine(fchannel, *self)
    }

    // Alpha

    /// Gets the [`NcAlpha`].
    ///
    /// *C style function: [channel_alpha()][c_api::ncchannel_alpha].*
    fn alpha(&self) -> NcAlpha {
        c_api::ncchannel_alpha(*self)
    }

    /// Sets the [`NcAlpha`].
    ///
    /// *C style function: [channel_set_alpha()][c_api::ncchannel_set_alpha].*
    fn set_alpha(&mut self, alpha: NcAlpha) -> Self {
        c_api::ncchannel_set_alpha(self, alpha);
        *self
    }

    // NcRgb

    /// Gets the [`NcRgb`].
    ///
    /// *C style function: [channel_rgb()][c_api::ncchannel_rgb].*
    //
    // Not in the C API
    fn rgb(&self) -> NcRgb {
        c_api::ncchannel_rgb(*self)
    }

    /// Sets the [`NcRgb`], and marks the NcChannel as NOT using the
    /// "default color", retaining the other bits unchanged.
    ///
    /// *C style function: [channel_set()][c_api::ncchannel_set].*
    fn set(&mut self, rgb: NcRgb) -> Self {
        c_api::ncchannel_set(self, rgb);
        *self
    }

    // NcComponent

    /// Gets the three [`NcComponent`]s.
    ///
    /// *C style function: [channel_rgb8()][c_api::ncchannel_rgb8].*
    fn rgb8(&self) -> (NcComponent, NcComponent, NcComponent) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncchannel_rgb8(*self, &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Sets the three [`NcComponent`]s, and
    /// marks the NcChannel as NOT using the "default color".
    ///
    /// *C style function: [channel_set_rgb8()][c_api::ncchannel_set_rgb8].*
    fn set_rgb8(&mut self, r: NcComponent, g: NcComponent, b: NcComponent) -> Self {
        c_api::ncchannel_set_rgb8(self, r, g, b);
        *self
    }

    /// Gets the red [`NcComponent`].
    ///
    /// *C style function: [channel_r()][c_api::ncchannel_r].*
    fn r(&self) -> NcComponent {
        c_api::ncchannel_r(*self)
    }

    /// Gets the green [`NcComponent`].
    ///
    /// *C style function: [channel_g()][c_api::ncchannel_g].*
    fn g(&self) -> NcComponent {
        c_api::ncchannel_g(*self)
    }

    /// Gets the blue [`NcComponent`].
    ///
    /// *C style function: [channel_b()][c_api::ncchannel_b].*
    fn b(&self) -> NcComponent {
        c_api::ncchannel_b(*self)
    }

    /// Sets the red [`NcComponent`], and returns the new `NcChannel`.
    ///
    /// *C style function: [channel_set_r()][c_api::ncchannel_set_r].*
    //
    // Not in the C API
    fn set_r(&mut self, r: NcComponent) -> Self {
        c_api::ncchannel_set_r(self, r)
    }

    /// Sets the green [`NcComponent`], and returns the new `NcChannel`.
    ///
    /// *C style function: [channel_set_g()][c_api::ncchannel_set_g].*
    //
    // Not in the C API
    fn set_g(&mut self, g: NcComponent) -> Self {
        c_api::ncchannel_set_g(self, g)
    }

    /// Sets the blue [`NcComponent`], and returns the new `NcChannel`.
    ///
    /// *C style function: [channel_set_b()][c_api::ncchannel_set_b].*
    //
    // Not in the C API
    fn set_b(&mut self, b: NcComponent) -> Self {
        c_api::ncchannel_set_b(self, b)
    }

    // default color

    /// Is this `NcChannel` using the "default color" rather than RGB/palette-indexed?
    ///
    /// *C style function: [channel_default_p()][c_api::ncchannel_default_p].*
    fn default_p(&self) -> bool {
        c_api::ncchannel_default_p(*self)
    }

    /// Marks this `NcChannel` as using its "default color",
    /// which also marks it opaque.
    ///
    /// *C style function: [channel_set_default()][c_api::ncchannel_set_default].*
    fn set_default(&mut self) -> Self {
        c_api::ncchannel_set_default(self)
    }

    /// Marks this `NcChannel` as *not* using its "default color".
    ///
    /// The following methods also marks the channel as NOT using the "default color":
    /// - [new()][NcChannel#method.new]
    /// - [set()][NcChannel#method.set]
    /// - [set_rgb8()][NcChannel#method.set_rgb8]
    ///
    /// *C style function: [channel_set_not_default()][c_api::ncchannel_set_not_default].*
    //
    // Not in the C API
    fn set_not_default(&mut self) -> Self {
        c_api::ncchannel_set_not_default(self)
    }

    // NcPaletteIndex

    /// Is this NcChannel using palette-indexed color rather than RGB?
    ///
    /// *C style function: [channel_set_default()][c_api::ncchannel_set_default].*
    fn palindex_p(&self) -> bool {
        c_api::ncchannel_palindex_p(*self)
    }
}

// NcChannels ---------------------------------------------------------------

/// # NcChannels Methods
impl NcChannelsApi for NcChannels {
    // Constants

    /// If this bit is set, we are *not* using the default background color.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const BG_DEFAULT_MASK: u32 = super::constants::NC_BGDEFAULT_MASK;

    /// Extract these bits to get the background [`NcAlpha`] mask.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const BG_ALPHA_MASK: u32 = super::constants::NC_BG_ALPHA_MASK;

    /// If this bit *and*
    /// [`BG_DEFAULT_MASK`][NcChannels#associatedconstant.BG_DEFAULT_MASK]
    /// are set, we're using a palette-indexed background color.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const BG_PALETTE_MASK: u32 = super::constants::NC_BG_PALETTE;

    /// Extract these bits to get the background [`NcRgb`][crate::NcRgb] value.
    const BG_RGB_MASK: u32 = super::constants::NC_BG_RGB_MASK;

    /// If this bit is set, we are *not* using the default foreground color.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const FG_DEFAULT_MASK: u64 = super::constants::NC_FGDEFAULT_MASK;

    /// Extract these bits to get the foreground [`NcAlpha`] mask.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const FG_ALPHA_MASK: u64 = super::constants::NC_FG_ALPHA_MASK;

    /// If this bit *and*
    /// [`FG_DEFAULT_MASK`][NcChannels#associatedconstant.FG_DEFAULT_MASK]
    /// are set, we're using a palette-indexed background color.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const FG_PALETTE_MASK: u64 = super::constants::NC_FG_PALETTE;

    /// Extract these bits to get the foreground [`NcRgb`][crate::NcRgb] value.
    ///
    /// See the detailed diagram at [`NcChannels`][crate::NcChannels]
    const FG_RGB_MASK: u64 = super::constants::NC_FG_RGB_MASK;
    // Constructors

    /// New `NcChannels`, set to black and NOT using the "default color".
    fn new() -> Self {
        Self::combine(
            0 as NcChannel | c_api::NC_BGDEFAULT_MASK,
            0 as NcChannel | c_api::NC_BGDEFAULT_MASK,
        )
    }

    /// New `NcChannels`, set to black and using the "default color".
    fn with_default() -> Self {
        Self::combine(0 as NcChannel, 0 as NcChannel)
    }

    /// New `NcChannels`, expects two separate [`NcRgb`]s for the foreground
    /// and background channels.
    fn from_rgb(fg_rgb: NcRgb, bg_rgb: NcRgb) -> Self {
        Self::combine(NcChannel::from_rgb(fg_rgb), NcChannel::from_rgb(bg_rgb))
    }

    /// New `NcChannels`, expects a single [`NcRgb`] for both foreground
    /// and background channels.
    fn from_rgb_both(rgb: NcRgb) -> Self {
        let channel = NcChannel::new().set(rgb);
        Self::combine(channel, channel)
    }

    /// New `NcChannels`, expects two separate [`NcRgb`] & [`NcAlpha`] for the
    /// foreground and background channels.
    fn from_rgb_alpha(fg_rgb: NcRgb, fg_alpha: NcAlpha, bg_rgb: NcRgb, bg_alpha: NcAlpha) -> Self {
        Self::combine(
            NcChannel::from_rgb(fg_rgb).set_alpha(fg_alpha),
            NcChannel::from_rgb(bg_rgb).set_alpha(bg_alpha),
        )
    }

    /// New `NcChannels`, expects [`NcRgb`] & [`NcAlpha`] for both
    /// channels.
    fn from_rgb_alpha_both(rgb: NcRgb, alpha: NcAlpha) -> Self {
        let channel = NcChannel::new().set(rgb).set_alpha(alpha);
        Self::combine(channel, channel)
    }

    /// New `NcChannels`, expects three RGB [`NcComponent`] components
    /// for each channel.
    fn from_rgb8(
        fg_r: NcComponent,
        fg_g: NcComponent,
        fg_b: NcComponent,
        bg_r: NcComponent,
        bg_g: NcComponent,
        bg_b: NcComponent,
    ) -> Self {
        Self::combine(
            NcChannel::from_rgb8(fg_r, fg_g, fg_b),
            NcChannel::from_rgb8(bg_r, bg_g, bg_b),
        )
    }

    /// New `NcChannels`, expects three RGB [`NcComponent`] components for
    /// both the foreground and background channels.
    fn from_rgb8_both(r: NcComponent, g: NcComponent, b: NcComponent) -> Self {
        let channel = NcChannel::new().set_rgb8(r, g, b);
        Self::combine(channel, channel)
    }

    /// New `NcChannels`, expects three RGB [`NcComponent`]s and
    /// [`NcAlpha`], for both the foreground and background channels.
    fn from_rgb8_alpha(
        fg_r: NcComponent,
        fg_g: NcComponent,
        fg_b: NcComponent,
        fg_alpha: NcAlpha,
        bg_r: NcComponent,
        bg_g: NcComponent,
        bg_b: NcComponent,
        bg_alpha: NcAlpha,
    ) -> Self {
        Self::combine(
            NcChannel::from_rgb8_alpha(fg_r, fg_g, fg_b, fg_alpha),
            NcChannel::from_rgb8_alpha(bg_r, bg_g, bg_b, bg_alpha),
        )
    }

    /// New `NcChannel`, expects three RGB [`NcComponent`]s.
    fn from_rgb8_alpha_both(
        r: NcComponent,
        g: NcComponent,
        b: NcComponent,
        alpha: NcAlpha,
    ) -> Self {
        let channel = NcChannel::new().set_rgb8(r, g, b).set_alpha(alpha);
        Self::combine(channel, channel)
    }

    // Combine

    /// Combines two [`NcChannel`]s into an [`NcChannels`].
    ///
    /// *C style function: [channels_combine()][c_api::ncchannels_combine].*
    fn combine(fchannel: NcChannel, bchannel: NcChannel) -> Self {
        c_api::ncchannels_combine(fchannel, bchannel)
    }

    // NcChannel

    /// Extracts the foreground [`NcChannel`].
    ///
    /// *C style function: [channels_fchannel()][c_api::ncchannels_fchannel].*
    fn fchannel(&self) -> NcChannel {
        c_api::ncchannels_fchannel(*self)
    }

    /// Extracts the background [`NcChannel`].
    ///
    /// *C style function: [channels_bchannel()][c_api::ncchannels_bchannel].*
    fn bchannel(&self) -> NcChannel {
        c_api::ncchannels_bchannel(*self)
    }

    /// Sets the foreground [`NcChannel`].
    ///
    /// *C style function: [channels_set_fchannel()][c_api::ncchannels_set_fchannel].*
    fn set_fchannel(&mut self, fchannel: NcChannel) -> Self {
        c_api::ncchannels_set_fchannel(self, fchannel)
    }

    /// Sets the background [`NcChannel`].
    ///
    /// *C style function: [channels_set_bchannel()][c_api::ncchannels_set_bchannel].*
    fn set_bchannel(&mut self, bchannel: NcChannel) -> Self {
        c_api::ncchannels_set_bchannel(self, bchannel)
    }

    // Alpha

    /// Gets the foreground [`NcAlpha`].
    ///
    /// *C style function: [channels_fg_alpha()][c_api::ncchannels_fg_alpha].*
    fn fg_alpha(&self) -> NcAlpha {
        c_api::ncchannels_fg_alpha(*self)
    }

    /// Gets the background [`NcAlpha`].
    ///
    /// *C style function: [channels_bg_alpha()][c_api::ncchannels_bg_alpha].*
    fn bg_alpha(&self) -> NcAlpha {
        c_api::ncchannels_bg_alpha(*self)
    }

    /// Sets the foreground [`NcAlpha`].
    ///
    /// *C style function: [channels_set_fg_alpha()][c_api::ncchannels_set_fg_alpha].*
    fn set_fg_alpha(&mut self, alpha: NcAlpha) {
        c_api::ncchannels_set_fg_alpha(self, alpha)
    }

    /// Sets the background [`NcAlpha`].
    ///
    /// *C style function: [channels_set_bg_alpha()][c_api::ncchannels_set_bg_alpha].*
    fn set_bg_alpha(&mut self, alpha: NcAlpha) {
        c_api::ncchannels_set_bg_alpha(self, alpha)
    }

    // NcRgb

    /// Gets the foreground [`NcRgb`].
    ///
    /// *C style function: [channels_fg_rgb()][c_api::ncchannels_fg_rgb].*
    fn fg_rgb(&self) -> NcRgb {
        c_api::ncchannels_fg_rgb(*self)
    }

    /// Gets the background [`NcRgb`].
    ///
    /// *C style function: [channels_bg_rgb()][c_api::ncchannels_bg_rgb].*
    fn bg_rgb(&self) -> NcRgb {
        c_api::ncchannels_bg_rgb(*self)
    }

    /// Sets the foreground [`NcRgb`].
    ///
    /// *C style function: [channels_set_fg_rgb()][c_api::ncchannels_set_fg_rgb].*
    fn set_fg_rgb(&mut self, rgb: NcRgb) -> Self {
        c_api::ncchannels_set_fg_rgb(self, rgb);
        *self
    }

    /// Sets the background [`NcRgb`].
    ///
    /// *C style function: [channels_set_bg_rgb()][c_api::ncchannels_set_bg_rgb].*
    fn set_bg_rgb(&mut self, rgb: NcRgb) -> Self {
        c_api::ncchannels_set_bg_rgb(self, rgb);
        *self
    }

    // NcComponent

    /// Gets the three foreground RGB [`NcComponent`]s (r, g, b).
    ///
    /// *C style function: [channels_fg_rgb8()][c_api::ncchannels_fg_rgb8].*
    fn fg_rgb8(&self) -> (NcComponent, NcComponent, NcComponent) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncchannels_fg_rgb8(*self, &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Gets the three background RGB [`NcComponent`]s (r, g, b).
    ///
    /// *C style function: [channels_bg_rgb8()][c_api::ncchannels_bg_rgb8].*
    fn bg_rgb8(&self) -> (NcComponent, NcComponent, NcComponent) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncchannels_bg_rgb8(*self, &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Sets the three foreground RGB [`NcComponent`]s (r, g, b), and
    /// marks the foreground [`NcChannel`] as not using the "default color".
    ///
    /// *C style function: [channels_set_fg_rgb8()][c_api::ncchannels_set_fg_rgb8].*
    fn set_fg_rgb8(&mut self, r: NcComponent, g: NcComponent, b: NcComponent) -> Self {
        c_api::ncchannels_set_fg_rgb8(self, r, g, b)
    }

    /// Sets the three background RGB [`NcComponent`]s (r, g, b), and
    /// marks the background [`NcChannel`] as not using the "default color".
    ///
    /// *C style function: [channels_set_bg_rgb8()][c_api::ncchannels_set_bg_rgb8].*
    fn set_bg_rgb8(&mut self, r: NcComponent, g: NcComponent, b: NcComponent) -> Self {
        c_api::ncchannels_set_bg_rgb8(self, r, g, b)
    }

    /// Gets the foreground red [`NcComponent`].
    ///
    /// *(No equivalent C style function)*
    fn fg_r(&self) -> NcComponent {
        c_api::ncchannel_r(c_api::ncchannels_fchannel(*self))
    }

    /// Gets the foreground green [`NcComponent`].
    ///
    /// *(No equivalent C style function)*
    fn fg_g(&self) -> NcComponent {
        c_api::ncchannel_g(c_api::ncchannels_fchannel(*self))
    }

    /// Gets the foreground blue [`NcComponent`].
    ///
    /// *(No equivalent C style function)*
    fn fg_b(&self) -> NcComponent {
        c_api::ncchannel_b(c_api::ncchannels_fchannel(*self))
    }

    /// Gets the background red [`NcComponent`].
    ///
    /// *(No equivalent C style function)*
    fn bg_r(&self) -> NcComponent {
        c_api::ncchannel_r(c_api::ncchannels_bchannel(*self))
    }

    /// Gets the background green [`NcComponent`].
    ///
    /// *(No equivalent C style function)*
    fn bg_g(&self) -> NcComponent {
        c_api::ncchannel_g(c_api::ncchannels_bchannel(*self))
    }

    /// Gets the background blue [`NcComponent`].
    ///
    /// *(No equivalent C style function)*
    fn bg_b(&self) -> NcComponent {
        c_api::ncchannel_b(c_api::ncchannels_bchannel(*self))
    }

    /// Sets the foreground red [`NcComponent`], and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    fn fg_set_r(&mut self, r: NcComponent) -> Self {
        let (_, g, b) = self.bg_rgb8();
        c_api::ncchannels_set_fg_rgb8(self, r, g, b)
    }

    /// Sets the foreground green [`NcComponent`], and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    fn fg_set_g(&mut self, g: NcComponent) -> Self {
        let (r, _, b) = self.bg_rgb8();
        c_api::ncchannels_set_fg_rgb8(self, r, g, b)
    }

    /// Sets the foreground blue [`NcComponent`], and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    fn fg_set_b(&mut self, b: NcComponent) -> Self {
        let (r, g, _) = self.bg_rgb8();
        c_api::ncchannels_set_fg_rgb8(self, r, g, b)
    }

    /// Sets the background red [`NcComponent`], and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    fn bg_set_r(&mut self, r: NcComponent) -> Self {
        let (_, g, b) = self.bg_rgb8();
        c_api::ncchannels_set_bg_rgb8(self, r, g, b)
    }

    /// Sets the background green [`NcComponent`], and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    fn bg_set_g(&mut self, g: NcComponent) -> Self {
        let (r, _, b) = self.bg_rgb8();
        c_api::ncchannels_set_bg_rgb8(self, r, g, b)
    }

    /// Sets the background blue [`NcComponent`], and returns the new `NcChannels`.
    ///
    /// *(No equivalent C style function)*
    fn bg_set_b(&mut self, b: NcComponent) -> Self {
        let (r, g, _) = self.bg_rgb8();
        c_api::ncchannels_set_bg_rgb8(self, r, g, b)
    }

    // default color

    /// Is the background using the "default background color"?
    ///
    /// *C style function: [channels_fg_default_p()][c_api::ncchannels_fg_default_p].*
    fn fg_default_p(&self) -> bool {
        c_api::ncchannels_fg_default_p(*self)
    }

    /// Is the background using the "default background color"?
    ///
    /// The "default background color" must generally be used to take advantage
    /// of terminal-effected transparency.
    ///
    /// *C style function: [channels_bg_default_p()][c_api::ncchannels_bg_default_p].*
    fn bg_default_p(&self) -> bool {
        c_api::ncchannels_bg_default_p(*self)
    }

    /// Marks the foreground as using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_fg_default()][c_api::ncchannels_set_fg_default].*
    fn set_fg_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_default(self)
    }

    /// Marks the background as using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_bg_default()][c_api::ncchannels_set_bg_default].*
    fn set_bg_default(&mut self) -> Self {
        c_api::ncchannels_set_bg_default(self)
    }

    /// Marks the foreground as NOT using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_fg_default()][c_api::ncchannels_set_fg_default].*
    //
    // Not in the C API
    fn set_fg_not_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_not_default(self)
    }

    /// Marks the background as NOT using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    /// *C style function: [channels_set_bg_not_default()][c_api::ncchannels_set_bg_not_default].*
    //
    // Not in the C API
    fn set_bg_not_default(&mut self) -> Self {
        c_api::ncchannels_set_bg_not_default(self)
    }

    /// Marks both the foreground and background as using its "default color", and
    /// returns the new [`NcChannels`].
    ///
    //
    // Not in the C API
    fn set_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_default(&mut c_api::ncchannels_set_bg_default(self))
    }

    /// Marks both the foreground and background as NOT using its "default color",
    /// and returns the new [`NcChannels`].
    ///
    //
    // Not in the C API
    fn set_not_default(&mut self) -> Self {
        c_api::ncchannels_set_fg_not_default(&mut c_api::ncchannels_set_bg_not_default(self))
    }

    // NcPaletteIndex

    /// Is the foreground of using an [indexed][NcPaletteIndex]
    /// [NcPalette][crate::NcPalette] color?
    ///
    /// *C style function: [channels_fg_palindex_p()][c_api::ncchannels_fg_palindex_p].*
    fn fg_palindex_p(&self) -> bool {
        c_api::ncchannels_fg_palindex_p(*self)
    }

    /// Is the background of using an [indexed][NcPaletteIndex]
    /// [NcPalette][crate::NcPalette] color?
    ///
    /// *C style function: [channels_bg_palindex_p()][c_api::ncchannels_bg_palindex_p].*
    fn bg_palindex_p(&self) -> bool {
        c_api::ncchannels_bg_palindex_p(*self)
    }

    /// Sets the foreground of an [`NcChannels`] as using an
    /// [indexed][NcPaletteIndex] [NcPalette][crate::NcPalette] color.
    ///
    /// *C style function: [channels_set_fg_palindex()][c_api::ncchannels_set_fg_palindex].*
    fn set_fg_palindex(&mut self, index: NcPaletteIndex) -> Self {
        c_api::ncchannels_set_fg_palindex(self, index);
        *self
    }

    /// Sets the background of an [`NcChannels`] as using an
    /// [indexed][NcPaletteIndex] [NcPalette][crate::NcPalette] color.
    ///
    /// *C style function: [channels_set_bg_palindex()][c_api::ncchannels_set_bg_palindex].*
    fn set_bg_palindex(&mut self, index: NcPaletteIndex) -> Self {
        c_api::ncchannels_set_bg_palindex(self, index);
        *self
    }

    /// Does this glyph completely obscure the background? If so, there's no need
    /// to emit a background when rasterizing, a small optimization. These are
    /// also used to track regions into which we must not cellblit.
    const NOBACKGROUND_MASK: u64 = super::constants::NC_NOBACKGROUND_MASK;
}
