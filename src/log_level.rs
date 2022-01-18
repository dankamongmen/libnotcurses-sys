//!

/// Log level for [`NcOptions`][crate::NcOptions].
///
/// # Default
///
///
/// These log levels consciously map cleanly to those of libav; notcurses itself
/// does not use this full granularity. The log level does not affect the opening
/// and closing banners, which can be disabled via `NcOptions::SUPPRESS_BANNERS`.
///
/// Note that if stderr is connected to the same terminal on which we're
/// rendering, any kind of logging will disrupt the output.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcLogLevel {
    /// Default. print nothing once fullscreen service begins.
    Silent = c_api::NCLOGLEVEL_SILENT,

    /// Print diagnostics immediately related to crashing.
    Panic = c_api::NCLOGLEVEL_PANIC,

    /// We're hanging around, but we've had a horrible fault.
    Fatal = c_api::NCLOGLEVEL_FATAL,

    /// We can't keep doin' this, but we can do other things.
    Error = c_api::NCLOGLEVEL_ERROR,

    /// You probably don't want what's happening to happen.
    Warning = c_api::NCLOGLEVEL_WARNING,

    /// "Standard information".
    Info = c_api::NCLOGLEVEL_INFO,

    /// "Detailed information".
    Verbose = c_api::NCLOGLEVEL_VERBOSE,

    /// This is honestly a bit much.
    Debug = c_api::NCLOGLEVEL_DEBUG,

    /// There's probably a better way to do what you want.
    Trace = c_api::NCLOGLEVEL_TRACE,
}

mod std_impls {
    use super::{c_api, NcLogLevel};
    use std::fmt;

    impl Default for NcLogLevel {
        fn default() -> Self {
            Self::Silent
        }
    }

    impl fmt::Display for NcLogLevel {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcLogLevel::*;
            write!(
                f,
                "{}",
                match self {
                    Silent => "Silent",
                    Panic => "Panic",
                    Fatal => "Fatal",
                    Error => "Error",
                    Warning => "Warning",
                    Info => "Info",
                    Verbose => "Verbose",
                    Debug => "Debug",
                    Trace => "Trace",
                }
            )
        }
    }

    impl From<c_api::NcLogLevel_i32> for NcLogLevel {
        fn from(log_level: c_api::NcLogLevel_i32) -> Self {
            use {c_api::*, NcLogLevel::*};
            match log_level {
                NCLOGLEVEL_SILENT => Silent,
                NCLOGLEVEL_PANIC => Panic,
                NCLOGLEVEL_FATAL => Fatal,
                NCLOGLEVEL_ERROR => Error,
                NCLOGLEVEL_WARNING => Warning,
                NCLOGLEVEL_INFO => Info,
                NCLOGLEVEL_VERBOSE => Verbose,
                NCLOGLEVEL_DEBUG => Debug,
                NCLOGLEVEL_TRACE => Trace,
                _ => Self::default(),
            }
        }
    }

    impl From<NcLogLevel> for c_api::NcLogLevel_i32 {
        fn from(loglevel: NcLogLevel) -> Self {
            use {c_api::*, NcLogLevel::*};
            match loglevel {
                Silent => NCLOGLEVEL_SILENT,
                Panic => NCLOGLEVEL_PANIC,
                Fatal => NCLOGLEVEL_FATAL,
                Error => NCLOGLEVEL_ERROR,
                Warning => NCLOGLEVEL_WARNING,
                Info => NCLOGLEVEL_INFO,
                Verbose => NCLOGLEVEL_VERBOSE,
                Debug => NCLOGLEVEL_DEBUG,
                Trace => NCLOGLEVEL_TRACE,
            }
        }
    }
}

pub(crate) mod c_api {
    use crate::bindings::ffi;

    /// Log level for [`NcOptions`][crate::NcOptions].
    ///
    /// It's recommended to use [`NcLogLevel`][crate::NcLogLevel] instead.
    ///
    /// These log levels consciously map cleanly to those of libav; notcurses itself
    /// does not use this full granularity. The log level does not affect the opening
    /// and closing banners, which can be disabled via the `NcOptions`
    /// `NCOPTION_SUPPRESS_BANNERS`.
    ///
    /// Note that if stderr is connected to the same terminal on which we're
    /// rendering, any kind of logging will disrupt the output.
    pub type NcLogLevel_i32 = ffi::ncloglevel_e;

    /// [`NcLogLevel_i32`] this is honestly a bit much.
    pub const NCLOGLEVEL_DEBUG: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_DEBUG;

    /// [`NcLogLevel_i32`] we can't keep doin' this, but we can do other things.
    pub const NCLOGLEVEL_ERROR: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_ERROR;

    /// [`NcLogLevel_i32`] we're hanging around, but we've had a horrible fault.
    pub const NCLOGLEVEL_FATAL: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_FATAL;

    /// [`NcLogLevel_i32`] "detailed information.
    pub const NCLOGLEVEL_INFO: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_INFO;

    /// [`NcLogLevel_i32`] print diagnostics immediately related to crashing.
    pub const NCLOGLEVEL_PANIC: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_PANIC;

    /// [`NcLogLevel_i32`] default. print nothing once fullscreen service begins.
    pub const NCLOGLEVEL_SILENT: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_SILENT;

    /// [`NcLogLevel_i32`] there's probably a better way to do what you want.
    pub const NCLOGLEVEL_TRACE: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_TRACE;

    /// [`NcLogLevel_i32`] "detailed information.
    pub const NCLOGLEVEL_VERBOSE: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_VERBOSE;

    /// [`NcLogLevel_i32`] you probably don't want what's happening to happen.
    pub const NCLOGLEVEL_WARNING: NcLogLevel_i32 = ffi::ncloglevel_e_NCLOGLEVEL_WARNING;
}
