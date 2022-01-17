use crate::{c_api, NcPixel};

impl NcPixel {
    /// Returns a new `NcPixel`.
    pub fn new(value: c_api::NcPixel_u32) -> Self {
        Self(value)
    }

    /// Constructs a libav-compatible ABGR pixel from RGB components.
    pub fn from_rgb8(red: u8, green: u8, blue: u8) -> Self {
        c_api::ncpixel(red, green, blue).into()
    }

    /// Extracts the 8-bit alpha component from an ABGR pixel.
    pub fn a(self) -> u8 {
        c_api::ncpixel_a(self.into())
    }

    /// Extracts the 8 bit blue component from an ABGR pixel.
    pub fn b(self) -> u8 {
        c_api::ncpixel_b(self.into())
    }

    /// Extracts the 8 bit green component from an ABGR pixel.
    pub fn g(self) -> u8 {
        c_api::ncpixel_g(self.into())
    }

    /// Extracts the 8 bit red component from an ABGR pixel.
    pub fn r(self) -> u8 {
        c_api::ncpixel_r(self.into())
    }

    /// Sets the 8-bit alpha component of an ABGR pixel.
    pub fn set_a(&mut self, alpha: u8) {
        c_api::ncpixel_set_a(self.into(), alpha)
    }

    /// Sets the 8-bit green component of an ABGR pixel.
    pub fn set_g(&mut self, green: u8) {
        c_api::ncpixel_set_b(self.into(), green)
    }

    /// Sets the 8-bit blue component of an ABGR pixel.
    pub fn set_b(&mut self, blue: u8) {
        c_api::ncpixel_set_b(self.into(), blue)
    }

    /// Sets the 8-bit red component of an ABGR pixel.
    pub fn set_r(&mut self, red: u8) {
        c_api::ncpixel_set_r(self.into(), red)
    }

    /// Sets the RGB components of an ABGR pixel.
    pub fn set_rgb8(&mut self, red: u8, green: u8, blue: u8) {
        c_api::ncpixel_set_rgb8(self.into(), red, green, blue);
    }
}
