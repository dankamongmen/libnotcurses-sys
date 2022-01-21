//! `NcResizeCb`

use crate::{c_api::NcResult_i32, NcPlane};

/// A callback function called when an [`NcPlane`] is resized.
///
/// # See also
/// - [`NcResizeCbUnsafe`][c_api::NcResizeCbUnsafe]
/// - [`ncresizecb_to_rust`][c_api::ncresizecb_to_rust]
/// - [`ncresizecb_to_c`][c_api::ncresizecb_to_c]
pub type NcResizeCb = fn(&mut NcPlane) -> NcResult_i32;

pub(crate) mod c_api {
    use super::*;

    /// The unsafe version of [`NcResizeCb`] expected by the notcurses C API.
    pub type NcResizeCbUnsafe = unsafe extern "C" fn(*mut NcPlane) -> NcResult_i32;

    /// Converts [`NcResizeCbUnsafe`] to [`NcResizeCb`].
    pub fn ncresizecb_to_rust(resizecb: Option<NcResizeCbUnsafe>) -> Option<NcResizeCb> {
        resizecb.map(|cb| unsafe { core::mem::transmute(cb) })
    }

    /// Converts [`NcResizeCb`] to [`NcResizeCbUnsafe`].
    ///
    // WAITING for https://github.com/rust-lang/rust/issues/57563
    // to make this function const, and then `NcPlaneOptions` constructors.
    pub fn ncresizecb_to_c(resizecb: Option<NcResizeCb>) -> Option<NcResizeCbUnsafe> {
        resizecb.map(|cb| unsafe { core::mem::transmute(cb) })
    }
}
