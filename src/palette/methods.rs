//! `NcPalette` methods and associated functions.

use crate::{c_api, error, Nc, NcChannel, NcComponent, NcPalette, NcPaletteIndex, NcResult, NcRgb};

impl NcPalette {
    /// New `NcPalette`.
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

    /// Attempts to configure the terminal with this NcPalette.
    ///
    /// *C style function: [ncpalette_use()][c_api::ncpalette_use].*
    pub fn r#use(&self, nc: &mut Nc) -> NcResult<()> {
        error![unsafe { c_api::ncpalette_use(nc, self) }]
    }

    /// Returns the [`NcComponent`]s from the [`NcChannel`] in this `NcPalette`.
    ///
    /// *C style function: [ncpalette_get_rgb()][c_api::ncpalette_get_rgb8].*
    pub fn get_rgb8(&self, index: NcPaletteIndex) -> (NcComponent, NcComponent, NcComponent) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncchannel_rgb8(self.chans[index as usize], &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Extracts the [`NcComponent`]s from an [`NcChannel`] entry inside
    /// this NcPalette, and returns the NcChannel.
    ///
    /// *C style function: [ncpalette_get_rgb()][c_api::ncpalette_get_rgb8].*
    pub fn get_rgb(&self, index: NcPaletteIndex) -> NcChannel {
        let (mut r, mut g, mut b) = (0, 0, 0);
        c_api::ncchannel_rgb8(self.chans[index as usize], &mut r, &mut g, &mut b)
    }

    /// Sets the [`NcRgb`] value of the [`NcChannel`][crate::NcChannel] entry
    /// inside this NcPalette.
    ///
    /// *C style function: [ncpalette_set()][c_api::ncpalette_set].*
    pub fn set(&mut self, index: NcPaletteIndex, rgb: NcRgb) {
        c_api::ncchannel_set(&mut self.chans[index as usize], rgb);
    }
}
