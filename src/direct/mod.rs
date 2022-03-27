//! `NcDirect`

// total: 63
// ---------------------------------------------------
// (X)  1 : wont do
// (~)  3 : TODO / WIP
//
// (f) 46 : unsafe ffi function exported by bindgen
// (w)  1 : safely wrapped ffi function
// (r) 11 : static function manually reimplemented
//
// (m) 54 : method implemented
//
// (t)  0 : unit test done for the function
// (T)  0 : unit test done also for the method
// ---------------------------------------------------
// fm  ncdirect_bg_default
// fm  ncdirect_bg_palindex
// fm  ncdirect_bg_rgb
// fm  ncdirect_box
// rm  ncdirect_canbraille
// rm  ncdirect_canchangecolor
// fm  ncdirect_canget_cursor
// rm  ncdirect_canfade
// rm  ncdirect_canhalfblock
// rm  ncdirect_canopen_images
// rm  ncdirect_canopen_videos
// rm  ncdirect_canquadrant
// rm  ncdirect_cantruecolor
// fm  ncdirect_canutf8
// wm  ncdirect_capabilities
// fm  ncdirect_check_pixel_support
// fm  ncdirect_clear
//~f   ncdirect_core_init
// fm  ncdirect_cursor_disable
// fm  ncdirect_cursor_down
// fm  ncdirect_cursor_enable
// fm  ncdirect_cursor_left
// fm  ncdirect_cursor_move_yx
// fm  ncdirect_cursor_pop
// fm  ncdirect_cursor_push
// fm  ncdirect_cursor_right
// fm  ncdirect_cursor_up
// fm  ncdirect_cursor_yx
// fm  ncdirect_detected_terminal
// fm  ncdirect_dim_x
// fm  ncdirect_dim_y
// fm  ncdirect_double_box
// fm  ncdirect_fg_default
// fm  ncdirect_fg_palindex
// fm  ncdirect_fg_rgb
// fm  ncdirect_flush
// fm  ncdirect_get
//~r   ncdirect_heavy_box,
// fm  ncdirect_hline_interp
// fm  ncdirect_init
// fm  ncdirect_inputready_fd
//~r   ncdirect_light_box,
// fm  ncplane_on_styles
// fm  ncplane_off_styles
// fm  ncdirect_palette_size
//X    ncdirect_printf_aligned
// f   ncdirect_putegc
// fm  ncdirect_putstr
// fm  ncdirect_raster_frame
// fm  ncdirect_readline
// fm  ncdirect_render_frame
// fm  ncdirect_render_image
// fm  ncdirect_rounded_box
// fm  ncdirect_set_styles
// fm  ncdirect_stop
// f   ncdirect_stream
// f   ncdirect_styles
// f   ncdirect_supported_styles
// fm  ncdirect_vline_interp
// r   ncdirect_bg_rgb8           // unneeded method
// r   ncdirect_fg_rgb8           // unneeded method
// rm  ncdirect_get_blocking
// rm  ncdirect_get_nblock

#[cfg(test)]
mod test;

mod methods;
pub(crate) mod reimplemented;

use c_api::NcDirectFlag_u64;

/// Minimal notcurses instance for styling text.
pub type NcDirect = crate::c_api::ffi::ncdirect;

/// A bitmask of [`NcDirect`][crate::NcDirect] flags.
///
/// # Flag
/// - [`DrainInput`][NcDirectFlag::DrainInput]
/// - [`InhibitCbreak`][NcDirectFlag::InhibitCbreak]
/// - [`InhibitSetLocale`][NcDirectFlag::InhibitSetLocale]
/// - [`NoQuitSigHandlers`][NcDirectFlag::NoQuitSigHandlers]
/// - [`Verbose`][NcDirectFlag::Verbose]
/// - [`VeryVerbose`][NcDirectFlag::VeryVerbose]
///
/// # Default
/// *[`NcDirectFlag::None`]
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NcDirectFlag(pub NcDirectFlag_u64);

///
impl NcDirectFlag {
    /// No flags.
    pub const None: NcDirectFlag = Self(0);

    /// Flag that indicates input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub const DrainInput: NcDirectFlag =
        Self(c_api::NCDIRECT_OPTION_DRAIN_INPUT as NcDirectFlag_u64);

    /// Flag that avoids placing the terminal into cbreak mode
    /// (disabling echo and line buffering).
    ///
    pub const InhibitCbreak: NcDirectFlag =
        Self(c_api::NCDIRECT_OPTION_INHIBIT_CBREAK as NcDirectFlag_u64);

    /// Flag that avoids calling setlocale(LC_ALL, NULL).
    ///
    /// If the result is either "C" or "POSIX", it will print a
    /// diagnostic to stderr, and then call setlocale(LC_ALL, "").
    ///
    /// This will attempt to set the locale based off the LANG
    /// environment variable. Your program should call setlocale(3)
    /// itself, usually as one of the first lines.
    ///
    pub const InhibitSetLocale: NcDirectFlag =
        Self(c_api::NCDIRECT_OPTION_INHIBIT_SETLOCALE as NcDirectFlag_u64);

