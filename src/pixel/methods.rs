use crate::{fns, NcComponent, NcPixel};

/// Enables the [`NcPixel`] methods.
//
// NOTE: waiting for: https://github.com/rust-lang/rust/issues/56546
// to move doc comments to the trait and appear unhidden at the implementation.
pub trait NcPixelMethods {
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

impl NcPixelMethods for NcPixel {
    /// Constructs a libav-compatible ABGR pixel from RGB [`NcComponent`]s.
    fn new(red: NcComponent, green: NcComponent, blue: NcComponent) -> Self {
        fns::ncpixel(red, green, blue)
    }

    /// Extracts the 8-bit alpha [`NcComponent`] from an ABGR pixel.
    fn a(self) -> NcComponent {
        fns::ncpixel_a(self)
    }

    /// Extracts the 8 bit blue [`NcComponent`] from an ABGR pixel.
    fn b(self) -> NcComponent {
        fns::ncpixel_b(self)
    }

    /// Extracts the 8 bit green [`NcComponent`] from an ABGR pixel.
    fn g(self) -> NcComponent {
        fns::ncpixel_g(self)
    }

    /// Extracts the 8 bit red [`NcComponent`] from an ABGR pixel.
    fn r(self) -> NcComponent {
        fns::ncpixel_r(self)
    }

    /// Sets the 8-bit alpha [`NcComponent`] of an ABGR pixel.
    fn set_a(&mut self, alpha: NcComponent) {
        fns::ncpixel_set_a(self, alpha)
    }

    /// Sets the 8-bit green [`NcComponent`] of an ABGR pixel.
    fn set_g(&mut self, green: NcComponent) {
        fns::ncpixel_set_b(self, green)
    }

    /// Sets the 8-bit blue [`NcComponent`] of an ABGR pixel.
    fn set_b(&mut self, blue: NcComponent) {
        fns::ncpixel_set_b(self, blue)
    }

    /// Sets the 8-bit red [`NcComponent`] of an ABGR pixel.
    fn set_r(&mut self, red: NcComponent) {
        fns::ncpixel_set_r(self, red)
    }

    /// Sets the RGB [`NcComponent`]s of an ABGR pixel.
    fn set_rgb8(&mut self, red: NcComponent, green: NcComponent, blue: NcComponent) {
        fns::ncpixel_set_rgb8(self, red, green, blue);
    }
}
