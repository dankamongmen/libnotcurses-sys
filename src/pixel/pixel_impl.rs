//!

use super::c_api;

/// Pixel blitting implementations, informative only.
///
/// This is returned by [`Nc.check_pixel_support`].
///
/// [`Nc.check_pixel_support`]: crate::Nc#method.check_pixel_support
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
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

mod std_impls {
    use super::{c_api, NcPixelImpl};
    use std::fmt;

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

    impl From<c_api::NcPixelImpl_u32> for NcPixelImpl {
        fn from(alpha: c_api::NcPixelImpl_u32) -> Self {
            use {c_api::*, NcPixelImpl::*};
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

    impl From<NcPixelImpl> for c_api::NcPixelImpl_u32 {
        fn from(alpha: NcPixelImpl) -> Self {
            use {c_api::*, NcPixelImpl::*};
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
