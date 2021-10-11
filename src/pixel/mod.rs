//! The ncpixel API facilitates direct management of the pixels within an
//! ncvisual (ncvisuals keep a backing store of 32-bit RGBA pixels, and render
//! them down to terminal graphics in ncvisual_render()).
//
// - NOTE: The pixel color & alpha [`NcComponent`]s are u8 instead of u32.
//   Because of type enforcing, some runtime checks are now unnecessary.
//
// - NOTE: None of the functions can't fail anymore and don't have to return an error.
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

mod methods;
pub(crate) mod reimplemented;

use crate::NcDim;
pub use methods::NcPixelApi;

// NcPixel (RGBA)
/// An ABGR pixel.
///
/// ## Diagram
///
/// ```txt
/// AAAAAAAA GGGGGGGG BBBBBBBB RRRRRRRR
/// ```
///
/// `type in C: ncpixel (uint32_t)`
///
/// NcPixel has 8 bits of alpha,  more or less linear, contributing
/// directly to the usual alpha blending equation.
///
/// We map the 8 bits of alpha to 2 bits of alpha via a [level
/// function](https://nick-black.com/dankwiki/index.php?title=Notcurses#Transparency.2FContrasting)
///
/// The ncpixel API facilitates direct management of the pixels within an
/// ncvisual (ncvisuals keep a backing store of 32-bit RGBA pixels, and render
/// them down to terminal graphics in ncvisual_render()).
///
/// Per libav, we "store as BGRA on little-endian, and ARGB on big-endian".
/// This is an RGBA *byte-order* scheme. libav emits bytes, not words. Those
/// bytes are R-G-B-A. When read as words, on little endian this will be ABGR,
/// and on big-endian this will be RGBA. force everything to LE ABGR.
///
pub type NcPixel = u32;

/// Pixel blitting implementations. (Informative only).
///
/// Returned by [`check_pixel_support`][crate::Nc#method.check_pixel_support].
pub type NcPixelImpl = crate::bindings::ffi::ncpixelimpl_e;

crate::impl_api![
    NcPixelImpl,
    NcPixelImplApi,
    /// No pixel support (for [`NcPixelImpl`]).
    const NOPIXEL: NcPixelImpl = constants::NCPIXEL_NONE;,
    /// Sixel (for [`NcPixelImpl`]).
    const SIXEL: NcPixelImpl = constants::NCPIXEL_SIXEL;,
    /// Linux framebuffer (for [`NcPixelImpl`]).
    const LINUXFB: NcPixelImpl = constants::NCPIXEL_LINUXFB;,
    /// iTerm2 (for [`NcPixelImpl`]).
    const ITERM2: NcPixelImpl = constants::NCPIXEL_ITERM2;,
    /// Kitty prior to C=1 and animation (for [`NcPixelImpl`]).
    const KITTY_STATIC: NcPixelImpl = constants::NCPIXEL_KITTY_STATIC;,
    /// Kitty with animation but not reflexive composition (for [`NcPixelImpl`]).
    const KITTY_ANIMATED: NcPixelImpl = constants::NCPIXEL_KITTY_ANIMATED;,
    /// Kitty with reflexive composition (for [`NcPixelImpl`]).
    const KITTY_SELFREF: NcPixelImpl = constants::NCPIXEL_KITTY_SELFREF;
];

pub(crate) mod constants {
    use crate::NcPixelImpl;

    /// No pixel support (for [`NcPixelImpl`]).
    pub const NCPIXEL_NONE: NcPixelImpl = crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_NONE;
    /// Sixel (for [`NcPixelImpl`]).
    pub const NCPIXEL_SIXEL: NcPixelImpl = crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_SIXEL;
    /// Linux framebuffer (for [`NcPixelImpl`]).
    pub const NCPIXEL_LINUXFB: NcPixelImpl = crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_LINUXFB;
    /// iTerm2 (for [`NcPixelImpl`]).
    pub const NCPIXEL_ITERM2: NcPixelImpl = crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_ITERM2;
    /// Kitty prior to C=1 and animation (for [`NcPixelImpl`]).
    pub const NCPIXEL_KITTY_STATIC: NcPixelImpl =
        crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_KITTY_STATIC;
    /// Kitty with animation but not reflexive composition (for [`NcPixelImpl`]).
    pub const NCPIXEL_KITTY_ANIMATED: NcPixelImpl =
        crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_KITTY_ANIMATED;
    /// Kitty with reflexive composition (for [`NcPixelImpl`]).
    pub const NCPIXEL_KITTY_SELFREF: NcPixelImpl =
        crate::bindings::ffi::ncpixelimpl_e_NCPIXEL_KITTY_SELFREF;
}

/// Contains the pixel geometry information as returned by the
/// NcPlane.[`pixel_geom`][crate::NcPlane#method.pixel_geom] method.
///
/// If bitmaps are not supported, the fields `max_bitmap_*` will be 0.
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
