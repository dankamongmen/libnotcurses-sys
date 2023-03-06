//!

#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::NcBlitter;

/// Describes all the geometries of an [`NcVisual`].
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
/// - [`NcVisualOptions`][crate::NcVisualOptions]
/// - [`NcVGeom`][crate::c_api::NcVGeom]
///
/// [`NcVisual`]: crate::NcVisual
/// [`ncdirectf_geom`]: crate::NcVisual#method.ncdirectf_geom
/// [`NcVisual.geom`]: crate::NcVisual#method.ncdirectf_geom
/// [`Nc.visual_geom`]: crate::Nc#method.visual_geom
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NcVisualGeometry {
    /// The true internal pixel geometry of [`NcVisual`] data, taken directly
    /// from the load, and updated by `ncvisual_resize`.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`pixy`], [`pixx`]).*
    ///
    /// [`NcVisual`]: crate::NcVisual
    /// [`pixy`]: crate::c_api::ffi::ncvgeom#structfield.pixy
    /// [`pixx`]: crate::c_api::ffi::ncvgeom#structfield.pixx
    pub pix_yx: Option<(u32, u32)>,

    /// *Terminal* **cell** geometry at the time of the call. This is the size
    /// of a cell in pixels.
    ///
    /// This can change with a font change, in which case all field values
    /// are invalidated (except for [`pix_yx`]).
    ///
    /// *Corresponds to the `NcVGeom` fields ([`cdimy`], [`cdimx`]), and to the
    /// `NcPixelGeometry` fields ([`cell_y`], [`cell_x`])*
    ///
    /// [`pix_yx`]: Self#structfield.pix_yx
    /// [`cdimy`]: crate::c_api::ffi::ncvgeom#structfield.cdimy
    /// [`cdimx`]: crate::c_api::ffi::ncvgeom#structfield.cdimx
    /// [`cell_y`]: crate::NcPixelGeometry#structfield.cell_y
    /// [`cell_x`]: crate::NcPixelGeometry#structfield.cell_x
    pub cdim_yx: Option<(u32, u32)>,

    /// Rendered **pixel** geometry, per `NcVisualOptions`.
    /// as handed to the blitter, following any scaling.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`rpixy`], [`rpixx`]).*
    ///
    /// [`rpixy`]: crate::c_api::ffi::ncvgeom#structfield.rpixy
    /// [`rpixx`]: crate::c_api::ffi::ncvgeom#structfield.rpixx
    pub rpix_yx: Option<(u32, u32)>,

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
    pub rcell_yx: Option<(u32, u32)>,

    /// The number of input pixels drawn to a single cell.
    ///
    /// When using `NcBlitter::Pixel`, they are equivalent to [`cdim_yx`].
    ///
    /// *Corresponds to the `NcVGeom` fields ([`scaley`], [`scalex`]).*
    ///
    /// [`scaley`]: crate::c_api::ffi::ncvgeom#structfield.scaley
    /// [`scalex`]: crate::c_api::ffi::ncvgeom#structfield.scalex
    /// [`cdim_yx`]: Self#structfield.cdim_yx
    pub scale_yx: Option<(u32, u32)>,

    /// The largest bitmap geometry that the terminal is willing to accept.
    ///
    /// It is only defined when using `NcBlitter::Pixel`.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`maxpixely`], [`maxpixelx`]).*
    ///
    /// [`maxpixely`]: crate::c_api::ffi::ncvgeom#structfield.maxmaxpixely
    /// [`maxpixelx`]: crate::c_api::ffi::ncvgeom#structfield.maxpixelx
    pub maxpixel_yx: Option<(u32, u32)>,

    /// The upper-left corner of the used section.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`begy`], [`begx`]), and the
    /// `NcVisualOptions` fields ([`begy`][vo#begy], [`begx`][vo#begx]).*
    ///
    /// [`begy`]: crate::c_api::ffi::ncvgeom#structfield.begy
    /// [`begx`]: crate::c_api::ffi::ncvgeom#structfield.begx
    /// [vo#begx]: crate::c_api::ffi::ncvisual_options#structfield.begx
    /// [vo#begy]: crate::c_api::ffi::ncvisual_options#structfield.begy
    pub beg_yx: Option<(u32, u32)>,

    /// The geometry of the used section.
    ///
    /// *Corresponds to the `NcVGeom` fields ([`leny`], [`lenx`]), and the
    /// `NcVisualOptions` fields ([`leny`][vo#leny], [`lenx`][vo#lenx]).*
    ///
    /// [`leny`]: crate::c_api::ffi::ncvgeom#structfield.leny
    /// [`lenx`]: crate::c_api::ffi::ncvgeom#structfield.lenx
    /// [vo#lenx]: crate::c_api::ffi::ncvisual_options#structfield.lenx
    /// [vo#leny]: crate::c_api::ffi::ncvisual_options#structfield.leny
    pub len_yx: Option<(u32, u32)>,

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
/// [`NcVGeom`]: crate::c_api::NcVGeom
///
impl NcVisualGeometry {
    pub fn pixy(&self) -> u32 {
        self.pix_yx.unwrap_or((0, 0)).0
    }
    pub fn pixx(&self) -> u32 {
        self.pix_yx.unwrap_or((0, 0)).1
    }
    pub fn cdimy(&self) -> u32 {
        self.cdim_yx.unwrap_or((0, 0)).0
    }
    pub fn cdimx(&self) -> u32 {
        self.cdim_yx.unwrap_or((0, 0)).1
    }
    pub fn rpixy(&self) -> u32 {
        self.rpix_yx.unwrap_or((0, 0)).0
    }
    pub fn rpixx(&self) -> u32 {
        self.rpix_yx.unwrap_or((0, 0)).1
    }
    pub fn rcelly(&self) -> u32 {
        self.rcell_yx.unwrap_or((0, 0)).0
    }
    pub fn rcellx(&self) -> u32 {
        self.rcell_yx.unwrap_or((0, 0)).1
    }
    pub fn scaley(&self) -> u32 {
        self.scale_yx.unwrap_or((0, 0)).0
    }
    pub fn scalex(&self) -> u32 {
        self.scale_yx.unwrap_or((0, 0)).1
    }
    pub fn maxpixely(&self) -> u32 {
        self.maxpixel_yx.unwrap_or((0, 0)).0
    }
    pub fn maxpixelx(&self) -> u32 {
        self.maxpixel_yx.unwrap_or((0, 0)).1
    }
    pub fn begy(&self) -> u32 {
        self.beg_yx.unwrap_or((0, 0)).0
    }
    pub fn begx(&self) -> u32 {
        self.beg_yx.unwrap_or((0, 0)).1
    }
    pub fn leny(&self) -> u32 {
        self.len_yx.unwrap_or((0, 0)).0
    }
    pub fn lenx(&self) -> u32 {
        self.len_yx.unwrap_or((0, 0)).1
    }
}

mod std_impls {
    use super::{c_api::NcVGeom, NcBlitter, NcVisualGeometry};

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
                pix_yx = Some((vg.pixy, vg.pixx));
            }
            if vg.cdimy == 0 || vg.cdimx == 0 {
                cdim_yx = None;
            } else {
                cdim_yx = Some((vg.cdimy, vg.cdimx));
            }
            if vg.rpixy == 0 || vg.rpixx == 0 {
                // MAYBE double CHECK this case
                rpix_yx = None;
            } else {
                rpix_yx = Some((vg.rpixy, vg.rpixx));
            }
            if vg.rcelly == 0 || vg.rcellx == 0 {
                rcell_yx = None;
            } else {
                rcell_yx = Some((vg.rcelly, vg.rcellx));
            }
            if vg.scaley == 0 || vg.scalex == 0 {
                scale_yx = None;
            } else {
                scale_yx = Some((vg.scaley, vg.scalex));
            }

            // maxpixel_yx is only defined when using NcBlitter::Pixel
            if vg.blitter == NcBlitter::Pixel.into() {
                maxpixel_yx = Some((vg.maxpixely, vg.maxpixelx));
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

                len_yx: Some((vg.leny, vg.lenx)),
                beg_yx: Some((vg.begy, vg.begx)),

                blitter: vg.blitter.into(),
            }
        }
    }
}

