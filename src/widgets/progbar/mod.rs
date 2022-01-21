//! `NcProgbar` widget.

// functions already exported by bindgen : 5
// -----------------------------------------
// (#) test: 0
// (W) wrap: 5 / 0
// -----------------------------------------
//W ncprogbar_create,
//W ncprogbar_destroy,
//W ncprogbar_plane,
//W ncprogbar_progress,
//W ncprogbar_set_progress,

use crate::c_api::ffi;

mod methods;

/// Progress bars. They proceed linearly in any of four directions.
///
/// The entirety of the plane will be used -- any border should be provided by
/// the caller on another plane.
///
/// The plane will not be erased; text preloaded into the plane will be consumed
/// by the progress indicator.
///
/// The bar is redrawn for each provided progress report (a double between 0
/// and 1), and can regress with lower values.
///
/// The procession will take place along the longer dimension at the time of each
/// redraw, with the horizontal length scaled by 2 for purposes of comparison.
/// I.e. for a plane of 20 rows and 50 columns, the progress will be to the
/// right (50 > 40), or left with
/// [NcProgBarOptions::RETROGRADE][NcProgBarOptions#associatedconstant.RETROGRADE].
///
/// `type in C: ncprogbar (struct)`
///
pub type NcProgBar = ffi::ncprogbar;

/// Options struct for [`NcProgBar`]
///
/// `type in C: ncprogbar_options (struct)`
///
pub type NcProgBarOptions = ffi::ncprogbar_options;

impl NcProgBarOptions {
    /// proceeds left/down
    pub const RETROGRADE: u32 = c_api::NCPROGBAR_OPTION_RETROGRADE;
}

pub(crate) mod c_api {
    use super::ffi;

    /// [`NcProgBarOptions`][super::NcProgBarOptions] flag
    /// to indicate proceeding left/down.
    pub const NCPROGBAR_OPTION_RETROGRADE: u32 = ffi::NCPROGBAR_OPTION_RETROGRADE;
}
