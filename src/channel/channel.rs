//! `NcChannel`
// #![allow(clippy::unnecessary_cast)] // CHECK

use crate::{
    c_api::{self, NcChannel_u32},
    NcAlpha, NcChannels, NcPaletteIndex, NcRgb,
};

// NcChannel
//
/// 32 bits of context-dependent info containing [`NcRgb`] + [`NcAlpha`] + extra
///
/// It is composed of:
/// - a 24-bit [`NcRgb`] value
/// - plus 8 bits divided in:
///   - 2 bits of [`NcAlpha`]
///   - 6 bits of context-dependent info
///
/// The context details are documented in [`NcChannels`]
///
/// ## Diagram
///
/// ```txt
/// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
/// `type in C: channel (uint32_t)`
///
/// # See also
/// - [`NcRgb`]
/// - [`NcRgba`]
///
/// [`NcRgb`]: crate::NcRgb
/// [`NcRgba`]: crate::NcRgba
/// [`NcAlpha`]: crate::NcAlpha
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NcChannel(pub NcChannel_u32);

mod std_impls {
    use super::{NcChannel, NcChannel_u32};

    impl Default for NcChannel {
        fn default() -> Self {
            Self::with_default()
        }
    }

    impl From<NcChannel> for [u8; 3] {
        #[inline]
        fn from(rgb: NcChannel) -> Self {
            rgb.into()
        }
    }
    impl From<[u8; 3]> for NcChannel {
        fn from(array: [u8; 3]) -> Self {
            Self::from_rgb(array)
        }
    }

    impl From<NcChannel> for (u8, u8, u8) {
        #[inline]
        fn from(rgb: NcChannel) -> Self {
            rgb.into()
        }
    }
    impl From<(u8, u8, u8)> for NcChannel {
        fn from(tuple: (u8, u8, u8)) -> Self {
            Self::from_rgb(tuple)
        }
    }

    crate::from_primitive![NcChannel, NcChannel_u32];
    crate::unit_impl_from![NcChannel, NcChannel_u32];
    crate::unit_impl_fmt![bases+display; NcChannel];
}

