use crate::{c_api, NcComponent, NcPixel};

/// Enables the [`NcPixel`] associated methods and constants.
pub trait NcPixelApi {
    fn new(r: NcComponent, g: NcComponent, b: NcComponent) -> Self;
    fn a(self) -> NcComponent;
    fn b(self) -> NcComponent;
    fn g(self) -> NcComponent;
    fn r(self) -> NcComponent;
    fn set_a(&mut self, green: NcComponent);
    fn set_b(&mut self, blue: NcComponent);
    fn set_g(&mut self, green: NcComponent);
    fn set_r(&mut self, red: NcComponent);
    fn set_rgb8(&mut self, red: NcComponent, green: NcComponent, blue: NcComponent);
}

impl NcPixelApi for NcPixel {
    /// Constructs a libav-compatible ABGR pixel from RGB [`NcComponent`]s.
    fn new(red: NcComponent, green: NcComponent, blue: NcComponent) -> Self {
        c_api::ncpixel(red, green, blue)
    }

    /// Extracts the 8-bit alpha [`NcComponent`] from an ABGR pixel.
    fn a(self) -> NcComponent {
        c_api::ncpixel_a(self)
    }

    /// Extracts the 8 bit blue [`NcComponent`] from an ABGR pixel.
    fn b(self) -> NcComponent {
        c_api::ncpixel_b(self)
    }

    /// Extracts the 8 bit green [`NcComponent`] from an ABGR pixel.
    fn g(self) -> NcComponent {
        c_api::ncpixel_g(self)
    }

    /// Extracts the 8 bit red [`NcComponent`] from an ABGR pixel.
    fn r(self) -> NcComponent {
        c_api::ncpixel_r(self)
    }

    /// Sets the 8-bit alpha [`NcComponent`] of an ABGR pixel.
    fn set_a(&mut self, alpha: NcComponent) {
        c_api::ncpixel_set_a(self, alpha)
    }

    /// Sets the 8-bit green [`NcComponent`] of an ABGR pixel.
    fn set_g(&mut self, green: NcComponent) {
        c_api::ncpixel_set_b(self, green)
    }

    /// Sets the 8-bit blue [`NcComponent`] of an ABGR pixel.
    fn set_b(&mut self, blue: NcComponent) {
        c_api::ncpixel_set_b(self, blue)
    }

    /// Sets the 8-bit red [`NcComponent`] of an ABGR pixel.
    fn set_r(&mut self, red: NcComponent) {
        c_api::ncpixel_set_r(self, red)
    }

    /// Sets the RGB [`NcComponent`]s of an ABGR pixel.
    fn set_rgb8(&mut self, red: NcComponent, green: NcComponent, blue: NcComponent) {
        c_api::ncpixel_set_rgb8(self, red, green, blue);
    }
}
