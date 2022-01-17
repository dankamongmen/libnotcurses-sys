use crate::{c_api, NcComponent, NcPixel};

impl NcPixel {
    /// Returns a new `NcPixel`.
    pub fn new(value: c_api::NcPixel_u32) -> Self {
        Self(value)
    }

    /// Constructs a libav-compatible ABGR pixel from RGB [`NcComponent`]s.
    pub fn from_rgb8(red: NcComponent, green: NcComponent, blue: NcComponent) -> Self {
        c_api::ncpixel(red, green, blue).into()
    }

    /// Extracts the 8-bit alpha [`NcComponent`] from an ABGR pixel.
    pub fn a(self) -> NcComponent {
        c_api::ncpixel_a(self.into())
    }

    /// Extracts the 8 bit blue [`NcComponent`] from an ABGR pixel.
    pub fn b(self) -> NcComponent {
        c_api::ncpixel_b(self.into())
    }

    /// Extracts the 8 bit green [`NcComponent`] from an ABGR pixel.
    pub fn g(self) -> NcComponent {
        c_api::ncpixel_g(self.into())
    }

    /// Extracts the 8 bit red [`NcComponent`] from an ABGR pixel.
    pub fn r(self) -> NcComponent {
        c_api::ncpixel_r(self.into())
    }

    /// Sets the 8-bit alpha [`NcComponent`] of an ABGR pixel.
    pub fn set_a(&mut self, alpha: NcComponent) {
        c_api::ncpixel_set_a(self.into(), alpha)
    }

    /// Sets the 8-bit green [`NcComponent`] of an ABGR pixel.
    pub fn set_g(&mut self, green: NcComponent) {
        c_api::ncpixel_set_b(self.into(), green)
    }

    /// Sets the 8-bit blue [`NcComponent`] of an ABGR pixel.
    pub fn set_b(&mut self, blue: NcComponent) {
        c_api::ncpixel_set_b(self.into(), blue)
    }

    /// Sets the 8-bit red [`NcComponent`] of an ABGR pixel.
    pub fn set_r(&mut self, red: NcComponent) {
        c_api::ncpixel_set_r(self.into(), red)
    }

    /// Sets the RGB [`NcComponent`]s of an ABGR pixel.
    pub fn set_rgb8(&mut self, red: NcComponent, green: NcComponent, blue: NcComponent) {
        c_api::ncpixel_set_rgb8(self.into(), red, green, blue);
    }
}
