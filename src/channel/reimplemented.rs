//! `ncchannel*_*` reimplemented functions.

use crate::{
    c_api::{self, NcAlpha_u32, NcChannel_u32, NcChannels_u64, NcResult_i32, NcRgb_u32},
    NcPaletteIndex,
};

#[allow(unused_imports)]
use crate::{NcChannel, NcChannels};

// Alpha -----------------------------------------------------------------------

/// Gets the [`NcAlpha_u32`] from an [`NcChannel_u32`].
///
/// It is not shifted down, and can be directly compared to `NCALPHA_*` values.
///
/// *Method: NcChannel.[alpha()][NcChannel#method.alpha]*
#[inline]
pub fn ncchannel_alpha(channel: impl Into<NcChannel_u32>) -> NcAlpha_u32 {
    channel.into() & c_api::NC_BG_ALPHA_MASK
}

/// Sets the [`NcAlpha_u32`] of an [`NcChannel_u32`].
///
/// Background channels must not be set to `NCALPHA_HIGHCONTRAST`.
///
/// It is an error if alpha contains any bits other than `NCALPHA_*`.
///
/// *Method: NcChannel.[set_alpha()][NcChannel#method.set_alpha]*
#[inline]
pub fn ncchannel_set_alpha(
    channel: &mut NcChannel_u32,
    alpha: impl Into<NcAlpha_u32>,
) -> NcResult_i32 {
    let alpha = alpha.into();

    if (alpha & !c_api::NC_BG_ALPHA_MASK) != 0 {
        return c_api::NCRESULT_ERR;
    }
    *channel = alpha | (*channel & !c_api::NC_BG_ALPHA_MASK);
    if alpha != c_api::NCALPHA_OPAQUE {
        *channel |= c_api::NC_BGDEFAULT_MASK;
    }
    c_api::NCRESULT_OK
}

/// Gets the foreground [`NcAlpha_u32`] from an [`NcChannels_u64`], shifted to LSBs.
///
/// *Method: NcChannels.[fg_alpha()][NcChannels#method.fg_alpha]*
#[inline]
pub fn ncchannels_fg_alpha(channels: impl Into<NcChannels_u64>) -> NcAlpha_u32 {
    ncchannel_alpha(ncchannels_fchannel(channels))
}

/// Gets the background [`NcAlpha_u32`] from an [`NcChannels_u64`], shifted to LSBs.
///
/// *Method: NcChannels.[bg_alpha()][NcChannels#method.bg_alpha]*
#[inline]
pub fn ncchannels_bg_alpha(channels: impl Into<NcChannels_u64>) -> NcAlpha_u32 {
    ncchannel_alpha(ncchannels_bchannel(channels))
}

/// Sets the [`NcAlpha_u32`] of the foreground [`NcChannel_u32`] of an [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_fg_alpha()][NcChannels#method.set_fg_alpha]*
#[inline]
pub fn ncchannels_set_fg_alpha(
    channels: &mut NcChannels_u64,
    alpha: impl Into<NcAlpha_u32>,
) -> NcResult_i32 {
    let mut channel = ncchannels_fchannel(*channels);
    if ncchannel_set_alpha(&mut channel, alpha.into()) < 0 {
        return c_api::NCRESULT_ERR;
    }
    *channels = (channel as NcChannels_u64) << 32 | *channels & 0xffffffff_u64;
    c_api::NCRESULT_OK
}

