//! `NcTime`

/// A time in seconds and nanoseconds.
///
/// It assumes that pre-epoch Timespecs have negative `tv_sec` and positive nsec fields.
///
/// A record specifying a time value in seconds and nanoseconds, where
/// nanoseconds represent the offset from the given second.

// Expected by [`notcurses_get`] & [`notcurses_get_nblock`]
pub type NcTime = crate::c_api::ffi::timespec;

impl NcTime {
    /// New NcTime with the specified seconds and nanoseconds.
    pub fn new(seconds: i64, nanoseconds: i64) -> Self {
        Self { tv_sec: seconds, tv_nsec: nanoseconds }
    }
}
