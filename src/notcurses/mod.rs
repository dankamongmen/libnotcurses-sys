//! `Nc`

// total: 55
// ---------------------------------------------------
// (X)  1 : wont do
// (…)  4 : TODO / WIP
//
// (f) 36 : unsafe ffi function exported by bindgen
// (w)  0 : safely wrapped ffi function
// (r) 15 : static function manually reimplemented
//
// (m) 42 : method implemented
//
// (t) 13 : unit test done for the function
// (T)  0 : unit test done also for the method
// ---------------------------------------------------
// fm  notcurses_at_yx
// rm  notcurses_bottom
// rm  notcurses_canbraille
// rmt notcurses_canchangecolor
// rmt notcurses_canfade
// fmt notcurses_canopen_images
// fmt notcurses_canopen_videos
// rmt notcurses_cansextant
// rmt notcurses_cantruecolor
// rmt notcurses_canutf8
// fm  notcurses_check_pixel_support
//~f   notcurses_core_init
// fm  notcurses_cursor_disable
// fm  notcurses_cursor_enable
// f   notcurses_cursor_yx
// fmt notcurses_debug
// fm  notcurses_default_background
// fm  notcurses_default_foreground
//~f   notcurses_detected_terminal
// fmt notcurses_drop_planes
// fm  notcurses_get
// fm  notcurses_getvec
// fmt notcurses_init
// fm  notcurses_inputready_fd
// fm  notcurses_lex_blitter
// fm  notcurses_lex_margins
// fm  notcurses_lex_scalemode
// fm  notcurses_linesigs_disable
// fm  notcurses_linesigs_enable
// fm  notcurses_mice_enable
// rm  notcurses_mice_disable
// fm  notcurses_osversion
// fm  notcurses_palette_size
// fm  notcurses_refresh
// rm  notcurses_render
// fm  notcurses_stats
// fm  notcurses_stats_alloc
// fm  notcurses_stats_reset
// fm  notcurses_stdplane
// fm  notcurses_stdplane_const
// fmt notcurses_stop
// fm  notcurses_str_blitter
// fm  notcurses_str_scalemode
// fm  notcurses_supported_styles
// rm  notcurses_top
//X    notcurses_ucs32_to_utf8 (not needed in rust)
// fmt notcurses_version
// fm  notcurses_version_components
// rmt notcurses_align
// rm  notcurses_canpixel
// rm  notcurses_get_blocking
// rm  notcurses_get_nblock
//~r   notcurses_stddim_yx           // multiple mutable references errors
//~r   notcurses_stddim_yx_const     //
// rm  notcurses_term_dim_yx

mod methods;

pub(crate) mod helpers;
pub(crate) mod reimplemented;

#[cfg(test)]
mod test;

/// Notcurses state for a given terminal, composed of [`NcPlane`]s.
///
/// It's built atop the terminfo abstraction layer to provide reasonably
/// portable vivid character displays.
///
/// [`NcPlane`]: crate::NcPlane
pub type Nc = crate::c_api::ffi::notcurses;

/// Options struct for [`Nc`]
pub type NcOptions = crate::c_api::ffi::notcurses_options;

/// A bitmask of [`NcOptions`] flags.
///
/// # Flags
/// - [`None`][NcFlag::None]
/// - [`DrainInput`][NcFlag::DrainInput]
/// - [`InhibitSetLocale`][NcFlag::InhibitSetLocale]
/// - [`NoAlternateScreen`][NcFlag::NoAlternateScreen]
/// - [`NoClearBitmaps`][NcFlag::NoClearBitmaps]
/// - [`NoFontChanges`][NcFlag::NoFontChanges]
/// - [`NoQuitSigHandlers`][NcFlag::NoQuitSigHandlers]
/// - [`NoWinchSigHandler`][NcFlag::NoWinchSigHandler]
/// - [`PreserveCursor`][NcFlag::PreserveCursor]
/// - [`Scrolling`][NcFlag::Scrolling]
/// - [`CliMode`][NcFlag::CliMode]
/// - [`SuppressBanners`][NcFlag::SuppressBanners]
///
/// # Default
/// *[`NcFlag::None`]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NcFlag(pub c_api::NcFlag_u64);

