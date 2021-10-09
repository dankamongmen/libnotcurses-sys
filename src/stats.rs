//! `NcStats`

use crate::{fns, Nc};

/// notcurses runtime statistics
pub type NcStats = crate::bindings::ffi::ncstats;

/// # `NcStats` Methods.
impl NcStats {
    /// Allocates an NcStats object.
    pub fn new(nc: &mut Nc) -> &mut Self {
        unsafe { &mut *fns::notcurses_stats_alloc(nc) }
    }

    /// Acquires an atomic snapshot of the notcurses object's stats.
    pub fn stats(&mut self, nc: &mut Nc) {
        unsafe { fns::notcurses_stats(nc, self) }
    }

    /// Resets all cumulative stats (immediate ones are not reset).
    pub fn reset(&mut self, nc: &mut Nc) {
        unsafe { fns::notcurses_stats_reset(nc, self) }
    }
}
