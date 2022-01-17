//! `ncchannel*_*` reimplemented functions.

use crate::{
    c_api::{self, NcResult_i32},
    NcAlpha, NcChannel, NcChannels, NcComponent, NcPaletteIndex, NcRgb,
};

// Alpha -----------------------------------------------------------------------

/// Gets the [`NcAlpha`] from an [`NcChannel`].
///
/// It is not shifted down, and can be directly compared to `NCALPHA_*` values.
///
/// *Method: NcChannel.[alpha()][NcChannel#method.alpha]*
#[inline]
pub fn ncchannel_alpha(channel: NcChannel) -> NcAlpha {
    (channel & c_api::NC_BG_ALPHA_MASK).into()
}

/// Sets the [`NcAlpha`] of an [`NcChannel`].
///
/// Background channels must not be set to `NCALPHA_HIGHCONTRAST`.
///
/// It is an error if alpha contains any bits other than `NCALPHA_*`.
///
/// *Method: NcChannel.[set_alpha()][NcChannel#method.set_alpha]*
#[inline]
pub fn ncchannel_set_alpha(channel: &mut NcChannel, alpha: NcAlpha) -> NcResult_i32 {
    let alpha_u32 = c_api::NcAlpha_u32::from(alpha);

    if (alpha_u32 & !c_api::NC_BG_ALPHA_MASK) != 0 {
        return c_api::NCRESULT_ERR;
    }
    *channel = alpha_u32 | (*channel & !c_api::NC_BG_ALPHA_MASK);
    if alpha != NcAlpha::Opaque {
        *channel |= c_api::NC_BGDEFAULT_MASK;
    }
    c_api::NCRESULT_OK
}

/// Gets the foreground [`NcAlpha`] from an [`NcChannels`], shifted to LSBs.
///
/// *Method: NcChannels.[fg_alpha()][NcChannels#method.fg_alpha]*
#[inline]
pub fn ncchannels_fg_alpha(channels: NcChannels) -> NcAlpha {
    ncchannel_alpha(ncchannels_fchannel(channels))
}

/// Gets the background [`NcAlpha`] from an [`NcChannels`], shifted to LSBs.
///
/// *Method: NcChannels.[bg_alpha()][NcChannels#method.bg_alpha]*
#[inline]
pub fn ncchannels_bg_alpha(channels: NcChannels) -> NcAlpha {
    ncchannel_alpha(ncchannels_bchannel(channels))
}

/// Sets the [`NcAlpha`] of the foreground [`NcChannel`] of an [`NcChannels`].
///
/// *Method: NcChannels.[set_fg_alpha()][NcChannels#method.set_fg_alpha]*
#[inline]
pub fn ncchannels_set_fg_alpha(channels: &mut NcChannels, alpha: NcAlpha) -> NcResult_i32 {
    let mut channel = ncchannels_fchannel(*channels);
    if ncchannel_set_alpha(&mut channel, alpha) < 0 {
        return c_api::NCRESULT_ERR;
    }
    *channels = (channel as NcChannels) << 32 | *channels & 0xffffffff_u64;
    c_api::NCRESULT_OK
}

/// Sets the [`NcAlpha`] of the background [`NcChannel`] of an [`NcChannels`].
///
/// *Method: NcChannels.[set_bg_alpha()][NcChannels#method.set_bg_alpha]*
#[inline]
pub fn ncchannels_set_bg_alpha(channels: &mut NcChannels, alpha: NcAlpha) -> NcResult_i32 {
    if alpha == NcAlpha::HighContrast {
        return c_api::NCRESULT_ERR;
    }
    let mut channel = ncchannels_bchannel(*channels);
    if ncchannel_set_alpha(&mut channel, alpha) < 0 {
        return c_api::NCRESULT_ERR;
    }
    ncchannels_set_bchannel(channels, channel);
    c_api::NCRESULT_OK
}

// Channels --------------------------------------------------------------------

/// Extracts the background [`NcChannel`] from a [`NcChannels`].
///
/// *Method: NcChannels.[bchannel()][NcChannels#method.bchannel]*
#[inline]
pub const fn ncchannels_bchannel(channels: NcChannels) -> NcChannel {
    (channels & 0xffffffff_u64) as NcChannel
}

