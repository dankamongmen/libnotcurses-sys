use crate::{NcBlitter, NcBlitterApi, NcDim};

/// Describes all the geometries of an [`NcVisual`], in a more Rust-friendly
/// struct than [`NcVGeom`] is.
///
/// It includes both inherent geometries and those which are dependent upon a
/// given rendering regime.
///
/// Inner values are calculated at the time of the call. A font change,
/// for example, could make all the fields invalid,
/// except for [`pix_yx`][Self#structfield.pix_yx].
///
/// This type is is returned by the [`NcVisual.geom`] method and by the
/// [`Nc.visual_geom`] method.
///
/// # See also
///
/// - [`NcPixelGeometry`][crate::NcPixelGeometry]
/// - [`NcVGeom`]
/// - [`NcVisualOptions`][crate::NcVisualOptions]
///
/// [`NcVisual`]: crate::NcVisual
/// [`ncdirectf_geom`]: crate::NcVisual#method.ncdirectf_geom
/// [`NcVisual.geom`]: crate::NcVisual#method.ncdirectf_geom
/// [`Nc.visual_geom`]: crate::Nc#method.visual_geom
#[derive(Debug, Clone, Copy, Default)]
pub struct NcVisualGeometry {
    /// The true internal pixel geometry of [`NcVisual`] data, taken directly
    /// from the load, and updated by `ncvisual_resize`.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`pixy`], [`pixx`]).*
    ///
    /// [`NcVisual`]: crate::NcVisual
    /// [`pixy`]: crate::c_api::ffi::ncvgeom#structfield.pixy
    /// [`pixx`]: crate::c_api::ffi::ncvgeom#structfield.pixx
    pub pix_yx: Option<(NcDim, NcDim)>,

    /// *Terminal* **cell** geometry at the time of the call.
    ///
    /// This can change with a font change, in which case all field values
    /// are invalidated (except for [`pix_yx`]).
    ///
    /// *Corresponds to the `NcVGeom` fields ([`cdimy`], [`cdimx`]).*
    ///
    /// [`pix_yx`]: Self#structfield.pix_yx
    /// [`cdimy`]: crate::c_api::ffi::ncvgeom#structfield.cdimy
    /// [`cdimx`]: crate::c_api::ffi::ncvgeom#structfield.cdimx
    pub cdim_yx: Option<(NcDim, NcDim)>,

    /// Rendered **pixel** geometry, per `NcVisualOptions`.
    /// as handed to the blitter, following any scaling.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`rpixy`], [`rpixx`]).*
    ///
    /// [`rpixy`]: crate::c_api::ffi::ncvgeom#structfield.rpixy
    /// [`rpixx`]: crate::c_api::ffi::ncvgeom#structfield.rpixx
    pub rpix_yx: Option<(NcDim, NcDim)>,

    /// Rendered **cell** geometry, per `NcVisualOptions`.
    ///
    /// As written by the blitter, following any padding (there is padding
    /// whenever `rpix{y, x}` is not evenly divided by `scale{y, x}`,
    /// and also sometimes for Sixel).
    ///
    /// *Corresponds to the `NcVGeom` fields ([`rcelly`], [`rcellx`]).*
    ///
    /// [`rcelly`]: crate::c_api::ffi::ncvgeom#structfield.rcelly
    /// [`rcellx`]: crate::c_api::ffi::ncvgeom#structfield.rcellx
    pub rcell_yx: Option<(NcDim, NcDim)>,

    /// The number of input pixels drawn to a single cell.
    ///
    /// When using `NcBlitter::PIXEL`, they are equivalent to [`cdim_yx`].
    ///
    /// *Corresponds to the `NcVGeom` fields ([`scaley`], [`scalex`]).*
    ///
    /// [`scaley`]: crate::c_api::ffi::ncvgeom#structfield.scaley
    /// [`scalex`]: crate::c_api::ffi::ncvgeom#structfield.scalex
    /// [`cdim_yx`]: Self#structfield.cdim_yx
    pub scale_yx: Option<(NcDim, NcDim)>,