impl NcFlag {
    /// No flags.
    pub const None: Self = Self(0);

    /// Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub const DrainInput: Self = Self(c_api::NCOPTION_DRAIN_INPUT);

    /// Do not call setlocale().
    ///
    /// notcurses_init() will call setlocale() to inspect the current locale. If
    /// that locale is "C" or "POSIX", it will call setlocale(LC_ALL, "") to set
    /// the locale according to the LANG environment variable. Ideally, this will
    /// result in UTF8 being enabled, even if the client app didn't call
    /// setlocale() itself. Unless you're certain that you're invoking setlocale()
    /// prior to notcurses_init(), you should not set this bit. Even if you are
    /// invoking setlocale(), this behavior shouldn't be an issue unless you're
    /// doing something weird (setting a locale not based on LANG).
    pub const InhibitSetLocale: Self = Self(c_api::NCOPTION_INHIBIT_SETLOCALE);

    /// Do not enter alternate mode.
    ///
    /// If smcup/rmcup capabilities are indicated, notcurses defaults to making use
    /// of the "alternate screen". This flag inhibits use of smcup/rmcup.
    pub const NoAlternateScreen: Self = Self(c_api::NCOPTION_NO_ALTERNATE_SCREEN);

    /// Do not try to clear any preexisting bitmaps.
    ///
    /// Note that they might still get cleared even if this is set, and they might
    /// not get cleared even if this is not set.
    pub const NoClearBitmaps: Self = Self(c_api::NCOPTION_NO_CLEAR_BITMAPS);

    /// Do not modify the font.
    ///
    /// Notcurses might attempt to change the font slightly, to support certain
    /// glyphs (especially on the Linux console). If this is set, no such
    /// modifications will be made. Note that font changes will not affect anything
    /// but the virtual console/terminal in which notcurses is running.
    pub const NoFontChanges: Self = Self(c_api::NCOPTION_NO_FONT_CHANGES);

    /// Do not handle `SIGINT`, `SIGSEGV`, `SIGABRT`, `SIGQUIT`.
    ///
    /// A signal handler will usually be installed for `SIGABRT`, `SIGBUS`,
    /// `SIGFPE`, `SIGILL`, `SIGINT`, `SIGQUIT`, `SIGSEGV` and `SIGTERM`,
    /// cleaning up the terminal on such exceptions.
    ///
    /// With this flag, the handler will not be installed.
    pub const NoQuitSigHandlers: Self = Self(c_api::NCOPTION_NO_QUIT_SIGHANDLERS);

    /// Do not handle `SIGWINCH`.
    ///
    /// A signal handler will usually be installed for `SIGWINCH`, resulting in
    /// [`Nckey::Resize`] events being generated on input. With this flag,
    /// the handler will not be installed.
    ///
    /// [`Nckey::Resize`]: crate::NcKey#associatedconstant.Resize
    pub const NoWinchSigHandler: Self = Self(c_api::NCOPTION_NO_WINCH_SIGHANDLER);

    /// Initializes the standard plane's virtual cursor to match the physical
    /// cursor at context creation time.
    ///
    /// Together with [`NoAlternateScreen`] and a scrolling standard plane,
    /// this facilitates easy scrolling-style programs in rendered mode.
    ///
    /// [`NoAlternateScreen`]: NcFlag#associatedconstant.NoAlternateScreen
    pub const PreserveCursor: Self = Self(c_api::NCOPTION_PRESERVE_CURSOR);

    /// Prepares the standard plane in scrolling mode, useful for CLIs. This is
    /// equivalent to calling [`ncplane_set_scrolling`]`(notcurses_stdplane(nc), true)`.
    ///
    /// [`ncplane_set_scrolling`]: crate::c_api::ncplane_set_scrolling
    pub const Scrolling: Self = Self(c_api::NCOPTION_SCROLLING);

