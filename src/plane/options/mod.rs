//!

mod builder;
pub(crate) mod constants;
pub use builder::NcPlaneOptionsBuilder;

use crate::{c_api, NcAlign, NcDim, NcOffset, NcResizeCb};

use std::ptr::{null, null_mut};

/// Options struct for [`NcPlane`][crate::NcPlane].
///
/// It is recommended to construct it via [`NcPlaneOptionsBuilder`]
/// by calling [`NcPlaneOptions::builder()`].
pub type NcPlaneOptions = crate::bindings::ffi::ncplane_options;

/// # Constructors
impl NcPlaneOptions {
    /// New NcPlaneOptions using the horizontal x.
    pub fn new(y: NcOffset, x: NcOffset, rows: NcDim, cols: NcDim) -> Self {
        Self::with_flags(y, x, rows, cols, None, 0, 0, 0)
    }

    /// Returns a builder object for `NcPlaneOptions`.
    pub fn builder() -> NcPlaneOptionsBuilder {
        NcPlaneOptionsBuilder::default()
    }

    /// New NcPlaneOptions with horizontal alignment.
    pub fn new_aligned(y: NcOffset, align: NcAlign, rows: NcDim, cols: NcDim) -> Self {
        Self::with_flags_aligned(y, align, rows, cols, None, NcPlaneOptions::HORALIGNED)
    }

    /// New NcPlaneOptions, with flags.
    pub fn with_flags(
        y: NcOffset,
        x: NcOffset,
        rows: NcDim,
        cols: NcDim,
        resizecb: Option<NcResizeCb>,
        flags: u64,
        margin_b: NcOffset,
        margin_r: NcOffset,
    ) -> Self {
        NcPlaneOptions {
            y: y as i32,
            x: x as i32,
            rows,
            cols,
            userptr: null_mut(),
            name: null(),
            resizecb: c_api::ncresizecb_to_c(resizecb),
            flags,
            margin_b: margin_b as i32,
            margin_r: margin_r as i32,
        }
    }

    /// New NcPlaneOptions, with flags and horizontal alignment.
    ///
    /// Note: Already includes the
    /// [`NcPlaneOptions::HORALIGNED`][NcPlaneOptions#associatedconstant.HORALIGNED]
    /// flag.
    pub fn with_flags_aligned(
        y: NcOffset,
        align: NcAlign,
        rows: NcDim,
        cols: NcDim,
        resizecb: Option<NcResizeCb>,
        flags: u64,
    ) -> Self {
        let flags = NcPlaneOptions::HORALIGNED | flags;
        NcPlaneOptions {
            y: y as i32,
            x: align as i32,
            rows,
            cols,
            userptr: null_mut(),
            name: null(),
            resizecb: c_api::ncresizecb_to_c(resizecb),
            flags,
            margin_b: 0,
            margin_r: 0,
        }
    }
}
