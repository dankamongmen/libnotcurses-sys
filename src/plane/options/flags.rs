/// A bitmask of flags for [`NcPlaneOptions`][crate::NcPlaneOptions].
///
/// # Flag
/// - [`None`][NcPlaneFlag::None]
/// - [`HorAligned`][NcPlaneFlag::HorAligned]
/// - [`VerAligned`][NcPlaneFlag::VerAligned]
/// - [`Marginalized`][NcPlaneFlag::Marginalized]
/// - [`Fixed`][NcPlaneFlag::Fixed]
/// - [`AutoGrow`][NcPlaneFlag::AutoGrow]
/// - [`VScroll`][NcPlaneFlag::VScroll]
///
/// # Default
/// *[`NcPlaneFlag::None`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NcPlaneFlag(pub c_api::NcPlaneFlag_u64);

impl NcPlaneFlag {
    /// No flags.
    pub const None: Self = Self(0);

    /// Horizontal alignment relative to the parent plane.
    ///
    /// Use [`NcAlign`][crate::NcAlign] for 'x'.
    pub const HorAligned: Self = Self(c_api::NCPLANE_OPTION_HORALIGNED);

    /// Vertical alignment relative to the parent plane.
    ///
    /// Use [`NcAlign`][crate::NcAlign] for 'y'.
    pub const VerAligned: Self = Self(c_api::NCPLANE_OPTION_VERALIGNED);

    /// Maximize relative to the parent plane, minus the provided margins.
    ///
    /// The margins are best-effort; the plane will always be at least 1 column
    /// by 1 row. If the margins can be effected, the plane will be sized to all
    /// remaining space. 'y' and 'x' are overloaded as the top and left margins
    /// when this flag is used. 'rows' and 'cols' must be 0 when this flag is
    /// used. This flag is exclusive with both of the alignment flags.
    pub const Marginalized: Self = Self(c_api::NCPLANE_OPTION_MARGINALIZED);

    /// Do not scroll alongside its parent.
    ///
    /// If this plane is bound to a scrolling plane, it ought *not* scroll along
    /// with the parent (it will still move with the parent, maintaining its
    /// relative position, if the parent is moved to a new location).
    pub const Fixed: Self = Self(c_api::NCPLANE_OPTION_FIXED);

    /// Enables automatic growth of the plane to accommodate output.
    ///
    /// Creating a plane with this flag is equivalent to immediately calling
    /// `NcPlane::`[`set_autogrow`]`(true)` following plane creation.
    ///
    /// [`set_autogrow`]: crate::NcPlane#method.set_autogrow
    pub const AutoGrow: Self = Self(c_api::NCPLANE_OPTION_AUTOGROW);

    /// Enables vertical scrolling of the plane to accommodate output.
    ///
    /// Creating a plane with this flag is equivalent to immediately calling
    /// `NcPlane::`[`set_scrolling`]`(true)` following plane creation.
    ///
    /// [`set_scrolling`]: crate::NcPlane#method.set_autogrow
    pub const VScroll: Self = Self(c_api::NCPLANE_OPTION_VSCROLL);
}

mod std_impls {
    use super::{c_api::NcPlaneFlag_u64, NcPlaneFlag};

    impl Default for NcPlaneFlag {
        fn default() -> Self {
            Self::None
        }
    }

    crate::from_primitive![NcPlaneFlag, NcPlaneFlag_u64];
    crate::unit_impl_from![NcPlaneFlag, NcPlaneFlag_u64];
    crate::unit_impl_ops![bitwise; NcPlaneFlag, NcPlaneFlag_u64];
    crate::unit_impl_fmt![bases+display; NcPlaneFlag];
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// A bitmask of flags for [`NcPlaneOptions`][crate::NcPlaneOptions].
    ///
    /// It's recommended to use [`NcPlaneFlag`][crate::NcPlaneFlag] instead.
    ///
    /// # Associated `c_api` constants
    /// - [`NCPLANE_OPTION_HORALIGNED`]
    /// - [`NCPLANE_OPTION_VERALIGNED`]
    /// - [`NCPLANE_OPTION_MARGINALIZED`]
    /// - [`NCPLANE_OPTION_FIXED`]
    /// - [`NCPLANE_OPTION_AUTOGROW`]
    /// - [`NCPLANE_OPTION_VSCROLL`]
    pub type NcPlaneFlag_u64 = u64;

    /// [`NcPlaneFlag_u64`] Horizontal alignment relative to the parent plane.
    ///
    /// Use `NcAlign_u32` for 'x'.
    pub const NCPLANE_OPTION_HORALIGNED: NcPlaneFlag_u64 =
        ffi::NCPLANE_OPTION_HORALIGNED as NcPlaneFlag_u64;

    /// [`NcPlaneFlag_u64`] flag for vertical alignment relative to the parent
    /// plane.
    ///
    /// Use `NcAlign_u32` for 'y'.
    pub const NCPLANE_OPTION_VERALIGNED: NcPlaneFlag_u64 =
        ffi::NCPLANE_OPTION_VERALIGNED as NcPlaneFlag_u64;

    /// [`NcPlaneFlag_u64`] flag to maximize relative to the parent plane,
    /// modulo the provided margins.
    ///
    /// The margins are best-effort; the plane will always be at least 1 column by
    /// 1 row. If the margins can be effected, the plane will be sized to all
    /// remaining space. 'y' and 'x' are overloaded as the top and left margins
    /// when this flag is used. 'rows' and 'cols' must be 0 when this flag is
    /// used. This flag is exclusive with both of the alignment flags.
    pub const NCPLANE_OPTION_MARGINALIZED: NcPlaneFlag_u64 =
        ffi::NCPLANE_OPTION_MARGINALIZED as NcPlaneFlag_u64;

    /// [`NcPlaneFlag_u64`] flag to avoid scrolling alongside its parent.
    ///
    /// If this plane is bound to a scrolling plane, it ought *not* scroll along
    /// with the parent (it will still move with the parent, maintaining its
    /// relative position, if the parent is moved to a new location).
    pub const NCPLANE_OPTION_FIXED: NcPlaneFlag_u64 = ffi::NCPLANE_OPTION_FIXED as NcPlaneFlag_u64;

    /// [`NcPlaneFlag_u64`] flag that enables automatic growth of the plane to
    /// accommodate output.
    ///
    /// Creating a plane with this flag is equivalent to immediately calling
    /// `ncplane_set_autogrow(p, true)` following plane creation.
    pub const NCPLANE_OPTION_AUTOGROW: NcPlaneFlag_u64 =
        ffi::NCPLANE_OPTION_AUTOGROW as NcPlaneFlag_u64;

    /// [`NcPlaneFlag_u64`] flag that enables vertical scrolling of the plane
    /// to accommodate output.
    ///
    /// Creating a plane with this flag is equivalent to immediately calling
    /// `ncplane_set_scrolling(p, true)` following plane creation.
    pub const NCPLANE_OPTION_VSCROLL: NcPlaneFlag_u64 =
        ffi::NCPLANE_OPTION_VSCROLL as NcPlaneFlag_u64;
}
