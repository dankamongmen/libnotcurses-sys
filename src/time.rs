//! `NcTime`

///
// Expected by [`notcurses_get`] & [`notcurses_get_nblock`]
pub type NcTime = crate::bindings::ffi::timespec;

impl NcTime {
    /// New NcTime with the specified seconds and nanoseconds.
    pub fn new(seconds: i64, nanoseconds: i64) -> Self {
        Self {
            tv_sec: seconds,
            tv_nsec: nanoseconds,
        }
    }
}
