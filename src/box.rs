//! `NcBoxMask`

/// Controls the drawing of borders, gradients and corners (alias of `u32`).
///
/// NcBoxMax is defined in the least significant byte, where bits [3, 0] are
/// are a border mask, and bits [7, 4] are a gradient mask.
///
/// The drawing of the corners is defined in the second byte,
/// see [`NcBoxMask::CORNER_MASK`][NcBoxMask#associatedconstant.CORNER_MASK].
///
/// ## Diagram
///
/// ```txt
/// MASK_TOP    0x0001  0b00000001
/// MASK_RIGHT  0x0002  0b00000010
/// MASK_BOTTOM 0x0004  0b00000100
/// MASK_LEFT   0x0008  0b00001000
///
/// GRAD_TOP    0x0010  0b00010000
/// GRAD_RIGHT  0x0020  0b00100000
/// GRAD_BOTTOM 0x0040  0b01000000
/// GRAD_LEFT   0x0080  0b10000000
///
/// NCBOXCORNER_MASK  0x0300  0b00000111_00000000
///
/// NCBOXCORNER_SHIFT 8
/// ```
pub type NcBoxMask = u32;

crate::impl_api![
    NcBoxMask,
    NcBoxMaskApi,
    /// [`NcBoxMask`] top gradient mask.
    const GRAD_TOP: NcBoxMask = constants::NCBOXGRAD_TOP;,
    /// [`NcBoxMask`] right gradient mask.
    const GRAD_RIGHT: NcBoxMask = constants::NCBOXGRAD_RIGHT;,
    /// [`NcBoxMask`] bottom gradient mask.
    const GRAD_BOTTOM: NcBoxMask = constants::NCBOXGRAD_BOTTOM;,
    /// [`NcBoxMask`] left gradient mask.
    const GRAD_LEFT: NcBoxMask = constants::NCBOXGRAD_LEFT;,
    /// [`NcBoxMask`] top border mask.
    const MASK_TOP: NcBoxMask = constants::NCBOXMASK_TOP;,
    /// [`NcBoxMask`] right border mask.
    const MASK_RIGHT: NcBoxMask = constants::NCBOXMASK_RIGHT;,
    /// [`NcBoxMask`] bottom border mask.
    const MASK_BOTTOM: NcBoxMask = constants::NCBOXMASK_BOTTOM;,
    /// [`NcBoxMask`] left border mask.
    const MASK_LEFT: NcBoxMask = constants::NCBOXMASK_LEFT;,
    /// [`NcBoxMask`] corner mask to control the drawing of boxes corners.
    ///
    /// By default, vertexes are drawn whether their connecting edges are drawn
    /// or not. The value of the bits control this, and are interpreted as the
    /// number of connecting edges necessary to draw a given corner.
    ///
    /// At 0 (the default), corners are always drawn. At 3, corners are never drawn
    /// (since at most 2 edges can touch a box's corner),.
    const CORNER_MASK: NcBoxMask = constants::NCBOXCORNER_MASK;,
    /// The number of bits
    /// [`NcBoxMask::CORNER_MASK`][NcBoxMask#associatedconstant.CORNER_MASK]
    /// is shifted.
    const CORNER_SHIFT: NcBoxMask = constants::NCBOXCORNER_SHIFT;
];

pub(crate) mod constants {
    use crate::NcBoxMask;

    /// [`NcBoxMask`] top gradient mask.
    pub const NCBOXGRAD_TOP: NcBoxMask = crate::bindings::ffi::NCBOXGRAD_TOP;
    /// [`NcBoxMask`] right gradient mask.
    pub const NCBOXGRAD_RIGHT: NcBoxMask = crate::bindings::ffi::NCBOXGRAD_RIGHT;
    /// [`NcBoxMask`] bottom gradient mask.
    pub const NCBOXGRAD_BOTTOM: NcBoxMask = crate::bindings::ffi::NCBOXGRAD_BOTTOM;
    /// [`NcBoxMask`] left gradient mask.
    pub const NCBOXGRAD_LEFT: NcBoxMask = crate::bindings::ffi::NCBOXGRAD_LEFT;

    /// [`NcBoxMask`] top border mask.
    pub const NCBOXMASK_TOP: NcBoxMask = crate::bindings::ffi::NCBOXMASK_TOP;
    /// [`NcBoxMask`] right border mask.
    pub const NCBOXMASK_RIGHT: NcBoxMask = crate::bindings::ffi::NCBOXMASK_RIGHT;
    /// [`NcBoxMask`] bottom border mask.
    pub const NCBOXMASK_BOTTOM: NcBoxMask = crate::bindings::ffi::NCBOXMASK_BOTTOM;
    /// [`NcBoxMask`] left border mask.
    pub const NCBOXMASK_LEFT: NcBoxMask = crate::bindings::ffi::NCBOXMASK_LEFT;

    /// [`NcBoxMask`] corner mask to control the drawing of boxes corners.
    ///
    /// By default, vertexes are drawn whether their connecting edges are drawn
    /// or not. The value of the bits control this, and are interpreted as the
    /// number of connecting edges necessary to draw a given corner.
    ///
    /// At 0 (the default), corners are always drawn. At 3, corners are never drawn
    /// (since at most 2 edges can touch a box's corner).
    pub const NCBOXCORNER_MASK: NcBoxMask = crate::bindings::ffi::NCBOXCORNER_MASK;

    /// The number of bits [`NCBOXCORNER_MASK`] is shifted in [`NcBoxMask`].
    pub const NCBOXCORNER_SHIFT: NcBoxMask = crate::bindings::ffi::NCBOXCORNER_SHIFT;
}
