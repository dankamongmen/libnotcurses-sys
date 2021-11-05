use crate::{c_api::notcurses_init, Nc, NcOptions};

/// Helper function for initializing Nc on C style tests.
#[allow(dead_code)]
pub(crate) unsafe fn notcurses_init_test<'a>() -> &'a mut Nc {
    &mut *notcurses_init(
        &NcOptions::with_flags(NcOptions::SUPPRESS_BANNERS | NcOptions::NO_ALTERNATE_SCREEN),
        core::ptr::null_mut(),
    )
}