/// Extracts the foreground [`NcChannel`] from an [`NcChannels`].
///
/// *Method: NcChannels.[fchannel()][NcChannels#method.fchannel]*
#[inline]
pub const fn ncchannels_fchannel(channels: NcChannels) -> NcChannel {
    ncchannels_bchannel(channels >> 32)
}

/// Sets the background [`NcChannel`] of an [`NcChannels`].
///
/// *Method: NcChannels.[set_bchannel()][NcChannels#method.set_bchannel]*
#[inline]
pub fn ncchannels_set_bchannel(channels: &mut NcChannels, bchannel: NcChannel) -> NcChannels {
    *channels = (*channels & 0xffffffff00000000_u64) | bchannel as NcChannels;
    *channels
}

/// Sets the foreground [`NcChannel`] of an [`NcChannels`].
///
/// *Method: NcChannels.[set_fchannel()][NcChannels#method.set_fchannel]*
#[inline]
pub fn ncchannels_set_fchannel(channels: &mut NcChannels, fchannel: NcChannel) -> NcChannels {
    *channels = (*channels & 0xffffffff_u64) | (fchannel as NcChannels) << 32;
    *channels
}

/// Combines two [`NcChannel`]s into an [`NcChannels`].
///
/// *Method: NcChannels.[combine()][NcChannels#method.combine]*
#[inline]
pub fn ncchannels_combine(fchannel: NcChannel, bchannel: NcChannel) -> NcChannels {
    let mut channels: NcChannels = 0;
    ncchannels_set_fchannel(&mut channels, fchannel);
    ncchannels_set_bchannel(&mut channels, bchannel);
    channels
}

/// Returns the `NcChannels` with the fore- and background's color information
/// swapped, but without touching housekeeping bits.
///
/// Alpha is retained unless it would lead to an illegal state: `HIGHCONTRAST`,
/// `TRANSPARENT` and `BLEND` are taken to `OPAQUE` unless the new value is RGB.
///
/// *Method: NcChannels.[reverse()][NcChannels#method.reverse]*
#[inline]
pub fn ncchannels_reverse(channels: NcChannels) -> NcChannels {
    let raw = ((ncchannels_bchannel(channels) as NcChannels) << 32)
        + ncchannels_fchannel(channels) as NcChannels;
    let statemask = ((c_api::NC_NOBACKGROUND_MASK | c_api::NC_BG_ALPHA_MASK as NcChannels) << 32)
        | c_api::NC_NOBACKGROUND_MASK
        | c_api::NC_BG_ALPHA_MASK as NcChannels;
    let mut ret = (raw as NcChannels) & !statemask;
    ret |= channels & statemask;
    if ncchannels_bg_alpha(ret) != NcAlpha::Opaque && !ncchannels_bg_rgb_p(ret) {
        ncchannels_set_bg_alpha(&mut ret, NcAlpha::Opaque);
    }
    if ncchannels_fg_alpha(ret) != NcAlpha::Opaque && !ncchannels_fg_rgb_p(ret) {
        ncchannels_set_fg_alpha(&mut ret, NcAlpha::Opaque);
    }
    ret
}

// NcComponent ---------------------------------------------------------------------

