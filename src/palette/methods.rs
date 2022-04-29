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

    /// Sets the [`NcRgb`] value of the [`NcChannel`][crate::NcChannel] entry
    /// inside this NcPalette.
    ///
    /// *C style function: [ncpalette_set()][c_api::ncpalette_set].*
    pub fn set(&mut self, index: impl Into<NcPaletteIndex>, rgb: impl Into<NcRgb>) {
        c_api::ncpalette_set(self, index.into(), rgb.into().into())
    }
}