    /// "CLI mode" is just setting these four options:
    ///
    /// [`NoAlternateScreen`][Self::NoAlternateScreen]
    /// | [`NoClearBitmaps`][Self::NoClearBitmaps]
    /// | [`PreserveCursor`][Self::PreserveCursor]
    /// | [`Scrolling`][Self::Scrolling]
    pub const CliMode: Self = Self(
        c_api::NCOPTION_NO_ALTERNATE_SCREEN
            | c_api::NCOPTION_NO_CLEAR_BITMAPS
            | c_api::NCOPTION_PRESERVE_CURSOR
            | c_api::NCOPTION_SCROLLING,
    );

    /// Do not print banners.
    ///
    /// Notcurses typically prints version info in `notcurses_init` and
    /// performance info in `notcurses_stop`. This inhibits that output.
    pub const SuppressBanners: Self = Self(c_api::NCOPTION_SUPPRESS_BANNERS);
}

mod std_impls {
    use super::{c_api::NcFlag_u64, NcFlag};

    impl Default for NcFlag {
        fn default() -> Self {
            Self::None
        }
    }

    crate::from_primitive![NcFlag, NcFlag_u64];
    crate::unit_impl_from![NcFlag, NcFlag_u64];
    crate::unit_impl_ops![bitwise; NcFlag, NcFlag_u64];
    crate::unit_impl_fmt![bases+display; NcFlag];
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// A bitmask of [`Nc`][crate::Nc] flags.
    ///
    /// It's recommended to use [`NcFlag`][crate::NcFlag] instead.
    ///
    /// # Associated `c_api` constants
    /// - [`NCOPTION_DRAIN_INPUT`]
    /// - [`NCOPTION_INHIBIT_SETLOCALE`]
    /// - [`NCOPTION_NO_ALTERNATE_SCREEN`]
    /// - [`NCOPTION_NO_CLEAR_BITMAPS`]
    /// - [`NCOPTION_NO_FONT_CHANGES`]
    /// - [`NCOPTION_NO_QUIT_SIGHANDLERS`]
    /// - [`NCOPTION_NO_WINCH_SIGHANDLER`]
    /// - [`NCOPTION_PRESERVE_CURSOR`]
    /// - [`NCOPTION_SCROLLING`]
    /// - [`NCOPTION_CLI_MODE`]
    /// - [`NCOPTION_SUPPRESS_BANNERS`]
    pub type NcFlag_u64 = u64;

    /// [`NcFlag_u64`] flag that indicates input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    pub const NCOPTION_DRAIN_INPUT: NcFlag_u64 = ffi::NCOPTION_DRAIN_INPUT as NcFlag_u64;

    /// [`NcFlag_u64`] flag to avoid calling setlocale().
    ///
    /// notcurses_init() will call setlocale() to inspect the current locale. If
    /// that locale is "C" or "POSIX", it will call setlocale(LC_ALL, "") to set
    /// the locale according to the LANG environment variable. Ideally, this will
    /// result in UTF8 being enabled, even if the client app didn't call
    /// setlocale() itself. Unless you're certain that you're invoking setlocale()
    /// prior to notcurses_init(), you should not set this bit. Even if you are
    /// invoking setlocale(), this behavior shouldn't be an issue unless you're
    /// doing something weird (setting a locale not based on LANG).
    pub const NCOPTION_INHIBIT_SETLOCALE: NcFlag_u64 =
        ffi::NCOPTION_INHIBIT_SETLOCALE as NcFlag_u64;

    /// [`NcFlag_u64`] flag to avoid entering alternate mode.
    ///
    /// If smcup/rmcup capabilities are indicated, notcurses defaults to making use
    /// of the "alternate screen". This flag inhibits use of smcup/rmcup.
    pub const NCOPTION_NO_ALTERNATE_SCREEN: NcFlag_u64 =
        ffi::NCOPTION_NO_ALTERNATE_SCREEN as NcFlag_u64;