/// Gets the red [`NcComponent`] from an [`NcChannel`].
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[r()][NcChannel#method.r]*
#[inline]
pub const fn ncchannel_r(channel: NcChannel) -> NcComponent {
    ((channel & 0xff0000) >> 16) as NcComponent
}

/// Gets the green [`NcComponent`] from an [`NcChannel`].
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[g()][NcChannel#method.g]*
#[inline]
pub const fn ncchannel_g(channel: NcChannel) -> NcComponent {
    ((channel & 0x00ff00) >> 8) as NcComponent
}

/// Gets the blue [`NcComponent`] from an [`NcChannel`].
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[b()][NcChannel#method.b]*
#[inline]
pub const fn ncchannel_b(channel: NcChannel) -> NcComponent {
    (channel & 0x0000ff) as NcComponent
}

/// Sets the red [`NcComponent`] of an [`NcChannel`], and returns it.
///
/// *Method: NcChannel.[set_r()][NcChannel#method.set_r]*
//
// Not in the C API.
#[inline]
pub fn ncchannel_set_r(channel: &mut NcChannel, r: NcComponent) -> NcChannel {
    *channel = (r as NcChannel) << 16 | (*channel & 0xff00) | (*channel & 0xff);
    *channel
}

/// Sets the green [`NcComponent`] of an [`NcChannel`], and returns it.
///
/// *Method: NcChannel.[set_g()][NcChannel#method.set_g]*
//
// Not in the C API.
#[inline]
pub fn ncchannel_set_g(channel: &mut NcChannel, g: NcComponent) -> NcChannel {
    *channel = (*channel & 0xff0000) | (g as NcChannel) << 8 | (*channel & 0xff);
    *channel
}

/// Sets the blue [`NcComponent`] of an [`NcChannel`], and returns it.
///
/// *Method: NcChannel.[set_b()][NcChannel#method.set_b]*
//
// Not in the C API.
#[inline]
pub fn ncchannel_set_b(channel: &mut NcChannel, b: NcComponent) -> NcChannel {
    *channel = (*channel & 0xff0000) | (*channel & 0xff00) | (b as NcChannel);
    *channel
}

/// Gets the three RGB [`NcComponent`]s from an [`NcChannel`], and returns it.
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[rgb8()][NcChannel#method.rgb8]*
#[inline]
pub fn ncchannel_rgb8(
    channel: NcChannel,
    r: &mut NcComponent,
    g: &mut NcComponent,
    b: &mut NcComponent,
) -> NcChannel {
    *r = ncchannel_r(channel);
    *g = ncchannel_g(channel);
    *b = ncchannel_b(channel);
    channel
}

/// Sets the three RGB [`NcComponent`]s an [`NcChannel`], and marks it as NOT using the
/// "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannel.[set_rgb8()][NcChannel#method.set_rgb8]*
#[inline]
pub fn ncchannel_set_rgb8(channel: &mut NcChannel, r: NcComponent, g: NcComponent, b: NcComponent) {
    let rgb: NcRgb = (r as NcChannel) << 16 | (g as NcChannel) << 8 | (b as NcChannel);
    *channel = (*channel & !(c_api::NC_BG_RGB_MASK | c_api::NC_BG_PALETTE))
        | c_api::NC_BGDEFAULT_MASK
        | rgb;
}

/// Gets the three foreground RGB [`NcComponent`]s from an [`NcChannels`], and
/// returns the foreground [`NcChannel`] (which can have some extra bits set).
///
/// *Method: NcChannels.[fg_rgb8()][NcChannels#method.fg_rgb8]*
#[inline]
pub fn ncchannels_fg_rgb8(
    channels: NcChannels,
    r: &mut NcComponent,
    g: &mut NcComponent,
    b: &mut NcComponent,
) -> NcChannel {
    ncchannel_rgb8(ncchannels_fchannel(channels), r, g, b)
}

/// Gets the three background RGB [`NcComponent`]s from an [`NcChannels`], and
/// returns the background [`NcChannel`] (which can have some extra bits set).
///
/// *Method: NcChannels.[bg_rgb8()][NcChannels#method.bg_rgb8]*
#[inline]
pub fn ncchannels_bg_rgb8(
    channels: NcChannels,
    r: &mut NcComponent,
    g: &mut NcComponent,
    b: &mut NcComponent,
) -> NcChannel {
    ncchannel_rgb8(ncchannels_bchannel(channels), r, g, b)
}

/// Sets the three foreground RGB [`NcComponent`]s of an [`NcChannels`], and
/// marks it as NOT using the "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one returns the new `NcChannels`.
///
/// *Method: NcChannels.[set_fg_rgb8()][NcChannels#method.set_fg_rgb8]*
#[inline]
pub fn ncchannels_set_fg_rgb8(
    channels: &mut NcChannels,
    r: NcComponent,
    g: NcComponent,
    b: NcComponent,
) -> NcChannels {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_rgb8(&mut channel, r, g, b);
    *channels = (channel as NcChannels) << 32 | *channels & 0xffffffff;
    *channels
}

/// Sets the three background RGB [`NcComponent`]s of an [`NcChannels`], and
/// marks it as NOT using the "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one returns the new `NcChannels`.
///
/// *Method: NcChannels.[set_bg_rgb8()][NcChannels#method.set_bg_rgb8]*
#[inline]
pub fn ncchannels_set_bg_rgb8(
    channels: &mut NcChannels,
    r: NcComponent,
    g: NcComponent,
    b: NcComponent,
) -> NcChannels {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_rgb8(&mut channel, r, g, b);
    ncchannels_set_bchannel(channels, channel);
    *channels
}

// NcRgb -----------------------------------------------------------------------

/// Gets the foreground [`NcRgb`] from an [`NcChannels`], shifted to LSBs.
///
/// *Method: NcChannels.[fg_rgb()][NcChannels#method.fg_rgb]*
#[inline]
pub const fn ncchannels_fg_rgb(channels: NcChannels) -> NcRgb {
    ncchannels_fchannel(channels) & c_api::NC_BG_RGB_MASK
}

/// Gets the background [`NcRgb`] from an [`NcChannels`], shifted to LSBs.
///
/// *Method: NcChannels.[bg_rgb()][NcChannels#method.bg_rgb]*
#[inline]
pub const fn ncchannels_bg_rgb(channels: NcChannels) -> NcRgb {
    ncchannels_bchannel(channels) & c_api::NC_BG_RGB_MASK
}

/// Returns true if the foreground channel is set to RGB color.
///
/// *Method: NcChannels.[fg_rgb_p()][NcChannels#method.fg_rgb_p]*
#[inline]
pub const fn ncchannels_fg_rgb_p(channels: NcChannels) -> bool {
    ncchannel_rgb_p(ncchannels_fchannel(channels))
}

/// Returns true if the background channel is set to RGB color.
///
/// *Method: NcChannels.[bg_rgb_p()][NcChannels#method.bg_rgb_p]*
#[inline]
pub const fn ncchannels_bg_rgb_p(channels: NcChannels) -> bool {
    ncchannel_rgb_p(ncchannels_bchannel(channels))
}

/// Gets the [`NcRgb`] of an [`NcChannel`].
///
/// This function basically removes the 4th byte of the NcChannel.
///
/// *Method: NcChannel.[rgb()][NcChannel#method.rgb]*
//
// Not in the C API
#[inline]
pub const fn ncchannel_rgb(channel: NcChannel) -> NcRgb {
    channel & c_api::NC_BG_RGB_MASK
}

/// Returns true if this `NcChannel` is using RGB color.
///
/// *Method: NcChannel.[rgb_p()][NcChannel#method.rgb_p]*
#[inline]
pub const fn ncchannel_rgb_p(channel: NcChannel) -> bool {
    // bitwise or is intentional (allows compiler more freedom)
    !(ncchannel_default_p(channel) | ncchannel_palindex_p(channel))
}

/// Sets the [`NcRgb`] of an [`NcChannel`], and marks it as NOT using the
/// "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannel.[set()][NcChannel#method.set]*
#[inline]
pub fn ncchannel_set(channel: &mut NcChannel, rgb: NcRgb) {
    *channel = (*channel & !(c_api::NC_BG_RGB_MASK | c_api::NC_BG_PALETTE))
        | c_api::NC_BGDEFAULT_MASK
        | (rgb & 0x00ffffff);
}

/// Sets the foreground [`NcRgb`] of an [`NcChannels`], and marks it as NOT using
/// the "default color", retaining the other bits unchanged.
///
/// *Method: NcChannels.[set_fg_rgb()][NcChannels#method.set_fg_rgb]*
#[inline]
pub fn ncchannels_set_fg_rgb(channels: &mut NcChannels, rgb: NcRgb) {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set(&mut channel, rgb);
    *channels = (channel as NcChannels) << 32 | *channels & 0xffffffff;
}

/// Sets the foreground [`NcRgb`] of an [`NcChannels`], and marks it as NOT using
/// the "default color", retaining the other bits unchanged.
///
/// *Method: NcChannels.[set_bg_rgb()][NcChannels#method.set_bg_rgb]*
#[inline]
pub fn ncchannels_set_bg_rgb(channels: &mut NcChannels, rgb: NcRgb) {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set(&mut channel, rgb);
    ncchannels_set_bchannel(channels, channel);
}

// Default ---------------------------------------------------------------------

/// Is this [`NcChannel`] using the "default color" rather than RGB/palette-indexed?
///
/// *Method: NcChannel.[default_p()][NcChannel#method.default_p]*
#[inline]
pub const fn ncchannel_default_p(channel: NcChannel) -> bool {
    (channel & c_api::NC_BGDEFAULT_MASK) == 0
}

/// Marks an [`NcChannel`] as using its "default color". Sets alpha as `OPAQUE`.
///
/// *Method: NcChannel.[set_default()][NcChannel#method.set_default]*
#[inline]
pub fn ncchannel_set_default(channel: &mut NcChannel) -> NcChannel {
    *channel &= !c_api::NC_BGDEFAULT_MASK; // turn off not-default bit
    ncchannel_set_alpha(channel, NcAlpha::Opaque);
    *channel
}

/// Marks an [`NcChannel`] as NOT using its "default color",
/// retaining the other bits unchanged.
///
/// *Method: NcChannel.[set_not_default()][NcChannel#method.set_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannel_set_not_default(channel: &mut NcChannel) -> NcChannel {
    *channel |= c_api::NC_BGDEFAULT_MASK;
    *channel
}

/// Is the foreground of an [`NcChannels`] using the "default foreground color"?
///
/// *Method: NcChannels.[fg_default_p()][NcChannels#method.fg_default_p]*
#[inline]
pub const fn ncchannels_fg_default_p(channels: NcChannels) -> bool {
    ncchannel_default_p(ncchannels_fchannel(channels))
}

/// Is the background using the "default background color"?
///
/// The "default background color" must generally be used to take advantage of
/// terminal-effected transparency.
///
/// *Method: NcChannels.[bg_default_p()][NcChannels#method.bg_default_p]*
#[inline]
pub const fn ncchannels_bg_default_p(channels: NcChannels) -> bool {
    ncchannel_default_p(ncchannels_bchannel(channels))
}

/// Marks the foreground of an [`NcChannels`] as using its "default color",
/// which also marks it opaque, and returns the new [`NcChannels`].
///
/// *Method: NcChannels.[set_fg_default()][NcChannels#method.set_fg_default]*
#[inline]
pub fn ncchannels_set_fg_default(channels: &mut NcChannels) -> NcChannels {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_default(&mut channel);
    *channels = (channel as NcChannels) << 32 | *channels & 0xffffffff;
    *channels
}

/// Marks the foreground of an [`NcChannels`] as NOT using its "default color",
/// retaining the other bits unchanged, and returns the new [`NcChannels`].
///
/// *Method: NcChannels.[set_fg_not_default()][NcChannels#method.set_fg_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_fg_not_default(channels: &mut NcChannels) -> NcChannels {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_not_default(&mut channel);
    *channels = (channel as NcChannels) << 32 | *channels & 0xffffffff;
    *channels
}

/// Marks the background of an [`NcChannels`] as using its "default color",
/// which also marks it opaque, and returns the new [`NcChannels`].
///
/// *Method: NcChannels.[set_bg_default()][NcChannels#method.set_bg_default]*
#[inline]
pub fn ncchannels_set_bg_default(channels: &mut NcChannels) -> NcChannels {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_default(&mut channel);
    ncchannels_set_bchannel(channels, channel);
    *channels
}

/// Marks the background of an [`NcChannels`] as NOT using its "default color",
/// retaining the other bits unchanged, and returns the new [`NcChannels`].
///
/// *Method: NcChannels.[set_bg_not_default()][NcChannels#method.set_bg_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_bg_not_default(channels: &mut NcChannels) -> NcChannels {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_not_default(&mut channel);
    ncchannels_set_bchannel(channels, channel);
    *channels
}

/// Marks both the foreground and background of an [`NcChannels`] as using their
/// "default color", which also marks them opaque, and returns the new [`NcChannels`].
///
/// *Method: NcChannels.[set_default()][NcChannels#method.set_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_default(channels: &mut NcChannels) -> NcChannels {
    ncchannels_set_bg_default(&mut ncchannels_set_fg_default(channels))
}

