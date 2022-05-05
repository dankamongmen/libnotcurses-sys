//!

use crate::{c_api::ffi, NcAlign, NcResizeCb};
use std::ptr::{null, null_mut};

mod builder;
pub(crate) mod flags;

pub use builder::NcPlaneOptionsBuilder;
pub use flags::NcPlaneFlag;

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
    /// New `NcPlaneOptions` using the horizontal x.
    pub fn new(y: i32, x: i32, rows: u32, cols: u32) -> Self {
        Self::with_flags(y, x, rows, cols, None, NcPlaneFlag::None, 0, 0)
    }

    /// Returns a default `NcPlane` options builder.
    pub fn builder() -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::default()
    }

    /// Returns a builder object from the current `NcPlane` options.
    pub fn to_builder(&self) -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::from_options(self)
    }

    /// New `NcPlaneOptions` with horizontal alignment.
    pub fn new_aligned(y: i32, align: impl Into<NcAlign>, rows: u32, cols: u32) -> Self {
        Self::with_flags_aligned(y, align.into(), rows, cols, None, NcPlaneFlag::HorAligned)
    }

    /// New `NcPlaneOptions`, with flags.
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

    /// New `NcPlaneOptions`, with flags and horizontal alignment.
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
