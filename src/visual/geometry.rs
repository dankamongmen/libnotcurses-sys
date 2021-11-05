use crate::NcDim;

#[allow(unused_imports)]
use crate::{NcBlitter, NcVisual};

/// Describes all geometries of an [`NcVisual`]:
/// those which are inherent, and those dependent upon a given rendering regime.
///
/// The inner values are calculated at the time of the call, and a font change
/// could make all the fields invalid, except for `pixx`/`pixy`.
///
/// This type is returned by the [`geom`] & [`ncdirectf_geom`] methods from `NcVisual`.
///
/// See also [`NcPixelGeometry`][crate::NcPixelGeometry].
///
/// [`ncdirectf_geom`]: NcVisual#method.ncdirectf_geom
/// [`geom`]: NcVisual#method.ncdirectf_geom
pub type NcVGeom = crate::bindings::ffi::ncvgeom;

impl NcVGeom {
    /// The true internal pixel geometry of [`NcVisual`] data, taken directly
    /// from the load (and updated by `ncvisual_resize`).
    ///
    /// Corresponds to the inner fields ([`pixy`], [`pixx`]).
    ///
    /// [`pixy`]: crate::c_api::ffi::ncvgeom#structfield.pixy
    /// [`pixx`]: crate::c_api::ffi::ncvgeom#structfield.pixx
    pub fn pix_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.pixy == 0 || self.pixx == 0 {
            None
        } else {
            Some((self.pixy as u32, self.pixx as u32))
        }
    }

    /// *Terminal* **cell** geometry at the time of the call.
    ///
    /// It can change with a font change, in which case all values other than
    /// pixy/pixx are invalidated.
    ///
    /// Corresponds to the inner fields ([`cdimy`], [`cdimx`]).
    ///
    /// [`cdimy`]: crate::c_api::ffi::ncvgeom#structfield.cdimy
    /// [`cdimx`]: crate::c_api::ffi::ncvgeom#structfield.cdimx
    pub fn cdim_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.cdimy == 0 || self.cdimx == 0 {
            None
        } else {
            Some((self.cdimy as u32, self.cdimx as u32))
        }
    }

    /// Rendered **pixel** geometry, per [`NcVisualOptions`][crate::NcVisualOptions].
    /// as handed to the blitter, following any scaling.
    ///
    /// Corresponds to the inner fields ([`rpixy`], [`rpixx`]).
    ///
    /// [`rpixy`]: crate::c_api::ffi::ncvgeom#structfield.rpixy
    /// [`rpixx`]: crate::c_api::ffi::ncvgeom#structfield.rpixx
    pub fn rpix_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.rpixy == 0 || self.rpixx == 0 {
            None
        } else {
            Some((self.rpixy as u32, self.rpixx as u32))
        }
    }

    /// Rendered **cell** geometry, per [`NcVisualOptions`][crate::NcVisualOptions].
    ///
    /// As written by the blitter, following any padding (there is padding
    /// whenever `rpix{y, x}` is not evenly divided by `scale{y, x}`,
    /// and also sometimes for Sixel).
    ///
    /// Corresponds to the inner fields ([`rcelly`], [`rcellx`]).
    ///
    /// [`rcelly`]: crate::c_api::ffi::ncvgeom#structfield.rcelly
    /// [`rcellx`]: crate::c_api::ffi::ncvgeom#structfield.rcellx
    pub fn rcell_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.rcelly == 0 || self.rcellx == 0 {
            None
        } else {
            Some((self.rcelly as u32, self.rcellx as u32))
        }
    }

    /// The number of input pixels drawn to a single cell.
    ///
    /// When using [`NcBlitter::PIXEL`], they are equivalent to `cdimy`/`cdimx`.
    ///
    /// Corresponds to the inner fields ([`scaley`], [`scalex`]).
    ///
    /// [`NCBlitter::PIXEL`]: NcBlitter#associatedconstant.PIXEL
    /// [`scaley`]: crate::c_api::ffi::ncvgeom#structfield.scaley
    /// [`scalex`]: crate::c_api::ffi::ncvgeom#structfield.scalex
    pub fn scale_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.scaley == 0 || self.scalex == 0 {
            None
        } else {
            Some((self.scaley as u32, self.scalex as u32))
        }
    }

    /// The largest bitmap geometry that the terminal is willing to accept.
    ///
    /// Only defined when using [`NcBlitter::PIXEL`].
    ///
    /// Corresponds to the inner fields ([`maxpixely`], [`maxpixelx`]).
    ///
    /// [`NCBlitter::PIXEL`]: NcBlitter#associatedconstant.PIXEL
    /// [`maxpixely`]: crate::c_api::ffi::ncvgeom#structfield.maxmaxpixely
    /// [`maxpixelx`]: crate::c_api::ffi::ncvgeom#structfield.maxpixelx
    pub fn maxpixel_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.maxpixely == 0 || self.maxpixelx == 0 {
            None
        } else {
            Some((self.maxpixely as u32, self.maxpixelx as u32))
        }
    }

    /// The upper-left corner of the used section.
    ///
    /// Corresponds to the inner fields ([`begy`], [`begx`]).
    ///
    /// [`begy`]: crate::c_api::ffi::ncvgeom#structfield.begy
    /// [`begx`]: crate::c_api::ffi::ncvgeom#structfield.begx
    pub fn beg_yx(&self) -> Option<(NcDim, NcDim)> {
        // TODO:CHECK & MAYBE:FIX
        if self.begy == 0 || self.begx == 0 {
            None
        } else {
            Some((self.begy as u32, self.begx as u32))
        }
    }

    /// The geometry of the used section.
    ///
    /// Corresponds to the inner fields ([`leny`], [`lenx`]).
    ///
    /// [`leny`]: crate::c_api::ffi::ncvgeom#structfield.leny
    /// [`lenx`]: crate::c_api::ffi::ncvgeom#structfield.lenx
    pub fn len_yx(&self) -> Option<(NcDim, NcDim)> {
        if self.leny == 0 || self.lenx == 0 {
            None
        } else {
            Some((self.leny as u32, self.lenx as u32))
        }
    }

    /// The [`NcBlitter`] which will be used.
    ///
    /// A function of the requested blitter and the blitters actually supported
    /// by this environment.
    pub fn blitter(&self) -> NcBlitter {
        self.blitter
    }

    /// The name of the blitter which will be used.
    ///
    /// A function of the requested blitter and the blitters actually supported
    /// by this environment.
    pub fn blitter_name(&self) -> String {
        crate::Nc::str_blitter(self.blitter)
    }
}