    /// The largest bitmap geometry that the terminal is willing to accept.
    ///
    /// It is only defined when using `NcBlitter::PIXEL`.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`maxpixely`], [`maxpixelx`]).*
    ///
    /// [`maxpixely`]: crate::c_api::ffi::ncvgeom#structfield.maxmaxpixely
    /// [`maxpixelx`]: crate::c_api::ffi::ncvgeom#structfield.maxpixelx
    pub maxpixel_yx: Option<(NcDim, NcDim)>,

    /// The upper-left corner of the used section.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`begy`], [`begx`]), and the
    /// `NcVisualOptions` fields ([`begy`][vo#begy], [`begx`][vo#begx]).*
    ///
    /// [`begy`]: crate::c_api::ffi::ncvgeom#structfield.begy
    /// [`begx`]: crate::c_api::ffi::ncvgeom#structfield.begx
    /// [vo#begx]: crate::c_api::ffi::ncvisual_options#structfield.begx
    /// [vo#begy]: crate::c_api::ffi::ncvisual_options#structfield.begy
    pub beg_yx: Option<(NcDim, NcDim)>,

    /// The geometry of the used section.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`leny`], [`lenx`]), and the
    /// `NcVisualOptions` fields ([`leny`][vo#leny], [`lenx`][vo#lenx]).*
    ///
    /// [`leny`]: crate::c_api::ffi::ncvgeom#structfield.leny
    /// [`lenx`]: crate::c_api::ffi::ncvgeom#structfield.lenx
    /// [vo#lenx]: crate::c_api::ffi::ncvisual_options#structfield.lenx
    /// [vo#leny]: crate::c_api::ffi::ncvisual_options#structfield.leny
    pub len_yx: Option<(NcDim, NcDim)>,

    /// The [`NcBlitter`] which will be used.
    ///
    /// A function of the requested blitter and the blitters actually supported
    /// by this environment.
    pub blitter: NcBlitter,
}

impl NcVisualGeometry {
    /// The name of the blitter which will be used.
    ///
    /// A function of the requested blitter and the blitters actually supported
    /// by this environment.
    pub fn blitter_name(&self) -> String {
        crate::Nc::str_blitter(self.blitter)
    }
}

/// # Getter methods for the `NcVGeom` fields
///
/// Each of the following methods return the corresponding [`NcVGeom`] field
/// with the same name.
///
impl NcVisualGeometry {
    pub fn pixy(&self) -> NcDim {
        self.pix_yx.unwrap_or((0, 0)).0
    }
    pub fn pixx(&self) -> NcDim {
        self.pix_yx.unwrap_or((0, 0)).1
    }
    pub fn cdimy(&self) -> NcDim {
        self.cdim_yx.unwrap_or((0, 0)).0
    }
    pub fn cdimx(&self) -> NcDim {
        self.cdim_yx.unwrap_or((0, 0)).1
    }
    pub fn rpixy(&self) -> NcDim {
        self.rpix_yx.unwrap_or((0, 0)).0
    }
    pub fn rpixx(&self) -> NcDim {
        self.rpix_yx.unwrap_or((0, 0)).1
    }
    pub fn rcelly(&self) -> NcDim {
        self.rcell_yx.unwrap_or((0, 0)).0
    }
    pub fn rcellx(&self) -> NcDim {
        self.rcell_yx.unwrap_or((0, 0)).1
    }
    pub fn scaley(&self) -> NcDim {
        self.scale_yx.unwrap_or((0, 0)).0
    }
    pub fn scalex(&self) -> NcDim {
        self.scale_yx.unwrap_or((0, 0)).1
    }
    pub fn maxpixely(&self) -> NcDim {
        self.maxpixel_yx.unwrap_or((0, 0)).0
    }
    pub fn maxpixelx(&self) -> NcDim {
        self.maxpixel_yx.unwrap_or((0, 0)).1
    }
    pub fn begy(&self) -> NcDim {
        self.beg_yx.unwrap_or((0, 0)).0
    }
    pub fn begx(&self) -> NcDim {
        self.beg_yx.unwrap_or((0, 0)).1
    }
    pub fn leny(&self) -> NcDim {
        self.len_yx.unwrap_or((0, 0)).0
    }
    pub fn lenx(&self) -> NcDim {
        self.len_yx.unwrap_or((0, 0)).1
    }
}

