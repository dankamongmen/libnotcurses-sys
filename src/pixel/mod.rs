//! The `NcPixel` API facilitates direct management of the pixels within an
//! [`NcVisual`] (`NcVisuals` keep a backing store of 32-bit RGBA pixels,
//! and render them down to terminal graphics in [`NcVisual.blit`].
//!
//! [`NcVisual`]: crate::NcVisual
//! [`NcVisual.blit`]: crate::NcVisual#method.blit
//
// - NOTE: The pixel color & alpha components are u8 instead of u32.
//   Because of type enforcing, some runtime checks are now unnecessary.
//
// - NOTE: no functions can fail anymore and therefore none returns errors.
//
// functions manually reimplemented: 10
// ------------------------------------------
// (+) done:  9
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
//X  ncpixel_set_rgb8

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

mod core_impls {
    use super::{c_api::NcPixel_u32, NcPixel};
    use crate::{NcRgb, NcRgba};

    crate::from_primitive![NcPixel, NcPixel_u32];
    crate::unit_impl_from![NcPixel, NcPixel_u32];
    crate::unit_impl_fmt![bases+display; NcPixel];

    impl From<NcRgb> for NcPixel {
        fn from(rgb: NcRgb) -> Self {
            Self::from_rgb(rgb)
        }
    }
    impl From<NcPixel> for NcRgb {
        fn from(bgra: NcPixel) -> Self {
            bgra.to_rgb()
        }
    }
    impl From<NcRgba> for NcPixel {
        fn from(rgba: NcRgba) -> Self {
            Self::from_rgba(rgba)
        }
    }
    impl From<NcPixel> for NcRgba {
        fn from(bgra: NcPixel) -> Self {
            bgra.to_rgba()
        }
    }
}

/// Contains the pixel geometry information as returned by the
/// `NcPlane.`[`pixel_geom`][crate::NcPlane#method.pixel_geom] method.
///
/// If bitmaps are not supported, the fields `max_bitmap_*` will be 0.
///
/// See also [`NcVisualGeometry`][crate::NcVisualGeometry].
#[derive(Clone, Debug)]
pub struct NcPixelGeometry {
    /// The height in pixels of the display region.
    pub term_y: u32,

    /// The width in pixels of the display region.
    pub term_x: u32,

    /// The height in pixels of a single cell.
    pub cell_y: u32,

    /// The width in pixels of a single cell.
    pub cell_x: u32,

    /// The height in pixels of the maximum displayable bitmap (0 if not supported).
    pub max_bitmap_y: u32,

    /// The width in pixels of the maximum displayable bitmap (0 if not supported).
    pub max_bitmap_x: u32,
}

pub(crate) mod c_api {
    pub use super::pixel_impl::c_api::*;

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
}
