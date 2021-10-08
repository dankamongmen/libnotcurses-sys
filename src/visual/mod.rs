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

#[allow(unused_imports)] // for the doc comments
use crate::{NcBlitter, NcChannel, NcDim, NcRgb};

mod methods;

/// Indicates how to scale an [`NcVisual`] during rendering.
///
/// - [`NCSCALE_NONE`] will apply no scaling.
/// - [`NCSCALE_SCALE`] scales a visual to the plane's size,
///   maintaining aspect ratio.
/// - [`NCSCALE_STRETCH`] stretches and scales the image in an
///   attempt to fill the entirety of the plane.
/// - [`NCSCALE_NONE_HIRES`] like `NCSCALE_NONE` admitting high-res blitters.
/// - [`NCSCALE_SCALE_HIRES`] like `NCSCALE_SCALE` admitting high-res blitters.
///
/// The `NCSCALE_*` preferences are applied only for the context of
/// [`NcVisual.render`][NcVisual#method.render]. You can think of it as a pipeline:
///
/// ```txt
/// NcVisual::fromfile() → frame → NcVisual.render() → scaling → output frame → blit
/// ```
///
/// where you still have the original frame. Whereas
/// [`NcVisual.resize`][NcVisual#method.resize] and
/// [`NcVisual.resize_noninterpolative`][NcVisual#method.resize_noninterpolative]
/// are changing that original frame.
///
pub type NcScale = crate::bindings::ffi::ncscale_e;

/// Maintains original size.
pub const NCSCALE_NONE: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_NONE;

/// Maintains aspect ratio.
pub const NCSCALE_SCALE: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_SCALE;

/// Throws away aspect ratio.
pub const NCSCALE_STRETCH: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_STRETCH;

/// Maintains original size, admitting high-resolution blitters
/// that don't preserve aspect ratio.
pub const NCSCALE_NONE_HIRES: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_NONE_HIRES;

/// Maintains aspect ratio, admitting high-resolution blitters
/// that don't preserve aspect ratio.
pub const NCSCALE_SCALE_HIRES: NcScale = crate::bindings::ffi::ncscale_e_NCSCALE_SCALE_HIRES;

/// A visual bit of multimedia.
///
/// It can be constructed from a rgba or bgra buffer.
///
/// The [NcVisualOptions] structure is used only by the following methods:
/// - [.geom][NcVisual#method.geom]
/// - [.render][NcVisual#method.render]
/// - [.simple_streamer][NcVisual#method.simple_streamer]
pub type NcVisual = crate::bindings::ffi::ncvisual;

/// A type alias of [`NcVisual`] (NcDirect ***F**rame*) intended to be used
/// with its `ncdirectf_*` methods, in [`NcDirect`][crate::NcDirect] mode.
pub type NcDirectF = NcVisual;

/// Describes all geometries of an [`NcVisual`] ncvisual–both those which are inherent, and
/// those in a given rendering regime.
///
/// *FIXME this ought be used in the rendered mode API as well;
/// it’s currently only used by direct mode.*
/// *(See [ncvgeom][1] more more information)*
///
/// This is the return type of the [NcDirectF.ncdirectf_geom()][0] method.
///
/// [0]: NcDirectF#method.ncdirectf_geom
/// [1]: crate::bindings::ffi::ncvgeom
pub type NcVGeom = crate::bindings::ffi::ncvgeom;

/// Options struct for [`NcVisual`]
///
/// If a plane is not provided, one will be created, having the exact size
/// necessary to display the visual (this might be smaller or larger than
/// the rendering area). if [`NCVISUAL_OPTION_CHILDPLANE`] is provided, this
/// will be interpreted as the parent.
///
/// A subregion of the visual can be rendered using `beg_x`, `beg_y`, `len_x`, and `len_y`.
pub type NcVisualOptions = crate::bindings::ffi::ncvisual_options;

// NcRgba
//
/// 32 bits broken into 3x 8bpp RGB channels + 8ppp alpha.
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
/// See also: [NcRgb] and [NcChannel] types.
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
// /// See also: [NcRgba], [NcRgb] and [NcChannel] types.
// pub type NcBgra = u32;

/// Treats as transparent the color specified in the `transcolor` field.
pub const NCVISUAL_OPTION_ADDALPHA: u32 = crate::bindings::ffi::NCVISUAL_OPTION_ADDALPHA;

/// Uses [`NCALPHA_BLEND`][crate::NCALPHA_BLEND] with visual.
pub const NCVISUAL_OPTION_BLEND: u32 = crate::bindings::ffi::NCVISUAL_OPTION_BLEND;

/// allows you to indicate that the n field of ncvisual_options refers not to
/// the plane onto which you'd like to blit, but the parent of a new plane.
///
/// A plane will be created using the other parameters in the ncvisual_options,
/// as a child of this parent. This means things like, say, vertically centering
/// a sprixel relative to the standard plane can be done in one step.
pub const NCVISUAL_OPTION_CHILDPLANE: u32 = crate::bindings::ffi::NCVISUAL_OPTION_CHILDPLANE;

/// Fails rather than gracefully degrade. See [`NcBlitter`].
pub const NCVISUAL_OPTION_NODEGRADE: u32 = crate::bindings::ffi::NCVISUAL_OPTION_NODEGRADE;

/// Y is an alignment, not absolute.
pub const NCVISUAL_OPTION_VERALIGNED: u32 = crate::bindings::ffi::NCVISUAL_OPTION_VERALIGNED;

/// X is an alignment, not absolute.
pub const NCVISUAL_OPTION_HORALIGNED: u32 = crate::bindings::ffi::NCVISUAL_OPTION_HORALIGNED;

/// Uses non-interpolative scaling.
pub const NCVISUAL_OPTION_NOINTERPOLATE: u32 = crate::bindings::ffi::NCVISUAL_OPTION_NOINTERPOLATE;

/// Contains the pixel geometry information as returned by the
/// NcPlane.[pixelgeom()][crate::NcPlane#method.pixelgeom] method.
///
/// If bitmaps are not supported, the fields `max_bitmap_*` will be 0.
#[derive(Clone, Debug)]
pub struct NcPixelGeometry {
    /// The height in pixels of the display region.
    pub term_y: NcDim,
    /// The width in pixels of the display region.
    pub term_x: NcDim,
    /// The height in pixels of a single cell.
    pub cell_y: NcDim,
    /// The width in pixels of a single cell.
    pub cell_x: NcDim,
    /// The height in pixels of the maximum displayable bitmap (0 if not supported).
    pub max_bitmap_y: NcDim,
    /// The width in pixels of the maximum displayable bitmap (0 if not supported).
    pub max_bitmap_x: NcDim,
}
