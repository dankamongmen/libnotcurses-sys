//! `NcTime`

use crate::c_api::ffi::{__syscall_slong_t, __time_t, timespec};

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
    //
    // Both fields should be i64 on 64-bit platforms, and i32 on 32-bit ones.
    pub fn new(seconds: __time_t, nanoseconds: __syscall_slong_t) -> Self {
        Self { tv_sec: seconds, tv_nsec: nanoseconds }
    }
}
