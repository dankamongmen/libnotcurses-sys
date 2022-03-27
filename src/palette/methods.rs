//! `NcPalette` methods and associated functions.

use crate::{c_api, error, Nc, NcPalette, NcPaletteIndex, NcResult, NcRgb};

impl NcPalette {
    /// Returns a new `NcPalette`.
    ///
    /// *C style function: [ncpalette_new()][c_api::ncpalette_new].*
    pub fn new<'a>(nc: &mut Nc) -> &'a mut Self {
        unsafe { &mut *c_api::ncpalette_new(nc) }
    }

    /// Frees this `NcPalette`.
    ///
    /// *C style function: [ncpalette_free()][c_api::ncpalette_free].*
    pub fn free(&mut self) {
        unsafe {
            c_api::ncpalette_free(self);
        }
    }

    /// Attempts to configure the terminal with this `NcPalette`.
    ///
    /// *C style function: [ncpalette_use()][c_api::ncpalette_use].*
    pub fn r#use(&self, nc: &mut Nc) -> NcResult<()> {
        error![unsafe { c_api::ncpalette_use(nc, self) }]
    }

    /// Returns the [`NcRgb`] value from an [`NcChannel`][crate::NcChannel]
    /// entry inside this `NcPalette`.
    ///
    /// *C style function: [ncpalette_get()][c_api::ncpalette_get].*
    pub fn get(&self, index: impl Into<NcPaletteIndex>) -> NcRgb {
        c_api::ncpalette_get(self, index.into()).into()
    }

    /// Returns the individual RGB color components from an
    /// [`NcChannel`][crate::NcChannel] entry inside this `NcPalette`.
    ///
    /// *C style function: [ncpalette_get_rgb()][c_api::ncpalette_get_rgb8].*
    pub fn get_rgb8(&self, index: impl Into<NcPaletteIndex>) -> (u8, u8, u8) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncpalette_get_rgb8(self, index.into(), &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Sets the [`NcRgb`] value of the [`NcChannel`][crate::NcChannel] entry
    /// inside this NcPalette.
    ///
    /// *C style function: [ncpalette_set()][c_api::ncpalette_set].*
    pub fn set(&mut self, index: impl Into<NcPaletteIndex>, rgb: impl Into<NcRgb>) {
        c_api::ncpalette_set(self, index.into(), rgb.into().into())
    }

    /// Sets the individual RGB components of an [`NcChannel`][crate::NcChannel]
    /// entry inside an [`NcPalette`].
    ///
    /// *C style function: [ncpalette_set_rgb8()][c_api::ncpalette_set_rgb8].*
    #[inline]
    pub fn set_rgb8(
        palette: &mut NcPalette,
        index: impl Into<NcPaletteIndex>,
        red: u8,
        green: u8,
        blue: u8,
    ) {
        c_api::ncpalette_set_rgb8(palette, index.into(), red, green, blue)
    }
}
