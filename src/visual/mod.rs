// functions already exported by bindgen : 24
// -----------------------------------------
// (W) wrap: 20
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
//W  ncvisual_media_defblitter
//W  ncvisual_polyfill_yx
//   ncvisual_plane_create
//W  ncvisual_resize
//W  ncvisual_rotate
//W  ncvisual_set_yx
//W  ncvisual_simple_streamer
//~  ncvisual_stream
//W  ncvisual_subtitle_plane

// functions manually reimplemented: 45
// ------------------------------------------
// (+) done: 1
// (W) wrap: 1
// (#) test: 0
// ------------------------------------------
//W+ ncvisualplane_create

#[allow(unused_imports)] // for doc comments
use crate::{NcBlitter, NcChannel, NcDim, NcOffset, NcPlane, NcRgb, NcScale};

mod methods;
mod reimplemented;

mod geometry;
pub use geometry::{NcVGeom, NcVisualGeometry};

/// A visual bit of multimedia.
///
/// It can be constructed from a rgba or bgra buffer.
///
/// The [`NcVisualOptions`] structure is used only by the following methods:
/// - [`geom`][NcVisual#method.geom]
/// - [`render`][NcVisual#method.render]
/// - [`simple_streamer`][NcVisual#method.simple_streamer]
pub type NcVisual = crate::bindings::ffi::ncvisual;

/// Options struct for [`NcVisual`].
///
/// It is recommended to use the [`NcVisualOptions::builder()`] method.
///
/// If a plane is not provided, one will be created, having the exact size
/// necessary to display the visual (this might be smaller or larger than
/// the rendering area). if [`NcVisualOptions::CHILDPLANE`] is provided,
/// this will be interpreted as the parent.
///
/// A subregion of the visual can be rendered using `beg_y`, `beg_x`, `len_y`,
/// and `len_x`.
///
/// # Fields
///
/// * [`n`] - an optional mutable pointer to an [`NcPlane`].
///
/// * [`scaling`] - how the source will be stretched/scaled relative to the
///   `NcPlane` ([`NOSCALE`], [`SCALE`], [`STRETCH`], [`NONE_HIRES`],
///   [`SCALE_HIRES`]).
///
/// * [`y`] - if an `NcPlane` is provided in `n` then this specifies where the
///   `NcVisual` will be on that plane.
///
///   Otherwise it specifies where the created `NcPlane` will be placed relative
///   to the standard plane's origin.
///
///   If [`VERALIGNED`] is set, this will be interpreted as an [`NcAlign`] value.
///
/// * [`x`] - if an `NcPlane` is provided in `n` then this specifies where the
///   `NcVisual` will be on that plane.
///
///   Otherwise it specifies where the created `NcPlane` will be placed relative
///   to the standard plane's origin.
///
///   If [`HORALIGNED`] is set, this will be interpreted as an [`NcAlign`] value.
///
/// * [`begy`] - origin of rendered section in the *y* axis.
/// * [`begx`] - origin of rendered section in the *x* axis.
/// * [`leny`] - length of rendered section in the *y* axis.
/// * [`lenx`] - length of rendered section in the *x* axis.
///
/// * [`blitter`] - [`NcBlitter`] glyph set to use for blitting.
///
/// * [`flags`] - bitmask of options: ([`ADDALPHA`], [`BLEND`], [`CHILDPLANE`],
///   [`NODEGRADE`], [`VERALIGNED`], [`HORALIGNED`], [`NOINTERPOLATE`]).
///
/// * [`transcolor`] - treats this color as transparent when the [`ADDALPHA`] flag
///   is active.
///
/// * [`pxoffy`] - pixel offset within the cell in the *y* axis.
///
///   If [`NcBlitter::PIXEL`] is used the bitmap will be drawn offset from the
///   upper-left cell’s origin by these amounts, otherwise this will be ignored.
///
///   It is an error if either number exceeds the cell-pixel geometry in any
///   dimension (see [`NcPixelGeometry.cell_y`], [`NcVisualGeometry.cdim_yx`]).
///
/// * [`pxoffx`] - pixel offset within the cell in the *x* axis.
///
///   If [`NcBlitter::PIXEL`] is used, the bitmap will be drawn offset from the
///   upper-left cell’s origin by these amounts, otherwise this will be ignored.
///
///   It is an error if either number exceeds the cell-pixel geometry in any
///   dimension (see [`NcPixelGeometry.cell_x`], [`NcVisualGeometry.cdim_yx`]).
///
/// [`NcVisualOptions::builder()`]: NcVisualOptions#method.builder
/// [`NcAlign`]: crate::NcAlign
/// [`NcBlitter::PIXEL`]: crate::NcBlitter#associatedconstant.PIXEL
/// [`NcPixelGeometry.cell_y`]: crate::NcPixelGeometry#structfield.cell_y
/// [`NcPixelGeometry.cell_x`]: crate::NcPixelGeometry#structfield.cell_x
/// [`NcVisualGeometry.cdim_yx`]: crate::NcVisualGeometry#structfield.cdim_yx
/// [`n`]: crate::c_api::ffi::ncvisual_options#structfield.n
/// [`scaling`]: crate::c_api::ffi::ncvisual_options#structfield.scaling
/// [`y`]: crate::c_api::ffi::ncvisual_options#structfield.y
/// [`x`]: crate::c_api::ffi::ncvisual_options#structfield.x
/// [`begy`]: crate::c_api::ffi::ncvisual_options#structfield.begy
/// [`begx`]: crate::c_api::ffi::ncvisual_options#structfield.begx
/// [`leny`]: crate::c_api::ffi::ncvisual_options#structfield.leny
/// [`lenx`]: crate::c_api::ffi::ncvisual_options#structfield.lenx
/// [`blitter`]: crate::c_api::ffi::ncvisual_options#structfield.blitter
/// [`flags`]: crate::c_api::ffi::ncvisual_options#structfield.flags
/// [`transcolor`]: crate::c_api::ffi::ncvisual_options#structfield.transcolor
/// [`pxoffy`]: crate::c_api::ffi::ncvisual_options#structfield.pxoffy
/// [`pxoffx`]: crate::c_api::ffi::ncvisual_options#structfield.pxoffx
/// [`NOSCALE`]: crate::NcScale#associatedconstant.NOSCALE
/// [`SCALE`]: crate::NcScale#associatedconstant.SCALE
/// [`STRETCH`]: crate::NcScale#associatedconstant.STRETCH
/// [`NONE_HIRES`]: crate::NcScale#associatedconstant.NONE_HIRES
/// [`SCALE_HIRES`]: crate::NcScale#associatedconstant.SCALE_HIRES
/// [`ADDALPHA`]: NcVisualOptions#associatedconstant.ADDALPHA
/// [`BLEND`]: NcVisualOptions#associatedconstant.BLEND
/// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
/// [`NcVisualOptions::CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
/// [`NODEGRADE`]: NcVisualOptions#associatedconstant.NODEGRADE
/// [`VERALIGNED`]:NcVisualOptions#associatedconstant.VERALIGNED
/// [`HORALIGNED`]: NcVisualOptions#associatedconstant.HORALIGNED
/// [`NOINTERPOLATE`]: NcVisualOptions#associatedconstant.NOINTERPOLATE
pub type NcVisualOptions = crate::bindings::ffi::ncvisual_options;

