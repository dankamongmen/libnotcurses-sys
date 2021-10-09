use crate::{NcComponent, NcPixel};

/// Constructs a libav-compatible ABGR pixel from RGB [`NcComponent`]s.
#[inline]
#[allow(clippy::unnecessary_cast)]
pub const fn ncpixel(red: NcComponent, green: NcComponent, blue: NcComponent) -> NcPixel {
    0xff000000 as NcPixel | red as NcPixel | (blue as NcPixel) << 8 | (green as NcPixel) << 16
}

/// Extracts the 8-bit alpha [`NcComponent`] from an ABGR pixel.
#[inline]
pub const fn ncpixel_a(pixel: NcPixel) -> NcComponent {
    ((pixel.to_le() & 0xff000000) >> 24) as NcComponent
}

/// Extracts the 8 bit blue [`NcComponent`] from an ABGR pixel.
#[inline]
pub const fn ncpixel_b(pixel: NcPixel) -> NcComponent {
    ((pixel.to_le() & 0x00ff0000) >> 16) as NcComponent
}

/// Extracts the 8 bit green [`NcComponent`] from an ABGR pixel.
#[inline]
pub const fn ncpixel_g(pixel: NcPixel) -> NcComponent {
    ((pixel.to_le() & 0x0000ff00) >> 8) as NcComponent
}

/// Extracts the 8 bit red [`NcComponent`] from an ABGR pixel.
#[inline]
pub const fn ncpixel_r(pixel: NcPixel) -> NcComponent {
    (pixel.to_le() & 0x000000ff) as NcComponent
}

/// Sets the 8-bit alpha [`NcComponent`] of an ABGR pixel.
#[inline]
pub fn ncpixel_set_a(pixel: &mut NcPixel, alpha: NcComponent) {
    *pixel = (((*pixel).to_le() & 0x00ffffff) | ((alpha as NcPixel) << 24)).to_le();
}

/// Sets the 8-bit blue [`NcComponent`] of an ABGR pixel.
#[inline]
pub fn ncpixel_set_b(pixel: &mut NcPixel, blue: NcComponent) {
    *pixel = (((*pixel).to_le() & 0xffff00ff) | ((blue as NcPixel) << 8)).to_le();
}

/// Sets the 8-bit green [`NcComponent`] of an ABGR pixel.
#[inline]
pub fn ncpixel_set_g(pixel: &mut NcPixel, green: NcComponent) {
    *pixel = (((*pixel).to_le() & 0xff00ffff) | ((green as NcPixel) << 16)).to_le();
}

/// Sets the 8-bit red [`NcComponent`] of an ABGR pixel.
#[inline]
pub fn ncpixel_set_r(pixel: &mut NcPixel, red: NcComponent) {
    *pixel = (((*pixel).to_le() & 0xffffff00) | red as NcPixel).to_le();
}

/// Sets the RGB [`NcComponent`]s of an ABGR pixel.
#[inline]
pub fn ncpixel_set_rgb8(
    pixel: &mut NcPixel,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) {
    ncpixel_set_r(pixel, red);
    ncpixel_set_g(pixel, green);
    ncpixel_set_b(pixel, blue);
}
