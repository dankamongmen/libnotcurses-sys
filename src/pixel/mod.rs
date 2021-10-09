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

pub use methods::NcPixelMethods;

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
