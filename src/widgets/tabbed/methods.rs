//! `NcTabe` & `NcTabbed*` methods and associated functions.

use super::{NcTab, NcTabbed, NcTabbedOptions};
// use crate::{error_ref_mut, ncreader_create, NcPlane, NcResult};

/// # `NcTabbedOptions` Constructors
impl NcTabbedOptions {
    /// `NcTabbedOptions` simple constructor
    pub const fn new() -> Self {
        Self {
            // channels used for input
            tchannels: 0,
            // attributes used for input
            tattrword: 0,
            // bitfield of NCREADER_OPTION_*
            flags: 0,
        }
    }
}

/// # `NcTabbed` Constructors
impl NcTabbed {
    /// `NcTabbed` simple constructor
    pub fn new<'a>(plane: &mut NcPlane) -> NcResult<&'a mut Self> {
        Self::with_options(plane, NcTabbedOptions::new())
    }

    /// `NcTabbed` constructor with options
    pub fn with_options<'a>(
        plane: &mut NcPlane,
        options: NcTabbedOptions,
    ) -> NcResult<&'a mut Self> {
        error_ref_mut![unsafe { nctabbed_create(plane, &options) }]
    }
}
