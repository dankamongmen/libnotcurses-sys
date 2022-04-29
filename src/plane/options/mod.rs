//!

use crate::{c_api::ffi, NcAlign, NcResizeCb};

mod builder;
pub use builder::NcPlaneOptionsBuilder;

use std::ptr::{null, null_mut};

/// Options struct for [`NcPlane`][crate::NcPlane].
///
/// It is recommended to construct it via [`NcPlaneOptionsBuilder`]
/// by calling [`NcPlaneOptions::builder()`].
///
/// # Fields
/// - [`y`]: vertical placement relative to parent plane.
/// - [`x`]: horizontal placement relative to parent plane.
/// - [`rows`]: vertical length in rows.
/// - [`cols`]: horizontal length in columns.
/// - [`userptr`]: optional user curry.
/// - [`name`]: optional string identifier for debugging.
/// - [`resizecb`]: callback when parent is resized.
/// - [`flags`]: [`NcPlaneFlag`].
/// - [`margin_b`]: bottom margin (requires the [`Marginalized`] flag).
/// - [`margin_r`]: right margin (requires the [`Marginalized`]).
///
/// [`y`]: ffi::ncplane_options#structfield.y
/// [`x`]: ffi::ncplane_options#structfield.x
/// [`rows`]: ffi::ncplane_options#structfield.rows
/// [`cols`]: ffi::ncplane_options#structfield.cols
/// [`userptr`]: ffi::ncplane_options#structfield.userptr
/// [`name`]: ffi::ncplane_options#structfield.name
/// [`resizecb`]: ffi::ncplane_options#structfield.resizecb
/// [`flags`]: ffi::ncplane_options#structfield.flags
/// [`margin_b`]: ffi::ncplane_options#structfield.margin_b
/// [`margin_r`]: ffi::ncplane_options#structfield.margin_r
/// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
pub type NcPlaneOptions = ffi::ncplane_options;

/// # Constructors
impl NcPlaneOptions {
    /// New NcPlaneOptions using the horizontal x.
    pub fn new(y: i32, x: i32, rows: u32, cols: u32) -> Self {
        Self::with_flags(y, x, rows, cols, None, NcPlaneFlag::None, 0, 0)
    }

    /// Returns a default `NcPlaneOptionsBuilder`.
    pub fn builder() -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::default()
    }

    /// Returns a builder object for `NcPlaneOptions` from the current options.
    pub fn to_builder(&self) -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::from_options(self)
    }

    /// New NcPlaneOptions with horizontal alignment.
    pub fn new_aligned(y: i32, align: impl Into<NcAlign>, rows: u32, cols: u32) -> Self {
        Self::with_flags_aligned(y, align.into(), rows, cols, None, NcPlaneFlag::HorAligned)
    }

    /// New NcPlaneOptions, with flags.
    pub fn with_flags(
        y: i32,
        x: i32,
        rows: u32,
        cols: u32,
        resizecb: Option<NcResizeCb>,
        flags: impl Into<NcPlaneFlag>,
        margin_b: u32,
        margin_r: u32,
    ) -> Self {
        NcPlaneOptions {
            y: y as i32,
            x: x as i32,
            rows,
            cols,
            userptr: null_mut(),
            name: null(),
            resizecb: crate::c_api::ncresizecb_to_c(resizecb),
            flags: flags.into().into(),
            margin_b,
            margin_r,
        }
    }

    /// New NcPlaneOptions, with flags and horizontal alignment.
    ///
    /// Note: Already includes the
    /// [`NcPlaneOptions::HORALIGNED`][NcPlaneOptions#associatedconstant.HORALIGNED]
    /// flag.
    pub fn with_flags_aligned(
        y: i32,
        align: impl Into<NcAlign>,
        rows: u32,
        cols: u32,
        resizecb: Option<NcResizeCb>,
        flags: impl Into<NcPlaneFlag>,
    ) -> Self {
        let flags = NcPlaneFlag::HorAligned | flags.into();
        NcPlaneOptions {
            y: y as i32,
            x: align.into().into(),
            rows,
            cols,
            userptr: null_mut(),
            name: null(),
            resizecb: crate::c_api::ncresizecb_to_c(resizecb),
            flags: flags.into(),
            margin_b: 0,
            margin_r: 0,
        }
    }
}

/// # Methods
impl NcPlaneOptions {
    /// Returns `true` if it has the [`VerAligned`] flag set.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    pub fn is_veraligned(&self) -> bool {
        self.flags & NcPlaneFlag::VerAligned != NcPlaneFlag::None
    }

    /// Returns `true` if it has the [`HorAligned`] flag set.
    ///
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn is_horaligned(&self) -> bool {
        self.flags & NcPlaneFlag::HorAligned != NcPlaneFlag::None
    }

    /// Returns `true` if it has the [`Marginalized`] flag set.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn is_marginalized(&self) -> bool {
        self.flags & NcPlaneFlag::Marginalized != NcPlaneFlag::None
    }

    /// Returns `true` if it has the [`Fixed`] flag set.
    ///
    /// [`Fixed`]: NcPlaneFlag#associatedconstant.Fixed
    pub fn is_fixed(&self) -> bool {
        self.flags & NcPlaneFlag::Fixed != NcPlaneFlag::None
    }

    /// Returns `true` if it has the [`AutoGrow`] flag set.
    ///
    /// [`AutoGrow`]: NcPlaneFlag#associatedconstant.AutoGrow
    pub fn is_autogrow(&self) -> bool {
        self.flags & NcPlaneFlag::AutoGrow != NcPlaneFlag::None
    }

    /// Returns `true` if it has the [`VScroll`] flag set.
    ///
    /// [`VScroll`]: NcPlaneFlag#associatedconstant.VScroll
    pub fn is_vscroll(&self) -> bool {
        self.flags & NcPlaneFlag::VScroll != NcPlaneFlag::None
    }
}

/// A bitmask of flags for [`NcPlaneOptions`].
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
    /// Use [`NcAlign`] for 'x'.
    pub const HorAligned: Self = Self(c_api::NCPLANE_OPTION_HORALIGNED);

    /// Vertical alignment relative to the parent plane.
    ///
    /// Use [`NcAlign`] for 'y'.
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
    use super::ffi;

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
