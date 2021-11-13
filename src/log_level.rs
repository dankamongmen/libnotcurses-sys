/// Log level for [`NcOptions`][crate::NcOptions] (alias of `i32`).
///
/// These log levels consciously map cleanly to those of libav; notcurses itself
/// does not use this full granularity. The log level does not affect the opening
/// and closing banners, which can be disabled via the `NcOptions`
/// `NCOPTION_SUPPRESS_BANNERS`.
/// Note that if stderr is connected to the same terminal on which we're
/// rendering, any kind of logging will disrupt the output.
pub type NcLogLevel = i32; // crate::bindings::ffi::ncloglevel_e;

crate::impl_api![
    NcLogLevel,
    NcLogLevelApi,
    /// this is honestly a bit much.
    const DEBUG: NcLogLevel = constants::NCLOGLEVEL_DEBUG;,
    /// we can't keep doin' this, but we can do other things.
    const ERROR: NcLogLevel = constants::NCLOGLEVEL_ERROR;,
    /// we're hanging around, but we've had a horrible fault.
    const FATAL: NcLogLevel = constants::NCLOGLEVEL_FATAL;,
    /// "detailed information.
    const INFO: NcLogLevel = constants::NCLOGLEVEL_INFO;,
    /// print diagnostics immediately related to crashing.
    const PANIC: NcLogLevel = constants::NCLOGLEVEL_PANIC;,
    /// default. print nothing once fullscreen service begins.
    const SILENT: NcLogLevel = constants::NCLOGLEVEL_SILENT;,
    /// there's probably a better way to do what you want.
    const TRACE: NcLogLevel = constants::NCLOGLEVEL_TRACE;,
    /// "detailed information.
    const VERBOSE: NcLogLevel = constants::NCLOGLEVEL_VERBOSE;,
    /// you probably don't want what's happening to happen.
    const WARNING: NcLogLevel = constants::NCLOGLEVEL_WARNING;
];

pub(crate) mod constants {
    use crate::NcLogLevel;

    /// this is honestly a bit much.
    pub const NCLOGLEVEL_DEBUG: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_DEBUG;

    /// we can't keep doin' this, but we can do other things.
    pub const NCLOGLEVEL_ERROR: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_ERROR;

    /// we're hanging around, but we've had a horrible fault.
    pub const NCLOGLEVEL_FATAL: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_FATAL;

    /// "detailed information.
    pub const NCLOGLEVEL_INFO: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_INFO;

    /// print diagnostics immediately related to crashing.
    pub const NCLOGLEVEL_PANIC: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_PANIC;

    /// default. print nothing once fullscreen service begins.
    pub const NCLOGLEVEL_SILENT: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_SILENT;

    /// there's probably a better way to do what you want.
    pub const NCLOGLEVEL_TRACE: NcLogLevel = crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_TRACE;

    /// "detailed information.
    pub const NCLOGLEVEL_VERBOSE: NcLogLevel =
        crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_VERBOSE;

    /// you probably don't want what's happening to happen.
    pub const NCLOGLEVEL_WARNING: NcLogLevel =
        crate::bindings::ffi::ncloglevel_e_NCLOGLEVEL_WARNING;
}
