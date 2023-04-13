/// A bitmask of [`NcOptions`][crate::NcOptions] flags.
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

mod core_impls {
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
