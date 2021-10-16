// functions already exported by bindgen : 27
// -----------------------------------------
// (W) wrap: 23
// (#) test: 0
// -----------------------------------------
//W  ncdirectf_free
//W  ncdirectf_from_file
//   ncdirectf_geom
//   ncdirectf_render
//W  ncvisual_at_yx
//W  ncvisual_decode
//W  ncvisual_decode_loop
//W  ncvisual_destroy
//W  ncvisual_from_bgra
//W  ncvisual_from_file
//W  ncvisual_from_palidx
//W  ncvisual_from_plane
//W  ncvisual_from_rgba
//W  ncvisual_from_rgb_packed
//W  ncvisual_from_rgb_loose
//W  ncvisual_blitter_geom
//W  ncvisual_media_defblitter
//W  ncvisual_polyfill_yx
//   ncvisual_plane_create
//W  ncvisual_render
//W  ncvisual_resize
//W  ncvisual_rotate
//W  ncvisual_set_yx
//W  ncvisual_simple_streamer
//~  ncvisual_stream
//W  ncvisual_subtitle
//W  ncvisual_subtitle_plane

#[allow(unused_imports)] // for doc comments
use crate::{NcChannel, NcRgb};

mod methods;

/// A visual bit of multimedia.
///
/// It can be constructed from a rgba or bgra buffer.
///
/// The [`NcVisualOptions`] structure is used only by the following methods:
/// - [`geom`][NcVisual#method.geom]
/// - [`render`][NcVisual#method.render]
/// - [`simple_streamer`][NcVisual#method.simple_streamer]
pub type NcVisual = crate::bindings::ffi::ncvisual;

/// Describes all geometries of an [`NcVisual`] ncvisual–both those which are inherent, and
/// those in a given rendering regime.
///
/// *FIXME this ought be used in the rendered mode API as well;
/// it’s currently only used by direct mode.*
/// *(See [`ncvgeom`][1] more more information)*
///
/// This is the return type of the [`NcDirectF.ncdirectf_geom`][0] method.
///
/// [0]: NcVisual#method.ncdirectf_geom
/// [1]: crate::bindings::ffi::ncvgeom
pub type NcVGeom = crate::bindings::ffi::ncvgeom;

/// Options struct for [`NcVisual`]
///
/// If a plane is not provided, one will be created, having the exact size
/// necessary to display the visual (this might be smaller or larger than
/// the rendering area). if
/// [`NcVisualOptions::CHILDPLANE`][NcVisualOptions#associatedconstant.CHILDPLANE]
/// is provided, this will be interpreted as the parent.
///
/// A subregion of the visual can be rendered using `beg_y`, `beg_x`, `len_y`,
/// and `len_x`.
pub type NcVisualOptions = crate::bindings::ffi::ncvisual_options;

// NcRgba
//
/// 32 bits broken into 3x 8bpp RGB channels + 8ppp alpha (alias of [`u32`]).
///
/// Unlike with [`NcChannel`], operations involving `NcRgb` ignores the last 4th byte
///
/// ## Diagram
///
/// ```txt
/// AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
/// `type in C: no data type`
///
/// See also: [`NcRgb`] and [`NcChannel`] types.
pub type NcRgba = u32;

// // NcBgra
// //
// /// 32 bits broken into 3x 8bpp BGR channels + 8ppp alpha.
// ///
// /// ## Diagram
// ///
// /// ```txt
// /// AAAAAAAA BBBBBBBB GGGGGGGG RRRRRRRR
// /// ```
// ///
// /// `type in C: no data type`
// ///
// /// See also: [`NcRgba`], [`NcRgb`] and [`NcChannel`] types.
// pub type NcBgra = u32;

impl NcVisualOptions {
    /// Treats as transparent the color specified in the `transcolor` field.
    pub const ADDALPHA: u32 = constants::NCVISUAL_OPTION_ADDALPHA;

    /// Uses [`NcAlpha::Blend`][crate::NcAlpha#associatedconstant.BLEND] with visual.
    pub const BLEND: u32 = constants::NCVISUAL_OPTION_BLEND;

    /// allows you to indicate that the n field of ncvisual_options refers not to
    /// the plane onto which you'd like to blit, but the parent of a new plane.
    ///
    /// A plane will be created using the other parameters in the ncvisual_options,
    /// as a child of this parent. This means things like, say, vertically centering
    /// a sprixel relative to the standard plane can be done in one step.
    pub const CHILDPLANE: u32 = constants::NCVISUAL_OPTION_CHILDPLANE;

    /// Fails rather than gracefully degrade. See [`NcBlitter`][crate::NcBlitter].
    pub const NODEGRADE: u32 = constants::NCVISUAL_OPTION_NODEGRADE;

    /// Y is an alignment, not absolute.
    pub const VERALIGNED: u32 = constants::NCVISUAL_OPTION_VERALIGNED;

    /// X is an alignment, not absolute.
    pub const HORALIGNED: u32 = constants::NCVISUAL_OPTION_HORALIGNED;

    /// Uses non-interpolative scaling.
    pub const NOINTERPOLATE: u32 = constants::NCVISUAL_OPTION_NOINTERPOLATE;
}

pub(crate) mod constants {
    /// Treats as transparent the color specified in the `transcolor` field.
    pub const NCVISUAL_OPTION_ADDALPHA: u32 = crate::bindings::ffi::NCVISUAL_OPTION_ADDALPHA;

    /// Uses [`NcAlpha::Blend`][crate::NcAlpha#associatedconstant.BLEND] with visual.
    pub const NCVISUAL_OPTION_BLEND: u32 = crate::bindings::ffi::NCVISUAL_OPTION_BLEND;

    /// allows you to indicate that the n field of ncvisual_options refers not to
    /// the plane onto which you'd like to blit, but the parent of a new plane.
    ///
    /// A plane will be created using the other parameters in the ncvisual_options,
    /// as a child of this parent. This means things like, say, vertically centering
    /// a sprixel relative to the standard plane can be done in one step.
    pub const NCVISUAL_OPTION_CHILDPLANE: u32 = crate::bindings::ffi::NCVISUAL_OPTION_CHILDPLANE;

    /// Fails rather than gracefully degrade. See [`NcBlitter`][crate::NcBlitter].
    pub const NCVISUAL_OPTION_NODEGRADE: u32 = crate::bindings::ffi::NCVISUAL_OPTION_NODEGRADE;

    /// Y is an alignment, not absolute.
    pub const NCVISUAL_OPTION_VERALIGNED: u32 = crate::bindings::ffi::NCVISUAL_OPTION_VERALIGNED;

    /// X is an alignment, not absolute.
    pub const NCVISUAL_OPTION_HORALIGNED: u32 = crate::bindings::ffi::NCVISUAL_OPTION_HORALIGNED;

    /// Uses non-interpolative scaling.
    pub const NCVISUAL_OPTION_NOINTERPOLATE: u32 =
        crate::bindings::ffi::NCVISUAL_OPTION_NOINTERPOLATE;
}