/// Describes all geometries of an [`NcVisual`]:
/// those which are inherent, and those dependent upon a given rendering regime.
///
/// The inner values are calculated at the time of the call, and a font change
/// could make all the fields invalid, except for `pixx`/`pixy`.
///
/// This type is created by the [`ncvisual_geom`] & [`ncdirectf_geom`] functions.
///
/// # See also
/// - [`NcVisualGeometry`][crate::NcVisualGeometry].
///
/// [`NcVisual`]: crate::NcVisual
/// [`ncdirectf_geom`]: crate::c_api::ncdirectf_geom
/// [`ncvisual_geom`]: crate::c_api::ncvisual_geom
pub type NcVGeom = crate::bindings::ffi::ncvgeom;

impl From<NcVGeom> for NcVisualGeometry {
    /// Since we don't know the origin of the `NcVGeom` struct, when some fields
    /// are 0, we can't really know whether that's a valid value or not.
    /// That should be determined manually by other means.
    ///
    /// Specifically [`len_yx`], [`beg_yx`] & [`maxpixel_yx`] wont be `None`
    /// even if the corresponding `NcVGeom` fields are 0. But they would be
    /// `None` if the NcVisualGeometry had been created by the
    ///
    /// [`len_yx`]: Self#structfield.len_yx
    /// [`beg_yx`]: Self#structfield.beg_yx
    /// [`maxpixel_yx`]: Self#structfield.maxpixel_yx
    /// See: <https://github.com/dankamongmen/notcurses/pull/2320#issuecomment-962170075>
    fn from(vg: NcVGeom) -> Self {
        // The following values following values can't have a valid 0 value:

        let (pix_yx, cdim_yx, rpix_yx, rcell_yx, scale_yx, maxpixel_yx);

        if vg.pixy == 0 || vg.pixx == 0 {
            pix_yx = None;
        } else {
            pix_yx = Some((vg.pixy as NcDim, vg.pixx as NcDim));
        }
        if vg.cdimy == 0 || vg.cdimx == 0 {
            cdim_yx = None;
        } else {
            cdim_yx = Some((vg.cdimy as NcDim, vg.cdimx as NcDim));
        }
        if vg.rpixy == 0 || vg.rpixx == 0 {
            // MAYBE double CHECK this case
            rpix_yx = None;
        } else {
            rpix_yx = Some((vg.rpixy as NcDim, vg.rpixx as NcDim));
        }
        if vg.rcelly == 0 || vg.rcellx == 0 {
            rcell_yx = None;
        } else {
            rcell_yx = Some((vg.rcelly as NcDim, vg.rcellx as NcDim));
        }
        if vg.scaley == 0 || vg.scalex == 0 {
            scale_yx = None;
        } else {
            scale_yx = Some((vg.scaley as NcDim, vg.scalex as NcDim));
        }

        // maxpixel_yx is only defined when using NcBlitter::PIXEL
        if vg.blitter == NcBlitter::PIXEL {
            maxpixel_yx = Some((vg.maxpixely as NcDim, vg.maxpixelx as NcDim));
        } else {
            maxpixel_yx = None;
        }

        NcVisualGeometry {
            pix_yx,
            cdim_yx,
            rpix_yx,
            rcell_yx,
            scale_yx,
            maxpixel_yx,

            len_yx: Some((vg.leny as NcDim, vg.lenx as NcDim)),
            beg_yx: Some((vg.begy as NcDim, vg.begx as NcDim)),

            blitter: vg.blitter,
        }
    }
}
