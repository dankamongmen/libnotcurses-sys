//!

use crate::c_api::NcPixel_u32;

/// Constructs a libav-compatible ABGR pixel from RGB components.
#[inline]
#[allow(clippy::unnecessary_cast)]
pub const fn ncpixel(red: u8, green: u8, blue: u8) -> NcPixel_u32 {
    0xff000000 as NcPixel_u32
        | red as NcPixel_u32
        | (green as NcPixel_u32) << 8
        | (blue as NcPixel_u32) << 16
}

/// Extracts the 8-bit alpha component from an ABGR pixel.
#[inline]
pub const fn ncpixel_a(pixel: NcPixel_u32) -> u8 {
    ((pixel.to_le() & 0xff000000) >> 24) as u8
}

/// Extracts the 8 bit blue component from an ABGR pixel.
#[inline]
pub const fn ncpixel_b(pixel: NcPixel_u32) -> u8 {
    ((pixel.to_le() & 0x00ff0000) >> 16) as u8
}

/// Extracts the 8 bit green component from an ABGR pixel.
#[inline]
pub const fn ncpixel_g(pixel: NcPixel_u32) -> u8 {
    ((pixel.to_le() & 0x0000ff00) >> 8) as u8
}

/// Extracts the 8 bit red component from an ABGR pixel.
#[inline]
pub const fn ncpixel_r(pixel: NcPixel_u32) -> u8 {
    (pixel.to_le() & 0x000000ff) as u8
}

/// Sets the 8-bit alpha component of an ABGR pixel.
#[inline]
pub fn ncpixel_set_a(pixel: &mut NcPixel_u32, alpha: u8) {
    *pixel = (((*pixel).to_le() & 0x00ffffff) | ((alpha as NcPixel_u32) << 24)).to_le();
}

/// Sets the 8-bit blue component of an ABGR pixel.
#[inline]
pub fn ncpixel_set_b(pixel: &mut NcPixel_u32, blue: u8) {
    *pixel = (((*pixel).to_le() & 0xff00ffff) | ((blue as NcPixel_u32) << 16)).to_le();
}

/// Sets the 8-bit green component of an ABGR pixel.
#[inline]
pub fn ncpixel_set_g(pixel: &mut NcPixel_u32, green: u8) {
    *pixel = (((*pixel).to_le() & 0xffff00ff) | ((green as NcPixel_u32) << 8)).to_le();
}

/// Sets the 8-bit red component of an ABGR pixel.
#[inline]
pub fn ncpixel_set_r(pixel: &mut NcPixel_u32, red: u8) {
    *pixel = (((*pixel).to_le() & 0xffffff00) | red as NcPixel_u32).to_le();
}

/// Sets the RGB components of an ABGR pixel.
#[inline]
pub fn ncpixel_set_rgb8(pixel: &mut NcPixel_u32, red: u8, green: u8, blue: u8) {
    ncpixel_set_b(pixel, blue);
    ncpixel_set_g(pixel, green);
    ncpixel_set_r(pixel, red);
}