pub(crate) mod c_api {
    use crate::{c_api::ffi, NcBlitter};

    /// Describes all geometries of an [`NcVisual`].
    ///
    /// Both those which are inherent,
    /// and those dependent upon a given rendering regime.
    ///
    /// It's recommended to use [`NcVisualGeometry`] instead.
    ///
    /// The inner values are calculated at the time of the call, and a font
    /// change could make all the fields invalid, except for `pixx`/`pixy`.
    ///
    /// This type is created by the [`ncvisual_geom`] & [`ncdirectf_geom`]
    /// functions.
    ///
    /// [`NcVisual`]: crate::NcVisual
    /// [`NcVisualGeometry`]: crate::NcVisualGeometry
    /// [`ncdirectf_geom`]: crate::c_api::ncdirectf_geom
    /// [`ncvisual_geom`]: crate::c_api::ncvisual_geom
    pub type NcVGeom = ffi::ncvgeom;

    /// # Constructors
    impl NcVGeom {
        /// Returns a new `NcVGeom` with zeroed fields.
        pub fn new() -> Self {
            Self {
                pixy: 0,
                pixx: 0,
                cdimy: 0,
                cdimx: 0,
                rpixy: 0,
                rpixx: 0,
                rcelly: 0,
                rcellx: 0,
                scaley: 0,
                scalex: 0,
                maxpixely: 0,
                maxpixelx: 0,
                begy: 0,
                begx: 0,
                leny: 0,
                lenx: 0,
                blitter: NcBlitter::Default.into(),
            }
        }
    }
}