/// Builder for [`NcVisualOptions`].
///
/// Can be constructed calling [`NcVisualOptions::builder()`].
///
/// [`NcVisualOptions::builder()`]: NcVisualOptions#method.builder
#[derive(Debug, Default)]
pub struct NcVisualOptionsBuilder<'ncplane> {
    plane: Option<&'ncplane mut NcPlane>,
    scale: NcScale,
    y: NcOffset,
    x: NcOffset,
    section_yx_lenyx: Option<(NcDim, NcDim, NcDim, NcDim)>,
    cell_offset_yx: Option<(NcDim, NcDim)>,
    blitter: NcBlitter,
    flags: u32,
    transcolor: NcRgba,
}

// NcRgba
//
/// Three RGB [`NcComponent`]s plus one alpha [`NcComponent`] (alias of `u32`).
///
/// ## Diagram
///
/// ```txt
/// AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
/// `type in C: no data type`
///
/// See also: [`NcRgb`] and [`NcChannel`] types.
///
/// [`NcComponent`]: crate::NcComponent
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

    /// Uses [`NcAlpha::Blend`][crate::NcAlpha#associatedconstant.BLEND] with
    /// the `NcVisual`.
    pub const BLEND: u32 = constants::NCVISUAL_OPTION_BLEND;

    /// allows you to indicate that the n field of ncvisual_options refers not to
    /// the plane onto which you'd like to blit, but the parent of a new plane.
    ///
    /// A plane will be created using the other parameters in the ncvisual_options,
    /// as a child of this parent. This means things like, say, vertically centering
    /// a sprixel relative to the standard plane can be done in one step.
    pub const CHILDPLANE: u32 = constants::NCVISUAL_OPTION_CHILDPLANE;

    /// Fails rather than gracefully degrade. See [`NcBlitter`].
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

    /// Uses [`NcAlpha::Blend`][crate::NcAlpha#associatedconstant.BLEND] with
    /// the `NcVisual`.
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
