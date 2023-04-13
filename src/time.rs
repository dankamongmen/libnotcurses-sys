//! `NcTime`

use crate::c_api::ffi::timespec;

#[cfg(not(feature = "libc"))]
use crate::c_api::ffi::{__syscall_slong_t as c_long, __time_t as time_t};

// needed for MacOs
#[cfg(feature = "libc")]
use libc::{c_long, time_t};

/// A time in seconds and nanoseconds.
///
/// It assumes that pre-epoch Timespecs have negative `tv_sec` and positive nsec fields.
///
/// A record specifying a time value in seconds and nanoseconds, where
/// nanoseconds represent the offset from the given second.
// Expected by [`notcurses_get`] & [`notcurses_get_nblock`].
pub type NcTime = timespec;

impl NcTime {
    /// New `NcTime` with the specified seconds and nanoseconds.
    pub fn new(seconds: time_t, nanoseconds: c_long) -> Self {
        Self { tv_sec: seconds, tv_nsec: nanoseconds }
    }
}