    /// [`NcFlag_u64`] flag to avoid trying to clear any preexisting bitmaps.
    ///
    /// Note that they might still get cleared even if this is set, and they might
    /// not get cleared even if this is not set.
    pub const NCOPTION_NO_CLEAR_BITMAPS: NcFlag_u64 = ffi::NCOPTION_NO_CLEAR_BITMAPS as NcFlag_u64;

    /// [`NcFlag_u64`] flag to avoid modifying the font.
    ///
    /// Notcurses might attempt to change the font slightly, to support certain
    /// glyphs (especially on the Linux console). If this is set, no such
    /// modifications will be made. Note that font changes will not affect anything
    /// but the virtual console/terminal in which notcurses is running.
    pub const NCOPTION_NO_FONT_CHANGES: NcFlag_u64 = ffi::NCOPTION_NO_FONT_CHANGES as NcFlag_u64;

    /// [`NcFlag_u64`] flag to avoid handling `SIGINT`, `SIGSEGV`,
    /// `SIGABRT`, `SIGQUIT`.
    ///
    /// A signal handler will usually be installed for `SIGINT`, `SIGQUIT`,
    /// `SIGSEGV`, `SIGTERM`, and `SIGABRT`, cleaning up the terminal on such
    /// exceptions. With this flag, the handler will not be installed.
    pub const NCOPTION_NO_QUIT_SIGHANDLERS: NcFlag_u64 =
        ffi::NCOPTION_NO_QUIT_SIGHANDLERS as NcFlag_u64;

    /// [`NcFlag_u64`] flag to avoid handling `SIGWINCH`.
    ///
    /// A signal handler will usually be installed for `SIGWINCH`, resulting in
    /// [`NCKEY_RESIZE`][crate::c_api::NCKEY_RESIZE] events being generated on
    /// input. With this flag, the handler will not be installed.
    pub const NCOPTION_NO_WINCH_SIGHANDLER: NcFlag_u64 =
        ffi::NCOPTION_NO_WINCH_SIGHANDLER as NcFlag_u64;

    /// [`NcFlag_u64`] flag to initialize the standard plane's virtual cursor
    /// to match the physical cursor at context creation time.
    ///
    /// Together with [`NCOPTION_NO_ALTERNATE_SCREEN`] and a scrolling standard
    /// plane, this facilitates easy scrolling-style programs in rendered mode.
    pub const NCOPTION_PRESERVE_CURSOR: NcFlag_u64 = ffi::NCOPTION_PRESERVE_CURSOR as NcFlag_u64;

    /// [`NcFlag_u64`] flag to prepare the standard plane in scrolling mode,
    /// useful for CLIs. This is equivalent to calling
    /// [`ncplane_set_scrolling`]`(notcurses_stdplane(nc), true)`.
    ///
    /// [`ncplane_set_scrolling`]: crate::c_api::ncplane_set_scrolling
    pub const NCOPTION_SCROLLING: NcFlag_u64 = ffi::NCOPTION_SCROLLING as NcFlag_u64;

    /// [`NcFlag_u64`] flag set composed of `NCOPTION_NO_ALTERNATE_SCREEN` |
    /// `NCOPTION_NO_CLEAR_BITMAPS` | `NCOPTION_PRESERVE_CURSOR` |
    /// `NCOPTION_SCROLLING`.
    pub const NCOPTION_CLI_MODE: NcFlag_u64 = NCOPTION_NO_ALTERNATE_SCREEN
        | NCOPTION_NO_CLEAR_BITMAPS
        | NCOPTION_PRESERVE_CURSOR
        | NCOPTION_SCROLLING;

    /// [`NcFlag_u64`] flag to avoid printing banners.
    ///
    /// Notcurses typically prints version info in notcurses_init() and performance
    /// info in notcurses_stop(). This inhibits that output.
    pub const NCOPTION_SUPPRESS_BANNERS: NcFlag_u64 = ffi::NCOPTION_SUPPRESS_BANNERS as NcFlag_u64;
}