/// Marks both the foreground and background of an [`NcChannels`] as NOT using their
/// "default color", retaining the other bits unchanged, and returns the new [`NcChannels`].
///
/// *Method: NcChannels.[set_not_default()][NcChannels#method.set_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_not_default(channels: &mut NcChannels) -> NcChannels {
    ncchannels_set_bg_not_default(&mut ncchannels_set_fg_not_default(channels))
}

// Palette ---------------------------------------------------------------------

/// Extracts the [`NcPaletteIndex`] from the [`NcChannel`].
///
/// The channel must be palette-indexed, or the return value is meaningless.
/// Verify palette indexing with [`ncchannel_palindex_p`].
///
/// *Method: NcChannel.[palindex()][NcChannel#method.palindex]*
pub const fn ncchannel_palindex(channel: NcChannel) -> NcPaletteIndex {
    (channel & 0xFF) as NcPaletteIndex
}

/// Sets the [`NcPaletteIndex`] of the [`NcChannel`], and the channel into
/// palette indexed mode.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannel.[set_palindex()][NcChannel#method.set_palindex]*
pub fn ncchannel_set_palindex(channel: &mut NcChannel, index: NcPaletteIndex) {
    ncchannel_set_alpha(channel, NcAlpha::Opaque);
    *channel &= 0xFF000000;
    *channel |= c_api::NC_BGDEFAULT_MASK | c_api::NC_BG_PALETTE | index as NcChannel;
}

