//! `NcVisual*` methods and associated functions.

// use core::ptr::null_mut;
use libc::c_void;

use crate::{NcSubproc, NcSubprocOptions};

/// # NcVisualOptions Constructors
impl NcSubprocOptions {
    ///
    pub fn new(curry: *mut c_void, restart_period: u64, flags: u64) -> Self {
        Self {
            curry,
            // restart this many seconds after an exit (watch)
            restart_period,
            // bitfield over NCOPTION_SUBPROC_* (none yet)
            flags,
        }
    }
}

/// # NcSubproc Constructors & Destructors
impl NcSubproc {}

/// # NcSubproc Methods
impl NcSubproc {}
