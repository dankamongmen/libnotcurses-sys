//! `ncpalette_*` reimplemented functions.

use crate::{
    c_api::{self, NcChannel_u32, NcRgb_u32},
    NcPalette, NcPaletteIndex,
};

/// Returns the [`NcRgb_u32`] value from an [`NcChannel_u32`] entry inside
/// an [`NcPalette`].
///
/// *Method: NcPalette.[get()][NcPalette#method.get].*
#[inline]
pub fn ncpalette_get(palette: &NcPalette, index: impl Into<NcPaletteIndex>) -> NcRgb_u32 {
    c_api::ncchannel_rgb(palette.chans[index.into() as usize])
}

/// Extracts the RGB components from an [`NcChannel_u32`] entry inside
/// an [`NcPalette`], and returns the `NcChannel_u32`.
///
/// *Method: NcPalette.[get_rgb8()][NcPalette#method.get_rgb8].*
#[inline]
pub fn ncpalette_get_rgb8(
    palette: &NcPalette,
    index: impl Into<NcPaletteIndex>,
    red: &mut u8,
    green: &mut u8,
    blue: &mut u8,
) -> NcChannel_u32 {
    c_api::ncchannel_rgb8(palette.chans[index.into() as usize], red, green, blue)
}

/// Sets the [`NcRgb_u32`] value of an [`NcChannel_u32`] entry inside an
/// [`NcPalette`].
///
/// *Method: NcPalette.[set()][NcPalette#method.set].*
#[inline]
pub fn ncpalette_set(palette: &mut NcPalette, index: impl Into<NcPaletteIndex>, rgb: NcRgb_u32) {
    c_api::ncchannel_set(&mut palette.chans[index.into() as usize], rgb);
}

/// Sets the RGB components of an [`NcChannel_u32`] entry inside an
/// [`NcPalette`].
///
/// *Method: NcPalette.[set_rgb8()][NcPalette#method.set_rgb8].*
#[inline]
pub fn ncpalette_set_rgb8(
    palette: &mut NcPalette,
    index: impl Into<NcPaletteIndex>,
    red: u8,
    green: u8,
    blue: u8,
) {
    c_api::ncchannel_set_rgb8(&mut palette.chans[index.into() as usize], red, green, blue)
}