/// Sets the [`NcAlpha_u32`] of the background [`NcChannel_u32`] of an [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_bg_alpha()][NcChannels#method.set_bg_alpha]*
#[inline]
pub fn ncchannels_set_bg_alpha(
    channels: &mut NcChannels_u64,
    alpha: impl Into<NcAlpha_u32>,
) -> NcResult_i32 {
    let alpha = alpha.into();
    if alpha == c_api::NCALPHA_HIGHCONTRAST {
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

/// Gets the background alpha and coloring bits as an [`NcChannel_u32`]
/// from a [`NcChannels_u64`].
///
/// *Method: NcChannels.[bchannel()][NcChannels#method.bchannel]*
#[inline]
pub fn ncchannels_bchannel(channels: impl Into<NcChannels_u64>) -> NcChannel_u32 {
    (channels.into()
        & (c_api::NC_BG_RGB_MASK
            | c_api::NC_BG_PALETTE
            | c_api::NC_BGDEFAULT_MASK
            | c_api::NC_BG_ALPHA_MASK) as NcChannels_u64) as NcChannel_u32
}

/// Gets the foreground alpha and coloring bits as an [`NcChannel_u32`]
/// from an [`NcChannels_u64`].
///
/// *Method: NcChannels.[fchannel()][NcChannels#method.fchannel]*
#[inline]
pub fn ncchannels_fchannel(channels: impl Into<NcChannels_u64>) -> NcChannel_u32 {
    ncchannels_bchannel(channels.into() >> 32)
}

/// Gets the alpha and coloring bits as an [`NcChannels_u64`].
///
/// *Method: NcChannels.[channels()][NcChannels#method.channels]*
#[inline]
pub fn ncchannels_channels(channels: impl Into<NcChannels_u64>) -> NcChannels_u64 {
    let channels = channels.into();
    ncchannels_bchannel(channels) as NcChannels_u64
        | (ncchannels_fchannel(channels) as NcChannels_u64) << 32
}

/// Sets the background alpha and coloring bits of the [`NcChannels_u64`]
/// from an [`NcChannel_u32`].
///
/// *Method: NcChannels.[set_bchannel()][NcChannels#method.set_bchannel]*
#[inline]
pub fn ncchannels_set_bchannel(
    channels: &mut NcChannels_u64,
    bchannel: impl Into<NcChannel_u32>,
) -> NcChannels_u64 {
    // drop the background color and alpha bit
    *channels &= (0xffffffff_u64 << 32) | c_api::NC_NOBACKGROUND_MASK;
    *channels |= bchannel.into() as NcChannels_u64 & !c_api::NC_NOBACKGROUND_MASK;
    *channels
}

/// Sets the foreground alpha and coloring bits of the [`NcChannels_u64`]
/// from an [`NcChannel_u32`].
///
/// *Method: NcChannels.[set_fchannel()][NcChannels#method.set_fchannel]*
#[inline]
pub fn ncchannels_set_fchannel(
    channels: &mut NcChannels_u64,
    fchannel: impl Into<NcChannel_u32>,
) -> NcChannels_u64 {
    // drop the background color and alpha bit
    *channels &= 0xffffffff_u64 | c_api::NC_NOBACKGROUND_MASK << 32;
    *channels |= (fchannel.into() as NcChannels_u64 & !c_api::NC_NOBACKGROUND_MASK) << 32;
    *channels
}

/// Sets the alpha and coloring bits of an [`NcChannels_u64`] from another [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_fchannel()][NcChannels#method.set_channels]*
#[inline]
pub fn ncchannels_set_channels(
    channels: &mut NcChannels_u64,
    other: impl Into<NcChannels_u64>,
) -> NcChannels_u64 {
    let other = other.into();
    ncchannels_set_bchannel(channels, (other & 0xffffffff_u64) as NcChannel_u32);
    ncchannels_set_fchannel(channels, ((other >> 32) & 0xffffffff_u64) as NcChannel_u32);
    *channels
}

/// Combines two [`NcChannel_u32`]s into an [`NcChannels_u64`].
///
/// *Method: NcChannels.[combine()][NcChannels#method.combine]*
#[inline]
pub fn ncchannels_combine(
    fchannel: impl Into<NcChannel_u32>,
    bchannel: impl Into<NcChannel_u32>,
) -> NcChannels_u64 {
    let mut channels: NcChannels_u64 = 0;
    ncchannels_set_fchannel(&mut channels, fchannel.into());
    ncchannels_set_bchannel(&mut channels, bchannel.into());
    channels
}

/// Returns the `NcChannels_u64` with the fore- and background's color information
/// swapped, but without touching housekeeping bits.
///
/// Alpha is retained unless it would lead to an illegal state: `HIGHCONTRAST`,
/// `TRANSPARENT` and `BLEND` are taken to `OPAQUE` unless the new value is RGB.
///
/// *Method: NcChannels.[reverse()][NcChannels#method.reverse]*
#[inline]
pub fn ncchannels_reverse(channels: impl Into<NcChannels_u64>) -> NcChannels_u64 {
    let channels = channels.into();
    let raw = ((ncchannels_bchannel(channels) as NcChannels_u64) << 32)
        + ncchannels_fchannel(channels) as NcChannels_u64;
    let statemask = ((c_api::NC_NOBACKGROUND_MASK | c_api::NC_BG_ALPHA_MASK as NcChannels_u64)
        << 32)
        | c_api::NC_NOBACKGROUND_MASK
        | c_api::NC_BG_ALPHA_MASK as NcChannels_u64;
    let mut ret = (raw as NcChannels_u64) & !statemask;
    ret |= channels & statemask;
    if ncchannels_bg_alpha(ret) != c_api::NCALPHA_OPAQUE && !ncchannels_bg_rgb_p(ret) {
        ncchannels_set_bg_alpha(&mut ret, c_api::NCALPHA_OPAQUE);
    }
    if ncchannels_fg_alpha(ret) != c_api::NCALPHA_OPAQUE && !ncchannels_fg_rgb_p(ret) {
        ncchannels_set_fg_alpha(&mut ret, c_api::NCALPHA_OPAQUE);
    }
    ret
}

// u8 ---------------------------------------------------------------------

/// Gets the red component from an [`NcChannel_u32`].
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[r()][NcChannel#method.r]*
#[inline]
pub fn ncchannel_r(channel: impl Into<NcChannel_u32>) -> u8 {
    ((channel.into() & 0xff0000) >> 16) as u8
}

/// Gets the green component from an [`NcChannel_u32`].
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[g()][NcChannel#method.g]*
#[inline]
pub fn ncchannel_g(channel: impl Into<NcChannel_u32>) -> u8 {
    ((channel.into() & 0x00ff00) >> 8) as u8
}

/// Gets the blue component from an [`NcChannel_u32`].
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[b()][NcChannel#method.b]*
#[inline]
pub fn ncchannel_b(channel: impl Into<NcChannel_u32>) -> u8 {
    (channel.into() & 0x0000ff) as u8
}

/// Sets the red component of an [`NcChannel_u32`], and returns it.
///
/// *Method: NcChannel.[set_r()][NcChannel#method.set_r]*
//
// Not in the C API.
#[inline]
pub fn ncchannel_set_r(channel: &mut NcChannel_u32, r: u8) -> NcChannel_u32 {
    *channel = (r as NcChannel_u32) << 16 | (*channel & 0xff00) | (*channel & 0xff);
    *channel
}

/// Sets the green component of an [`NcChannel_u32`], and returns it.
///
/// *Method: NcChannel.[set_g()][NcChannel#method.set_g]*
//
// Not in the C API.
#[inline]
pub fn ncchannel_set_g(channel: &mut NcChannel_u32, g: u8) -> NcChannel_u32 {
    *channel = (*channel & 0xff0000) | (g as NcChannel_u32) << 8 | (*channel & 0xff);
    *channel
}

/// Sets the blue component of an [`NcChannel_u32`], and returns it.
///
/// *Method: NcChannel.[set_b()][NcChannel#method.set_b]*
//
// Not in the C API.
#[inline]
pub fn ncchannel_set_b(channel: &mut NcChannel_u32, b: u8) -> NcChannel_u32 {
    *channel = (*channel & 0xff0000) | (*channel & 0xff00) | (b as NcChannel_u32);
    *channel
}

/// Gets the three RGB components from an [`NcChannel_u32`], and returns it.
///
/// Only valid if `ncchannel_rgb_p` would return true for the channel.
///
/// *Method: NcChannel.[rgb()][NcChannel#method.rgb]*
#[inline]
pub fn ncchannel_rgb8(
    channel: impl Into<NcChannel_u32>,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
) -> NcChannel_u32 {
    let channel = channel.into();
    *r = ncchannel_r(channel);
    *g = ncchannel_g(channel);
    *b = ncchannel_b(channel);
    channel
}

/// Sets the three RGB components an [`NcChannel_u32`], and marks it as NOT using the
/// "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannel.[set_rgb()][NcChannel#method.set_rgb]*
#[inline]
pub fn ncchannel_set_rgb8(channel: &mut NcChannel_u32, r: u8, g: u8, b: u8) {
    let rgb: NcRgb_u32 =
        (r as NcChannel_u32) << 16 | (g as NcChannel_u32) << 8 | (b as NcChannel_u32);
    *channel = (*channel & !(c_api::NC_BG_RGB_MASK | c_api::NC_BG_PALETTE))
        | c_api::NC_BGDEFAULT_MASK
        | rgb;
}

/// Gets the three foreground RGB components from an [`NcChannels_u64`], and
/// returns the foreground [`NcChannel_u32`] (which can have some extra bits set).
///
/// *Method: NcChannels.[fg_rgb()][NcChannels#method.fg_rgb]*
#[inline]
pub fn ncchannels_fg_rgb8(
    channels: impl Into<NcChannels_u64>,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
) -> NcChannel_u32 {
    ncchannel_rgb8(ncchannels_fchannel(channels.into()), r, g, b)
}

/// Gets the three background RGB components from an [`NcChannels_u64`], and
/// returns the background [`NcChannel_u32`] (which can have some extra bits set).
///
/// *Method: NcChannels.[bg_rgb()][NcChannels#method.bg_rgb]*
#[inline]
pub fn ncchannels_bg_rgb8(
    channels: impl Into<NcChannels_u64>,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
) -> NcChannel_u32 {
    ncchannel_rgb8(ncchannels_bchannel(channels.into()), r, g, b)
}

/// Sets the three foreground RGB components of an [`NcChannels_u64`], and
/// marks it as NOT using the "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one returns the new `NcChannels_u64`.
///
/// *Method: NcChannels.[set_fg_rgb()][NcChannels#method.set_fg_rgb]*
#[inline]
pub fn ncchannels_set_fg_rgb8(
    channels: &mut NcChannels_u64,
    r: u8,
    g: u8,
    b: u8,
) -> NcChannels_u64 {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_rgb8(&mut channel, r, g, b);
    *channels = (channel as NcChannels_u64) << 32 | *channels & 0xffffffff;
    *channels
}

/// Sets the three background RGB components of an [`NcChannels_u64`], and
/// marks it as NOT using the "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one returns the new `NcChannels_u64`.
///
/// *Method: NcChannels.[set_bg_rgb()][NcChannels#method.set_bg_rgb]*
#[inline]
pub fn ncchannels_set_bg_rgb8(
    channels: &mut NcChannels_u64,
    r: u8,
    g: u8,
    b: u8,
) -> NcChannels_u64 {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_rgb8(&mut channel, r, g, b);
    ncchannels_set_bchannel(channels, channel);
    *channels
}

// NcRgb_u32 -------------------------------------------------------------------

/// Gets the foreground [`NcRgb_u32`] from an [`NcChannels_u64`], shifted to LSBs.
///
/// *Method: NcChannels.[fg_rgb()][NcChannels#method.fg_rgb]*
#[inline]
pub fn ncchannels_fg_rgb(channels: impl Into<NcChannels_u64>) -> NcRgb_u32 {
    ncchannels_fchannel(channels.into()) & c_api::NC_BG_RGB_MASK
}

/// Gets the background [`NcRgb_u32`] from an [`NcChannels_u64`], shifted to LSBs.
///
/// *Method: NcChannels.[bg_rgb()][NcChannels#method.bg_rgb]*
#[inline]
pub fn ncchannels_bg_rgb(channels: impl Into<NcChannels_u64>) -> NcRgb_u32 {
    ncchannels_bchannel(channels.into()) & c_api::NC_BG_RGB_MASK
}

/// Returns true if the foreground channel is set to RGB color.
///
/// *Method: NcChannels.[fg_rgb_p()][NcChannels#method.fg_rgb_p]*
#[inline]
pub fn ncchannels_fg_rgb_p(channels: impl Into<NcChannels_u64>) -> bool {
    ncchannel_rgb_p(ncchannels_fchannel(channels.into()))
}

/// Returns true if the background channel is set to RGB color.
///
/// *Method: NcChannels.[bg_rgb_p()][NcChannels#method.bg_rgb_p]*
#[inline]
pub fn ncchannels_bg_rgb_p(channels: impl Into<NcChannels_u64>) -> bool {
    ncchannel_rgb_p(ncchannels_bchannel(channels.into()))
}

/// Gets the [`NcRgb_u32`] of an [`NcChannel_u32`].
///
/// This function basically removes the 4th byte of the `NcChannel_u32`.
///
/// *Method: NcChannel.[rgb()][NcChannel#method.rgb]*
//
// Not in the C API
#[inline]
pub fn ncchannel_rgb(channel: impl Into<NcChannel_u32>) -> NcRgb_u32 {
    channel.into() & c_api::NC_BG_RGB_MASK
}

/// Returns true if this `NcChannel_u32` is using RGB color.
///
/// *Method: NcChannel.[rgb_p()][NcChannel#method.rgb_p]*
#[inline]
pub fn ncchannel_rgb_p(channel: impl Into<NcChannel_u32>) -> bool {
    let channel = channel.into();
    !(ncchannel_default_p(channel) | ncchannel_palindex_p(channel))
}

/// Sets the [`NcRgb_u32`] of an [`NcChannel_u32`], and marks it as NOT using
/// the "default color", retaining the other bits unchanged.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannel.[set()][NcChannel#method.set]*
#[inline]
pub fn ncchannel_set(channel: &mut NcChannel_u32, rgb: impl Into<NcRgb_u32>) {
    *channel = (*channel & !(c_api::NC_BG_RGB_MASK | c_api::NC_BG_PALETTE))
        | c_api::NC_BGDEFAULT_MASK
        | (rgb.into() & 0x00ffffff);
}

/// Sets the foreground [`NcRgb_u32`] of an [`NcChannels_u64`], and marks it as
/// NOT using the "default color", retaining the other bits unchanged.
///
/// *Method: NcChannels.[set_fg_rgb()][NcChannels#method.set_fg_rgb]*
#[inline]
pub fn ncchannels_set_fg_rgb(channels: &mut NcChannels_u64, rgb: impl Into<NcRgb_u32>) {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set(&mut channel, rgb.into());
    *channels = (channel as NcChannels_u64) << 32 | *channels & 0xffffffff;
}

/// Sets the foreground [`NcRgb_u32`] of an [`NcChannels_u64`], and marks it as NOT using
/// the "default color", retaining the other bits unchanged.
///
/// *Method: NcChannels.[set_bg_rgb()][NcChannels#method.set_bg_rgb]*
#[inline]
pub fn ncchannels_set_bg_rgb(channels: &mut NcChannels_u64, rgb: impl Into<NcRgb_u32>) {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set(&mut channel, rgb);
    ncchannels_set_bchannel(channels, channel);
}

// Default ---------------------------------------------------------------------

/// Is this [`NcChannel_u32`] using the "default color" rather than RGB/palette-indexed?
///
/// *Method: NcChannel.[default_p()][NcChannel#method.default_p]*
#[inline]
pub fn ncchannel_default_p(channel: impl Into<NcChannel_u32>) -> bool {
    (channel.into() & c_api::NC_BGDEFAULT_MASK) == 0
}

/// Marks an [`NcChannel_u32`] as using its "default color". Sets alpha as `OPAQUE`.
///
/// *Method: NcChannel.[set_default()][NcChannel#method.set_default]*
#[inline]
pub fn ncchannel_set_default(channel: &mut NcChannel_u32) -> NcChannel_u32 {
    *channel &= !c_api::NC_BGDEFAULT_MASK; // turn off not-default bit
    ncchannel_set_alpha(channel, c_api::NCALPHA_OPAQUE);
    *channel
}

/// Marks an [`NcChannel_u32`] as NOT using its "default color",
/// retaining the other bits unchanged.
///
/// *Method: NcChannel.[set_not_default()][NcChannel#method.set_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannel_set_not_default(channel: &mut NcChannel_u32) -> NcChannel_u32 {
    *channel |= c_api::NC_BGDEFAULT_MASK;
    *channel
}

/// Is the foreground of an [`NcChannels_u64`] using the "default foreground color"?
///
/// *Method: NcChannels.[fg_default_p()][NcChannels#method.fg_default_p]*
#[inline]
pub fn ncchannels_fg_default_p(channels: impl Into<NcChannels_u64>) -> bool {
    ncchannel_default_p(ncchannels_fchannel(channels.into()))
}

/// Is the background using the "default background color"?
///
/// The "default background color" must generally be used to take advantage of
/// terminal-effected transparency.
///
/// *Method: NcChannels.[bg_default_p()][NcChannels#method.bg_default_p]*
#[inline]
pub fn ncchannels_bg_default_p(channels: impl Into<NcChannels_u64>) -> bool {
    ncchannel_default_p(ncchannels_bchannel(channels.into()))
}

/// Marks the foreground of an [`NcChannels_u64`] as using its "default color",
/// which also marks it opaque, and returns the new [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_fg_default()][NcChannels#method.set_fg_default]*
#[inline]
pub fn ncchannels_set_fg_default(channels: &mut NcChannels_u64) -> NcChannels_u64 {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_default(&mut channel);
    *channels = (channel as NcChannels_u64) << 32 | *channels & 0xffffffff;
    *channels
}

/// Marks the foreground of an [`NcChannels_u64`] as NOT using its "default color",
/// retaining the other bits unchanged, and returns the new [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_fg_not_default()][NcChannels#method.set_fg_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_fg_not_default(channels: &mut NcChannels_u64) -> NcChannels_u64 {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_not_default(&mut channel);
    *channels = (channel as NcChannels_u64) << 32 | *channels & 0xffffffff;
    *channels
}

/// Marks the background of an [`NcChannels_u64`] as using its "default color",
/// which also marks it opaque, and returns the new [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_bg_default()][NcChannels#method.set_bg_default]*
#[inline]
pub fn ncchannels_set_bg_default(channels: &mut NcChannels_u64) -> NcChannels_u64 {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_default(&mut channel);
    ncchannels_set_bchannel(channels, channel);
    *channels
}

/// Marks the background of an [`NcChannels_u64`] as NOT using its "default color",
/// retaining the other bits unchanged, and returns the new [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_bg_not_default()][NcChannels#method.set_bg_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_bg_not_default(channels: &mut NcChannels_u64) -> NcChannels_u64 {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_not_default(&mut channel);
    ncchannels_set_bchannel(channels, channel);
    *channels
}

/// Marks both the foreground and background of an [`NcChannels_u64`] as using their
/// "default color", which also marks them opaque, and returns the new [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_default()][NcChannels#method.set_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_default(channels: &mut NcChannels_u64) -> NcChannels_u64 {
    ncchannels_set_bg_default(&mut ncchannels_set_fg_default(channels))
}

/// Marks both the foreground and background of an [`NcChannels_u64`] as NOT using their
/// "default color", retaining the other bits unchanged, and returns the new [`NcChannels_u64`].
///
/// *Method: NcChannels.[set_not_default()][NcChannels#method.set_not_default]*
//
// Not in the C API
#[inline]
pub fn ncchannels_set_not_default(channels: &mut NcChannels_u64) -> NcChannels_u64 {
    ncchannels_set_bg_not_default(&mut ncchannels_set_fg_not_default(channels))
}

// Palette ---------------------------------------------------------------------

/// Gets the [`NcPaletteIndex`] from the [`NcChannel_u32`].
///
/// The channel must be palette-indexed, or the return value is meaningless.
/// Verify palette indexing with [`ncchannel_palindex_p`].
///
/// *Method: NcChannel.[palindex()][NcChannel#method.palindex]*
pub fn ncchannel_palindex(channel: impl Into<NcChannel_u32>) -> NcPaletteIndex {
    (channel.into() & 0xFF) as NcPaletteIndex
}

/// Sets the [`NcPaletteIndex`] of the [`NcChannel_u32`], and the channel into
/// palette indexed mode.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannel.[set_palindex()][NcChannel#method.set_palindex]*
pub fn ncchannel_set_palindex(channel: &mut NcChannel_u32, index: impl Into<NcPaletteIndex>) {
    ncchannel_set_alpha(channel, c_api::NCALPHA_OPAQUE);
    *channel &= 0xFF000000;
    *channel |= c_api::NC_BGDEFAULT_MASK | c_api::NC_BG_PALETTE | index.into() as NcChannel_u32;
}

/// Is this [`NcChannel_u32`] using palette-indexed color rather than RGB?
///
/// *Method: NcChannel.[palindex_p()][NcChannel#method.palindex_p]*
#[inline]
pub fn ncchannel_palindex_p(channel: impl Into<NcChannel_u32>) -> bool {
    let channel = channel.into();
    !ncchannel_default_p(channel) && (channel & c_api::NC_BG_PALETTE) != 0
}

/// Gets the [`NcPaletteIndex`] from the foreground [`NcChannel_u32`].
///
/// *Method: NcChannels.[fg_palindex()][NcChannels#method.fg_palindex]*
#[inline]
pub fn ncchannels_fg_palindex(channels: impl Into<NcChannels_u64>) -> NcPaletteIndex {
    ncchannel_palindex(ncchannels_fchannel(channels.into()))
}

/// Gets the [`NcPaletteIndex`] from the background [`NcChannel_u32`].
///
/// *Method: NcChannels.[bg_palindex()][NcChannels#method.bg_palindex]*
#[inline]
pub fn ncchannels_bg_palindex(channels: impl Into<NcChannels_u64>) -> NcPaletteIndex {
    ncchannel_palindex(ncchannels_bchannel(channels.into()))
}

/// Is the foreground of an [`NcChannels_u64`] using an [indexed][`NcPaletteIndex`]
/// [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcChannels.[fg_palindex_p()][NcChannels#method.fg_palindex_p]*
#[inline]
pub fn ncchannels_fg_palindex_p(channels: impl Into<NcChannels_u64>) -> bool {
    ncchannel_palindex_p(ncchannels_fchannel(channels.into()))
}

/// Is the background of an [`NcChannels_u64`] using an [indexed][`NcPaletteIndex`]
/// [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcChannels.[bg_palindex_p()][NcChannels#method.bg_palindex_p]*
#[inline]
pub fn ncchannels_bg_palindex_p(channels: impl Into<NcChannels_u64>) -> bool {
    ncchannel_palindex_p(ncchannels_bchannel(channels.into()))
}

/// Sets the foreground of an [`NcChannels_u64`] as using an
/// [*indexed*][`NcPaletteIndex`] [`NcPalette`][crate::NcPalette] color.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannels.[set_fg_palindex()][NcChannels#method.set_fg_palindex]*
#[inline]
#[allow(clippy::unnecessary_cast)]
pub fn ncchannels_set_fg_palindex(channels: &mut NcChannels_u64, index: impl Into<NcPaletteIndex>) {
    let mut channel = ncchannels_fchannel(*channels);
    ncchannel_set_palindex(&mut channel, index.into());
    *channels = (channel as NcChannels_u64) << 32 | (*channels & 0xffffffff as NcChannels_u64)
}

/// Sets the background of an [`NcChannels_u64`] as using an
/// [*indexed*][`NcPaletteIndex`] [`NcPalette`][crate::NcPalette] color.
///
/// Note: Unlike the original C function, this one can't fail.
///
/// *Method: NcChannels.[set_bg_palindex()][NcChannels#method.set_bg_palindex]*
#[inline]
pub fn ncchannels_set_bg_palindex(channels: &mut NcChannels_u64, index: impl Into<NcPaletteIndex>) {
    let mut channel = ncchannels_bchannel(*channels);
    ncchannel_set_palindex(&mut channel, index.into());
    ncchannels_set_bchannel(channels, channel);
}