/// Is this [`NcChannel`] using palette-indexed color rather than RGB?
///
/// *Method: NcChannel.[palindex_p()][NcChannel#method.palindex_p]*
#[inline]
pub const fn ncchannel_palindex_p(channel: NcChannel) -> bool {
    !(ncchannel_default_p(channel) && (channel & c_api::NC_BG_PALETTE) == 0)
}

/// Extracts the [`NcPaletteIndex`] from the foreground [`NcChannel`].
///
/// *Method: NcChannels.[fg_palindex()][NcChannels#method.fg_palindex]*
#[inline]
pub const fn ncchannels_fg_palindex(channels: NcChannels) -> NcPaletteIndex {
    ncchannel_palindex(ncchannels_fchannel(channels))
}

/// Extracts the [`NcPaletteIndex`] from the background [`NcChannel`].
///
/// *Method: NcChannels.[bg_palindex()][NcChannels#method.bg_palindex]*
#[inline]
pub const fn ncchannels_bg_palindex(channels: NcChannels) -> NcPaletteIndex {
    ncchannel_palindex(ncchannels_bchannel(channels))
}

/// Is the foreground of an [`NcChannels`] using an [indexed][`NcPaletteIndex`]
/// [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcChannels.[fg_palindex_p()][NcChannels#method.fg_palindex_p]*
#[inline]
pub const fn ncchannels_fg_palindex_p(channels: NcChannels) -> bool {
    ncchannel_palindex_p(ncchannels_fchannel(channels))
}