/// # Constants
impl NcChannel {
    /// If this bit is set, we are *not* using the default color.
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_DEFAULT_MASK`][NcChannels#associatedconstant.BG_DEFAULT_MASK]
    pub const DEFAULT_MASK: u32 = super::c_api::NC_BGDEFAULT_MASK;

    /// Extract these bits to get the (background) [`NcAlpha`] mask.
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_ALPHA_MASK`][NcChannels#associatedconstant.BG_ALPHA_MASK]
    pub const ALPHA_MASK: u32 = super::c_api::NC_BG_ALPHA_MASK;

    /// If this bit *and*
    /// [`DEFAULT_MASK`][NcChannel#associatedconstant.DEFAULT_MASK] are set,
    /// we're using a palette-indexed background color
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_PALETTE_MASK`][NcChannels#associatedconstant.BG_PALETTE_MASK]
    pub const PALETTE_MASK: u32 = super::c_api::NC_BG_PALETTE;

    /// Extract these bits to get the background [`NcRgb`][crate::NcRgb] value.
    ///
    /// Note: this is equivalent to
    /// [`NcChannels::BG_RGB_MASK`][NcChannels#associatedconstant.BG_RGB_MASK]
    pub const RGB_MASK: u32 = super::c_api::NC_BG_RGB_MASK;
}

/// # Constructors
impl NcChannel {
    /// New `NcChannel`, set to black and NOT using the "default color".
    pub fn new() -> Self {
        Self(c_api::NC_BGDEFAULT_MASK)
    }

    /// New `NcChannel`, set to black and using the "default color".
    pub fn with_default() -> Self {
        Self(0)
    }

    /// New `NcChannel`, expects [`NcRgb`].
    pub fn from_rgb(rgb: impl Into<NcRgb>) -> Self {
        Self::new().set(rgb.into())
    }

    /// New `NcChannel`, expects [`NcRgb`] & [`NcAlpha`].
    pub fn from_rgb_alpha(rgb: impl Into<NcRgb>, alpha: NcAlpha) -> Self {
        Self::new().set(rgb.into()).set_alpha(alpha)
    }
}

/// # Methods
impl NcChannel {
    // Combine

    /// Combines this [`NcChannel`] as foreground, with another as background
    /// into an [`NcChannels`].
    ///
    /// *C style function: [channels_combine()][c_api::ncchannels_combine].*
    //
    // Not in the C API
    pub fn fcombine(&self, bchannel: impl Into<NcChannel>) -> NcChannels {
        c_api::ncchannels_combine(self.0, bchannel.into().0).into()
    }

    /// Combines this [`NcChannel`] as background, with another as foreground
    /// into an [`NcChannels`].
    ///
    /// *C style function: [channels_combine()][c_api::ncchannels_combine].*
    //
    // Not in the C API
    pub fn bcombine(&self, fchannel: impl Into<NcChannel>) -> NcChannels {
        c_api::ncchannels_combine(fchannel.into().0, self.0).into()
    }

    // Alpha

    /// Gets the [`NcAlpha`].
    ///
    /// *C style function: [ncchannel_alpha()][c_api::ncchannel_alpha].*
    pub fn alpha(&self) -> NcAlpha {
        c_api::ncchannel_alpha(self.0).into()
    }

    /// Sets the [`NcAlpha`].
    ///
    /// *C style function: [ncchannel_set_alpha()][c_api::ncchannel_set_alpha].*
    pub fn set_alpha(&mut self, alpha: impl Into<NcAlpha>) -> Self {
        c_api::ncchannel_set_alpha(&mut self.0, alpha.into());
        *self
    }

    // NcRgb

    /// Returns true if the channel is set to RGB color.
    ///
    /// *C style function: [ncchannel_rgb_p()][c_api::ncchannel_rgb_p].*
    pub fn rgb_p(&self) -> bool {
        c_api::ncchannel_rgb_p(self.0)
    }

    /// Gets the [`NcRgb`].
    ///
    /// *C style function: [ncchannel_rgb()][c_api::ncchannel_rgb].*
    //
    // Not in the C API
    pub fn rgb(&self) -> NcRgb {
        c_api::ncchannel_rgb(self.0).into()
    }

    /// Sets the [`NcRgb`], and marks the NcChannel as NOT using the
    /// "default color", retaining the other bits unchanged.
    ///
    /// *C style function: [ncchannel_set()][c_api::ncchannel_set].*
    pub fn set(&mut self, rgb: impl Into<NcRgb>) -> Self {
        c_api::ncchannel_set(&mut self.0, rgb.into());
        *self
    }

    // u8

    /// Gets the three components.
    ///
    /// *C style function: [ncchannel_rgb8()][c_api::ncchannel_rgb8].*
    pub fn rgb8(&self) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncchannel_rgb8(self.0, &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Sets the three components, and
    /// marks the NcChannel as NOT using the "default color".
    ///
    /// *C style function: [ncchannel_set_rgb8()][c_api::ncchannel_set_rgb8].*
    pub fn set_rgb(&mut self, rgb: impl Into<NcRgb>) -> Self {
        let (r, g, b) = rgb.into().into();
        c_api::ncchannel_set_rgb8(&mut self.0, r, g, b);
        *self
    }

    /// Gets the red component.
    ///
    /// *C style function: [ncchannel_r()][c_api::ncchannel_r].*
    pub fn r(&self) -> u8 {
        c_api::ncchannel_r(self.0)
    }

    /// Gets the green component.
    ///
    /// *C style function: [ncchannel_g()][c_api::ncchannel_g].*
    pub fn g(&self) -> u8 {
        c_api::ncchannel_g(self.0)
    }

    /// Gets the blue component.
    ///
    /// *C style function: [ncchannel_b()][c_api::ncchannel_b].*
    pub fn b(&self) -> u8 {
        c_api::ncchannel_b(self.0)
    }

    /// Sets the red component, and returns the new `NcChannel`.
    ///
    /// *C style function: [ncchannel_set_r()][c_api::ncchannel_set_r].*
    //
    // Not in the C API
    pub fn set_r(&mut self, r: impl Into<u8>) -> Self {
        c_api::ncchannel_set_r(&mut self.0, r.into()).into()
    }

    /// Sets the green component, and returns the new `NcChannel`.
    ///
    /// *C style function: [ncchannel_set_g()][c_api::ncchannel_set_g].*
    //
    // Not in the C API
    pub fn set_g(&mut self, g: impl Into<u8>) -> Self {
        c_api::ncchannel_set_g(&mut self.0, g.into()).into()
    }

    /// Sets the blue component, and returns the new `NcChannel`.
    ///
    /// *C style function: [ncchannel_set_b()][c_api::ncchannel_set_b].*
    //
    // Not in the C API
    pub fn set_b(&mut self, b: impl Into<u8>) -> Self {
        c_api::ncchannel_set_b(&mut self.0, b.into()).into()
    }

    // default color

    /// Is this `NcChannel` using the "default color" rather than RGB/palette-indexed?
    ///
    /// *C style function: [ncchannel_default_p()][c_api::ncchannel_default_p].*
    pub fn default_p(&self) -> bool {
        c_api::ncchannel_default_p(self.0)
    }

    /// Marks this `NcChannel` as using its "default color",
    /// which also marks it opaque.
    ///
    /// *C style function: [ncchannel_set_default()][c_api::ncchannel_set_default].*
    pub fn set_default(&mut self) -> Self {
        c_api::ncchannel_set_default(&mut self.0).into()
    }

    /// Marks this `NcChannel` as *not* using its "default color".
    ///
    /// The following methods also marks the channel as NOT using the "default color":
    /// - [`new`][NcChannel#method.new]
    /// - [`set`][NcChannel#method.set]
    /// - [`set_rgb`][NcChannel#method.set_rgb]
    ///
    /// *C style function: [ncchannel_set_not_default()][c_api::ncchannel_set_not_default].*
    //
    // Not in the C API
    pub fn set_not_default(&mut self) -> Self {
        c_api::ncchannel_set_not_default(&mut self.0).into()
    }

    // NcPaletteIndex

    /// Extracts the [`NcPaletteIndex`] from the [`NcChannel`].
    ///
    /// The channel must be palette-indexed, or the return value is meaningless.
    /// Verify palette indexing with [`palindex_p`][NcChannel#method.palindex_p].
    ///
    /// *C style function: [ncchannel_palindex()][c_api::ncchannel_palindex].*
    pub fn palindex(&self) -> NcPaletteIndex {
        c_api::ncchannel_palindex(self.0)
    }

    /// Is this NcChannel using palette-indexed color rather a than RGB?
    ///
    /// *C style function: [ncchannel_palindex_p()][c_api::ncchannel_palindex_p].*
    pub fn palindex_p(&self) -> bool {
        c_api::ncchannel_palindex_p(self.0)
    }

    /// Sets the [`NcPaletteIndex`] of the [`NcChannel`], and the channel into
    /// palette-indexed mode.
    ///
    /// *C style function: [ncchannel_set_palindex()][c_api::ncchannel_set_palindex].*
    pub fn set_palindex(&mut self, index: impl Into<NcPaletteIndex>) -> Self {
        c_api::ncchannel_set_palindex(&mut self.0, index.into());
        *self
    }
}
