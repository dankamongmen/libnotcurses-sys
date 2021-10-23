use crate::{NcBlitter, NcDim};

/// Contains the blitter geometry information as returned by the
/// NcVisual.[`blitter_geom`][crate::NcVisual#method.blitter_geom] method.
///
/// - `y`, `x`: the input size in pixels.
/// - `scale_y`, `scale_x`: the scaling
/// - `blitter` The blitter that will be used
///
#[derive(Clone, Debug)]
pub struct NcBlitterGeometry {
    ///
    pub y: NcDim,
    ///
    pub x: NcDim,
    ///
    pub scale_y: NcDim,
    ///
    pub scale_x: NcDim,
    /// The blitter that will be used.
    pub blitter: NcBlitter,
}
