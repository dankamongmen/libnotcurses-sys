//! `ncpalette_*` reimplemented functions.

use crate::{c_api, NcChannel, NcPalette, NcPaletteIndex, NcRgb};

/// Extracts the RGB components from an [`NcChannel`] entry inside
/// an [`NcPalette`], and returns the `NcChannel`.
///
/// *Method: NcPalette.[get_rgb()][NcPalette#method.get_rgb].*
/// *Method: NcPalette.[get_rgb8()][NcPalette#method.get_rgb8].*
#[inline]
pub fn ncpalette_get_rgb8(
    palette: &NcPalette,
    index: NcPaletteIndex,
    red: &mut u8,
    green: &mut u8,
    blue: &mut u8,
) -> NcChannel {
    c_api::ncchannel_rgb8(palette.chans[index as usize], red, green, blue)
}

/// Sets the [`NcRgb`] value of the [`NcChannel`] entry inside an [`NcPalette`].
///
/// *Method: NcPalette.[set()][NcPalette#method.set].*
#[inline]
pub fn ncpalette_set(palette: &mut NcPalette, index: NcPaletteIndex, rgb: NcRgb) {
    c_api::ncchannel_set(&mut palette.chans[index as usize], rgb);
}

/// Sets the RGB components of the [`NcChannel`] entry inside an
/// [`NcPalette`].
///
/// *Method: NcPalette.[set_rgb()][NcPalette#method.set_rgb].*
#[inline]
pub fn ncpalette_set_rgb8(
    palette: &mut NcPalette,
    index: NcPaletteIndex,
    red: u8,
    green: u8,
    blue: u8,
) {
    c_api::ncchannel_set_rgb8(&mut palette.chans[index as usize], red, green, blue)
}