    /// Flag that inhibits registration of the `SIGABRT`, `SIGBUS`, `SIGFPE`,
    /// `SIGILL`, `SIGINT`, `SIGQUIT`, `SIGSEGV` and `SIGTERM`, signal handlers.
    pub const NoQuitSigHandlers: NcDirectFlag =
        Self(c_api::NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS as NcDirectFlag_u64);

    /// Flag that enables showing detailed information.
    pub const Verbose: NcDirectFlag = Self(c_api::NCDIRECT_OPTION_VERBOSE as NcDirectFlag_u64);

    /// Flag that enables showing all diagnostics (equivalent to
    /// [`NcLogLevel::trace`]. Implies [`NcDirectFlag::Verbose`].
    ///
    /// [`NcLogLevel::Trace`]: crate::NcLogLevel#associatedconstant.Trace
    pub const VeryVerbose: NcDirectFlag =
        Self(c_api::NCDIRECT_OPTION_VERY_VERBOSE as NcDirectFlag_u64);
}

mod std_impls {
    use super::{c_api::NcDirectFlag_u64, NcDirectFlag};

    impl Default for NcDirectFlag {
        fn default() -> Self {
            Self::None
        }
    }

    crate::from_primitive![NcDirectFlag, NcDirectFlag_u64];
    crate::unit_impl_from![NcDirectFlag, NcDirectFlag_u64];
    crate::unit_impl_ops![bitwise; NcDirectFlag, NcDirectFlag_u64];
    crate::unit_impl_fmt![bases+display; NcDirectFlag];
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// A bitmask of [`NcDirect`][crate::NcDirect] flags.
    ///
    /// It's recommended to use [`NcDirectFlag`][crate::NcDirectFlag] instead.
    ///
    /// # Associated `c_api` constants
    /// - [`NCDIRECT_OPTION_DRAIN_INPUT`]
    /// - [`NCDIRECT_OPTION_INHIBIT_CBREAK`]
    /// - [`NCDIRECT_OPTION_INHIBIT_SETLOCALE`]
    /// - [`NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS`]
    /// - [`NCDIRECT_OPTION_VERBOSE`]
    /// - [`NCDIRECT_OPTION_VERY_VERBOSE`]
    pub type NcDirectFlag_u64 = u64;

    /// [`NcDirectFlag_u64`] flag that indicates input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually
    /// preventing *notcurses* from processing terminal messages.
    pub const NCDIRECT_OPTION_DRAIN_INPUT: NcDirectFlag_u64 =
        ffi::NCDIRECT_OPTION_DRAIN_INPUT as NcDirectFlag_u64;

    /// [`NcDirectFlag_u64`] flag to avoid placing the terminal into cbreak
    /// mode (disabling echo and line buffering)
    pub const NCDIRECT_OPTION_INHIBIT_CBREAK: NcDirectFlag_u64 =
        ffi::NCDIRECT_OPTION_INHIBIT_CBREAK as NcDirectFlag_u64;

    /// [`NcDirectFlag_u64`] flag to avoid calling setlocale(LC_ALL, NULL)
    ///
    /// If the result is either "C" or "POSIX", it will print a
    /// diagnostic to stderr, and then call setlocale(LC_ALL, "").
    ///
    /// This will attempt to set the locale based off the LANG
    /// environment variable. Your program should call setlocale(3)
    /// itself, usually as one of the first lines.
    ///
    pub const NCDIRECT_OPTION_INHIBIT_SETLOCALE: NcDirectFlag_u64 =
        ffi::NCDIRECT_OPTION_INHIBIT_SETLOCALE as NcDirectFlag_u64;

    /// [`NcDirectFlag_u64`] flag that inhibits registration of the `SIGINT`,
    /// `SIGSEGV`, `SIGABRT` & `SIGQUIT` signal handlers.
    pub const NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS: NcDirectFlag_u64 =
        ffi::NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS as NcDirectFlag_u64;

    /// [`NcDirectFlag_u64`] flag that enables showing detailed information.
    pub const NCDIRECT_OPTION_VERBOSE: NcDirectFlag_u64 =
        ffi::NCDIRECT_OPTION_VERBOSE as NcDirectFlag_u64;

    /// [`NcDirectFlag_u64`] flag that enables showing all diagnostics
    /// (equivalent to [`NCLOGLEVEL_TRACE`]).
    /// Implies [`NCDIRECT_OPTION_VERBOSE`].
    ///
    /// [`NCLOGLEVEL_TRACE`]: crate::c_api::NCLOGLEVEL_TRACE
    pub const NCDIRECT_OPTION_VERY_VERBOSE: NcDirectFlag_u64 =
        ffi::NCDIRECT_OPTION_VERY_VERBOSE as NcDirectFlag_u64;
}
