//! `NcBoxMask`

/// A bitmask for drawing borders, gradients and corners.
///
/// # Flags
/// - [`GradTop`][NcBoxMask::GradTop]
/// - [`GradRight`][NcBoxMask::GradRight]
/// - [`GradBottom`][NcBoxMask::GradBottom]
/// - [`GradLeft`][NcBoxMask::GradLeft]
/// - [`MaskTop`][NcBoxMask::MaskTop]
/// - [`MaskRight`][NcBoxMask::MaskRight]
/// - [`MaskBottom`][NcBoxMask::MaskBottom]
/// - [`MaskLeft`][NcBoxMask::MaskLeft]
/// - [`CornerMask`][NcBoxMask::CornerMask]
/// - [`CornerShift`][NcBoxMask::CornerShift]
/// - [`None`][NcBoxMask::None]
///
/// # Default
/// *[`NcBoxMask::None`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NcBoxMask(pub c_api::NcBoxMask_u32);

/// # Constants
impl NcBoxMask {
    /// Top gradient mask.
    pub const GradTop: Self = Self(c_api::NCBOXGRAD_TOP);

    /// Right gradient mask.
    pub const GradRight: Self = Self(c_api::NCBOXGRAD_RIGHT);

    /// Bottom gradient mask.
    pub const GradBottom: Self = Self(c_api::NCBOXGRAD_BOTTOM);

    /// Left gradient mask.
    pub const GradLeft: Self = Self(c_api::NCBOXGRAD_LEFT);

    /// Top border mask.
    pub const MaskTop: Self = Self(c_api::NCBOXMASK_TOP);

    /// Right border mask.
    pub const MaskRight: Self = Self(c_api::NCBOXMASK_RIGHT);

    /// Bottom border mask.
    pub const MaskBottom: Self = Self(c_api::NCBOXMASK_BOTTOM);

    /// Left border mask.
    pub const MaskLeft: Self = Self(c_api::NCBOXMASK_LEFT);

    /// Corner mask to control the drawing of boxes corners.
    ///
    /// By default, vertexes are drawn whether their connecting edges are drawn
    /// or not. The value of the bits control this, and are interpreted as the
    /// number of connecting edges necessary to draw a given corner.
    ///
    /// At 0 (the default), corners are always drawn. At 3, corners are never
    /// drawn (since at most 2 edges can touch a box's corner).
    pub const CornerMask: Self = Self(c_api::NCBOXCORNER_MASK);

    /// The number of bits [`CornerMask`] is shifted.
    ///
    /// [`CornerMask`]: NcBoxMask#associatedconstant.CornerMask
    pub const CornerShift: Self = Self(c_api::NCBOXCORNER_SHIFT);

    /// None of the bits set.
    pub const None: Self = Self(0);
}

/// # Methods
impl NcBoxMask {
    /// Returns true if the current style has included the `other_style`.
    pub fn has(&self, other: NcBoxMask) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Adds the `other_style` to the current style.
    pub fn add(&mut self, other: NcBoxMask) {
        self.0 |= other.0
    }
}

mod std_impls {
    use super::{c_api::NcBoxMask_u32, NcBoxMask};

    impl Default for NcBoxMask {
        fn default() -> Self {
            Self::None
        }
    }

    crate::from_primitive![NcBoxMask, NcBoxMask_u32];
    crate::unit_impl_from![NcBoxMask, NcBoxMask_u32];
    crate::unit_impl_ops![bitwise; NcBoxMask, NcBoxMask_u32];
    crate::unit_impl_fmt![bases+display; NcBoxMask];
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// Controls the drawing of borders, gradients and corners.
    ///
    /// It's recommended to use [`NcBoxMask`][crate::NcBoxMask] instead.
    ///
    /// `NcBoxMax_u32` is defined in the least significant byte, where bits [3, 0] are
    /// are a border mask, and bits [7, 4] are a gradient mask.
    ///
    /// The drawing of the corners is defined in the second byte,
    /// see [`NCBOXCORNER_MASK`].
    ///
    /// # Associated `c_api` constants
    /// - [`NCBOXGRAD_TOP`]
    /// - [`NCBOXGRAD_RIGHT`]
    /// - [`NCBOXGRAD_BOTTOM`]
    /// - [`NCBOXGRAD_LEFT`]
    /// - [`NCBOXMASK_TOP`]
    /// - [`NCBOXMASK_RIGHT`]
    /// - [`NCBOXMASK_BOTTOM`]
    /// - [`NCBOXMASK_LEFT`]
    /// - [`NCBOXCORNER_MASK`]
    /// - [`NCBOXCORNER_SHIFT`]
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
    pub type NcBoxMask_u32 = u32;

    /// [`NcBoxMask_u32`] top gradient mask.
    pub const NCBOXGRAD_TOP: NcBoxMask_u32 = ffi::NCBOXGRAD_TOP;

    /// [`NcBoxMask_u32`] right gradient mask.
    pub const NCBOXGRAD_RIGHT: NcBoxMask_u32 = ffi::NCBOXGRAD_RIGHT;

    /// [`NcBoxMask_u32`] bottom gradient mask.
    pub const NCBOXGRAD_BOTTOM: NcBoxMask_u32 = ffi::NCBOXGRAD_BOTTOM;

    /// [`NcBoxMask_u32`] left gradient mask.
    pub const NCBOXGRAD_LEFT: NcBoxMask_u32 = ffi::NCBOXGRAD_LEFT;

    /// [`NcBoxMask_u32`] top border mask.
    pub const NCBOXMASK_TOP: NcBoxMask_u32 = ffi::NCBOXMASK_TOP;

    /// [`NcBoxMask_u32`] right border mask.
    pub const NCBOXMASK_RIGHT: NcBoxMask_u32 = ffi::NCBOXMASK_RIGHT;

    /// [`NcBoxMask_u32`] bottom border mask.
    pub const NCBOXMASK_BOTTOM: NcBoxMask_u32 = ffi::NCBOXMASK_BOTTOM;

    /// [`NcBoxMask_u32`] left border mask.
    pub const NCBOXMASK_LEFT: NcBoxMask_u32 = ffi::NCBOXMASK_LEFT;

    /// [`NcBoxMask_u32`] corner mask to control the drawing of boxes corners.
    ///
    /// By default, vertexes are drawn whether their connecting edges are drawn
    /// or not. The value of the bits control this, and are interpreted as the
    /// number of connecting edges necessary to draw a given corner.
    ///
    /// At 0 (the default), corners are always drawn. At 3, corners are never
    /// drawn (since at most 2 edges can touch a box's corner).
    pub const NCBOXCORNER_MASK: NcBoxMask_u32 = ffi::NCBOXCORNER_MASK;

    /// [`NcBoxMask_u32`] the number of bits [`NCBOXCORNER_MASK`] is shifted.
    pub const NCBOXCORNER_SHIFT: NcBoxMask_u32 = ffi::NCBOXCORNER_SHIFT;
}
