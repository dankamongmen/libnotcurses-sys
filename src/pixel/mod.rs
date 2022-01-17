//! The `NcPixel` API facilitates direct management of the pixels within an
//! [`NcVisual`] (`NcVisuals` keep a backing store of 32-bit RGBA pixels,
//! and render them down to terminal graphics in [`NcVisual.blit`].
//!
//! [`NcVisual`]: crate::NcVisual
//! [`NcVisual.blit`]: crate::NcVisual#method.blit
//
// - NOTE: The pixel color & alpha [`NcComponent`]s are u8 instead of u32.
//   Because of type enforcing, some runtime checks are now unnecessary.
//
// - NOTE: no functions can fail anymore and therefore none returns errors.
//
// functions manually reimplemented: 10
// ------------------------------------------
// (+) done: 10 /  0
// (#) test:  0
// (W) wrap: 10
// ------------------------------------------
//W+ ncpixel
//W+ ncpixel_a
//W+ ncpixel_b
//W+ ncpixel_g
//W+ ncpixel_r
//W+ ncpixel_set_a
//W+ ncpixel_set_b
//W+ ncpixel_set_g
//W+ ncpixel_set_r
//W+ ncpixel_set_rgb8

use crate::NcDim;

mod methods;
pub(crate) mod reimplemented;

mod pixel_impl;
pub use pixel_impl::NcPixelImpl;

/// An ABGR pixel.
///
/// ## Diagram
///
/// ```txt
/// AAAAAAAA BBBBBBBB GGGGGGGG RRRRRRRR
/// ```
///
/// `NcPixel` has 8 bits of alpha,  more or less linear, contributing
/// directly to the usual alpha blending equation.
///
/// We map the 8 bits of alpha to 2 bits of alpha via a [level function][0]
///
/// [0]: https://nick-black.com/dankwiki/index.php?title=Notcurses#Transparency.2FContrasting
///
/// The `NcPixel` API facilitates direct management of the pixels within an
/// [`NcVisual`] (`NcVisuals` keep a backing store of 32-bit RGBA pixels,
/// and render them down to terminal graphics in `NcVisual.render`).
///
/// Per libav, we "store as BGRA on little-endian, and ARGB on big-endian".
/// This is an RGBA *byte-order* scheme. libav emits bytes, not words. Those
/// bytes are R-G-B-A. When read as words, on little endian this will be ABGR,
/// and on big-endian this will be RGBA. force everything to LE ABGR, a no-op
/// on (and thus favoring) little-endian, which is the dominant ordering for
/// processor architectures (x86, most ARM implementations, base RISC-V
/// implementations) and their associated memory.
///
/// [`NcVisual`]: crate::NcVisual
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NcPixel(pub c_api::NcPixel_u32);

mod std_impls {
    use super::{c_api::NcPixel_u32, NcPixel};

    crate::unit_impl_from![NcPixel, NcPixel_u32];
}

/// Contains the pixel geometry information as returned by the
/// `NcPlane.`[`pixel_geom`][crate::NcPlane#method.pixel_geom] method.
///
/// If bitmaps are not supported, the fields `max_bitmap_*` will be 0.
///
/// See also [`NcVGeom`][crate::NcVGeom].
#[derive(Clone, Debug)]
pub struct NcPixelGeometry {
    /// The height in pixels of the display region.
    pub term_y: NcDim,

    /// The width in pixels of the display region.
    pub term_x: NcDim,

    /// The height in pixels of a single cell.
    pub cell_y: NcDim,

    /// The width in pixels of a single cell.
    pub cell_x: NcDim,

    /// The height in pixels of the maximum displayable bitmap (0 if not supported).
    pub max_bitmap_y: NcDim,

    /// The width in pixels of the maximum displayable bitmap (0 if not supported).
    pub max_bitmap_x: NcDim,
}

pub(crate) mod c_api {
    use crate::bindings::ffi;

    /// An ABGR pixel.
    ///
    /// It's recommended to use [`NcPixel`][crate::NcPixel] instead.
    ///
    /// ## Diagram
    ///
    /// ```txt
    /// AAAAAAAA BBBBBBBB GGGGGGGG RRRRRRRR
    /// ```
    ///
    /// `type in C: ncpixel (uint32_t)`
    ///
    /// `NcPixel` has 8 bits of alpha,  more or less linear, contributing
    /// directly to the usual alpha blending equation.
    ///
    /// We map the 8 bits of alpha to 2 bits of alpha via a [level function][0]
    ///
    /// [0]: https://nick-black.com/dankwiki/index.php?title=Notcurses#Transparency.2FContrasting
    ///
    /// The `NcPixel` API facilitates direct management of the pixels within an
    /// [`NcVisual`] (`NcVisuals` keep a backing store of 32-bit RGBA pixels,
    /// and render them down to terminal graphics in `NcVisual.render`).
    ///
    /// Per libav, we "store as BGRA on little-endian, and ARGB on big-endian".
    /// This is an RGBA *byte-order* scheme. libav emits bytes, not words. Those
    /// bytes are R-G-B-A. When read as words, on little endian this will be ABGR,
    /// and on big-endian this will be RGBA. force everything to LE ABGR, a no-op
    /// on (and thus favoring) little-endian, which is the dominant ordering for
    /// processor architectures (x86, most ARM implementations, base RISC-V
    /// implementations) and their associated memory.
    ///
    /// [`NcVisual`]: crate::NcVisual
    pub type NcPixel_u32 = u32;

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
