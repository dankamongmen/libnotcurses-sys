//!

/// Pixel blitting implementations, informative only.
///
/// This is returned by [`Nc.check_pixel_support`].
///
/// [`Nc.check_pixel_support`]: crate::Nc#method.check_pixel_support
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NcPixelImpl {
    /// No pixel support.
    None = c_api::NCPIXEL_NONE,

    /// Sixel.
    Sixel = c_api::NCPIXEL_SIXEL,

    /// Linux framebuffer.
    LinuxFb = c_api::NCPIXEL_LINUXFB,

    /// iTerm2.
    Iterm2 = c_api::NCPIXEL_ITERM2,

    /// Kitty prior to C=1 and animation.
    KittyStatic = c_api::NCPIXEL_KITTY_STATIC,

    /// Kitty with animation but not reflexive composition.
    KittyAnimated = c_api::NCPIXEL_KITTY_ANIMATED,

    /// Kitty with reflexive composition.
    KittySelfRef = c_api::NCPIXEL_KITTY_SELFREF,
}

mod core_impls {
    use super::{c_api::*, NcPixelImpl};
    use core::fmt;

    impl Default for NcPixelImpl {
        fn default() -> Self {
            Self::None
        }
    }

    impl fmt::Display for NcPixelImpl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcPixelImpl::*;
            write!(
                f,
                "{}",
                match self {
                    NcPixelImpl::None => "None",
                    Sixel => "Sixel",
                    LinuxFb => "LinuxFb",
                    Iterm2 => "Iterm2",
                    KittyStatic => "KittyStatic",
                    KittyAnimated => "KittyAnimated",
                    KittySelfRef => "KittySelfRef",
                }
            )
        }
    }

    impl From<NcPixelImpl_u32> for NcPixelImpl {
        fn from(alpha: NcPixelImpl_u32) -> Self {
            use NcPixelImpl::*;
            match alpha {
                NCPIXEL_NONE => NcPixelImpl::None,
                NCPIXEL_SIXEL => Sixel,
                NCPIXEL_LINUXFB => LinuxFb,
                NCPIXEL_ITERM2 => Iterm2,
                NCPIXEL_KITTY_STATIC => KittyStatic,
                NCPIXEL_KITTY_ANIMATED => KittyAnimated,
                NCPIXEL_KITTY_SELFREF => KittySelfRef,
                _ => Self::default(),
            }
        }
    }

    impl From<NcPixelImpl> for NcPixelImpl_u32 {
        fn from(alpha: NcPixelImpl) -> Self {
            use NcPixelImpl::*;
            match alpha {
                NcPixelImpl::None => NCPIXEL_NONE,
                Sixel => NCPIXEL_SIXEL,
                LinuxFb => NCPIXEL_LINUXFB,
                Iterm2 => NCPIXEL_ITERM2,
                KittyStatic => NCPIXEL_KITTY_STATIC,
                KittyAnimated => NCPIXEL_KITTY_ANIMATED,
                KittySelfRef => NCPIXEL_KITTY_SELFREF,
            }
        }
    }
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// Pixel blitting implementations, informative only.
    ///
    /// It's recommended to use [`NcPixelImpl`][crate::NcPixelImpl] instead.
    ///
    /// This is returned by [`notcurses_check_pixel_support`]
    ///
    /// # Associated `c_api` constants
    ///
    /// - [`NCPIXEL_NONE`]
    /// - [`NCPIXEL_SIXEL`]
    /// - [`NCPIXEL_LINUXFB`]
    /// - [`NCPIXEL_ITERM2`]
    /// - [`NCPIXEL_KITTY_STATIC`]
    /// - [`NCPIXEL_KITTY_ANIMATED`]
    /// - [`NCPIXEL_KITTY_SELFREF`]
    ///
    /// [`notcurses_check_pixel_support`]: crate::c_api::notcurses::check_pixel_support
    pub type NcPixelImpl_u32 = ffi::ncpixelimpl_e;

    /// [`NcPixelImpl_u32`] No pixel support.
    pub const NCPIXEL_NONE: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_NONE;

    /// [`NcPixelImpl_u32`] Sixel.
    pub const NCPIXEL_SIXEL: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_SIXEL;

    /// [`NcPixelImpl_u32`] Linux framebuffer.
    pub const NCPIXEL_LINUXFB: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_LINUXFB;

    /// [`NcPixelImpl_u32`] iTerm2.
    pub const NCPIXEL_ITERM2: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_ITERM2;

    /// [`NcPixelImpl_u32`] Kitty prior to C=1 and animation.
    pub const NCPIXEL_KITTY_STATIC: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_KITTY_STATIC;

    /// [`NcPixelImpl_u32`] Kitty with animation but not reflexive composition.
    pub const NCPIXEL_KITTY_ANIMATED: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_KITTY_ANIMATED;

    /// [`NcPixelImpl_u32`] Kitty with reflexive composition.
    pub const NCPIXEL_KITTY_SELFREF: NcPixelImpl_u32 = ffi::ncpixelimpl_e_NCPIXEL_KITTY_SELFREF;
}
