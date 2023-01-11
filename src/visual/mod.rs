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
//W  ncvisual_blit
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
use crate::{c_api::NcResult_i32, NcBlitter, NcChannel, NcPlane, NcScale, NcTime};

mod geometry;
mod methods;
pub(crate) mod options;
mod reimplemented;

pub use geometry::NcVisualGeometry;
pub use options::{NcVisualFlag, NcVisualOptions, NcVisualOptionsBuilder};

/// A visual bit of multimedia.
///
/// It can be constructed from a rgba or bgra buffer.
///
/// The [`NcVisualOptions`] structure is used only by the following methods:
/// - [`geom`][NcVisual#method.geom]
/// - [`blit`][NcVisual#method.blit]
/// - [`simple_streamer`][NcVisual#method.simple_streamer]
pub type NcVisual = crate::c_api::ffi::ncvisual;

/// Called for each frame rendered from an `NcVisual`
///
/// If anything but 0 is returned, the streaming operation ceases immediately,
/// and that value is propagated out.
///
/// The recommended absolute display time target is passed in 'tspec'.
pub type NcStreamCb =
    fn(&mut NcVisual, &mut NcVisualOptions, &NcTime, Option<&mut NcPlane>) -> NcResult_i32;

pub(crate) mod c_api {
    pub use super::geometry::c_api::*;
    pub use super::options::c_api::*;
}