/// Is the background of an [`NcChannels`] using an [indexed][`NcPaletteIndex`]
/// [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcChannels.[bg_palindex_p()][NcChannels#method.bg_palindex_p]*
#[inline]
pub const fn ncchannels_bg_palindex_p(channels: NcChannels) -> bool {
    ncchannel_palindex_p(ncchannels_bchannel(channels))
}

/// Sets the foreground of an [`NcChannels`] as using an
/// [*indexed*][`NcPaletteIndex`] [`NcPalette`][crate::NcPalette] color.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannels.[set_fg_palindex()][NcChannels#method.set_fg_palindex]*
#[inline]
#[allow(clippy::unnecessary_cast)]
pub fn ncchannels_set_fg_palindex(channels: &mut NcChannels, index: NcPaletteIndex) {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_palindex(&mut channel, index);
    *channels = (channel as NcChannels) << 32 | (*channels & 0xffffffff as NcChannels)
}

/// Sets the background of an [`NcChannels`] as using an
/// [*indexed*][`NcPaletteIndex`] [`NcPalette`][crate::NcPalette] color.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannels.[set_bg_palindex()][NcChannels#method.set_bg_palindex]*
#[inline]
pub fn ncchannels_set_bg_palindex(channels: &mut NcChannels, index: NcPaletteIndex) {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_palindex(&mut channel, index);
    ncchannels_set_bchannel(channels, channel);
}
