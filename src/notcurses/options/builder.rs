//!

use crate::{NcFlag, NcLogLevel, NcOptions};
use core::ptr::null;

/// Builder object for [`NcOptions`].
#[derive(Clone, Copy, Debug, Default)]
pub struct NcOptionsBuilder {
    // pub(crate): termtype: String,
    pub(crate) loglevel: NcLogLevel,
    pub(crate) margin_t: u32,
    pub(crate) margin_r: u32,
    pub(crate) margin_b: u32,
    pub(crate) margin_l: u32,
    pub(crate) flags: u64,
}

mod std_impls {
    use super::{NcOptions, NcOptionsBuilder};

    impl From<NcOptionsBuilder> for NcOptions {
        fn from(builder: NcOptionsBuilder) -> NcOptions {
            builder.build()
        }
    }
    impl From<&NcOptionsBuilder> for NcOptions {
        fn from(builder: &NcOptionsBuilder) -> Self {
            builder.build()
        }
    }
    impl From<&mut NcOptionsBuilder> for NcOptions {
        fn from(builder: &mut NcOptionsBuilder) -> Self {
            builder.build()
        }
    }
    //
    impl From<NcOptions> for NcOptionsBuilder {
        fn from(options: NcOptions) -> NcOptionsBuilder {
            Self::from_options(&options)
        }
    }
    impl From<&NcOptions> for NcOptionsBuilder {
        fn from(options: &NcOptions) -> Self {
            Self::from_options(options)
        }
    }
    impl From<&mut NcOptions> for NcOptionsBuilder {
        fn from(options: &mut NcOptions) -> Self {
            Self::from_options(options)
        }
    }
}

/// # constructors
impl NcOptionsBuilder {
    /// New `NcOptionsBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// New builder from pre-existing options.
    pub fn from_options(options: &NcOptions) -> Self {
        let mut builder = Self::default();

        if options.is_drain_input() {
            builder = builder.drain_input(true);
        }

        if options.is_inhibit_set_locale() {
            builder = builder.inhibit_set_locale(true);
        }

        if options.is_no_alternate_screen() {
            builder = builder.no_alternate_screen(true);
        }

        if options.is_no_font_changes() {
            builder = builder.no_font_changes(true);
        }

        if options.is_no_quit_sig_handlers() {
            builder = builder.no_quit_sig_handlers(true);
        }

        if options.is_no_winch_sig_handler() {
            builder = builder.no_winch_sig_handler(true);
        }

        if options.is_preserve_cursor() {
            builder = builder.preserve_cursor(true);
        }

        if options.is_scrolling() {
            builder = builder.scrolling(true);
        }

        if options.is_cli_mode() {
            builder = builder.cli_mode(true);
        }

        if options.is_suppress_banners() {
            builder = builder.suppress_banners(true);
        }
        builder
    }

    /// Finishes the building and returns [`NcOptions`].
    pub fn build(self) -> NcOptions {
        NcOptions {
            termtype: null(),
            margin_t: self.margin_t,
            margin_r: self.margin_r,
            margin_b: self.margin_b,
            margin_l: self.margin_l,
            loglevel: self.loglevel.into(),
            flags: self.flags,
        }
    }
}

/// # methods (chainable)
impl NcOptionsBuilder {
    // /// Sets the TERM type.
    // pub fn term_type(mut self, term_type: &str) -> Self {
    //     self.termtype = term_type;
    //     self
    // }

    /// Sets the log level.
    pub fn log_level(mut self, log_level: NcLogLevel) -> Self {
        self.loglevel = log_level;
        self
    }

    /// Sets the margins.
    pub fn margins(mut self, top: u32, right: u32, bottom: u32, left: u32) -> Self {
        self.margin_t = top;
        self.margin_r = right;
        self.margin_b = bottom;
        self.margin_l = left;
        self
    }

    /// Sets the top margin.
    pub fn margin_top(mut self, top: u32) -> Self {
        self.margin_t = top;
        self
    }

    /// Sets the right margin.
    pub fn margin_right(mut self, right: u32) -> Self {
        self.margin_r = right;
        self
    }

    /// Sets the bottom margin.
    pub fn margin_bottom(mut self, bottom: u32) -> Self {
        self.margin_b = bottom;
        self
    }

    /// Sets the left margin.
    pub fn margin_left(mut self, left: u32) -> Self {
        self.margin_l = left;
        self
    }

    // flags

    /// If `true`, Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    ///
    /// See `NcFlag::`[`DrainInput`][NcFlag#associatedconstant.DrainInput].
    pub fn drain_input(mut self, drain: bool) -> Self {
        if drain {
            self.flags |= NcFlag::DrainInput;
        } else {
            self.flags &= !NcFlag::DrainInput;
        }
        self
    }

    /// If `true`, wont call setlocale().
    ///
    /// See `NcFlag::`[`InhibitSetLocale`][NcFlag#associatedconstant.InhibitSetLocale].
    pub fn inhibit_set_locale(mut self, inhibit: bool) -> Self {
        if inhibit {
            self.flags |= NcFlag::InhibitSetLocale;
        } else {
            self.flags &= !NcFlag::InhibitSetLocale;
        }
        self
    }

    /// If `true`, wont enter alternate mode.
    ///
    /// See `NcFlag::`[`NoAlternateScreen`][NcFlag#associatedconstant.NoAlternateScreen].
    pub fn no_alternate_screen(mut self, no_alternate: bool) -> Self {
        if no_alternate {
            self.flags |= NcFlag::NoAlternateScreen;
        } else {
            self.flags &= !NcFlag::NoAlternateScreen;
        }
        self
    }

    /// If `true`, wont try to clear any preexisting bitmaps.
    ///
    /// See `NcFlag::`[`NoClearBitmaps`][NcFlag#associatedconstant.NoClearBitmaps].
    pub fn no_clear_bitmaps(mut self, no_clear: bool) -> Self {
        if no_clear {
            self.flags |= NcFlag::NoClearBitmaps;
        } else {
            self.flags &= !NcFlag::NoClearBitmaps;
        }
        self
    }

    /// If `true`, wont modify the font.
    ///
    /// See `NcFlag::`[`NoFontChanges`][NcFlag#associatedconstant.NoFontChanges].
    pub fn no_font_changes(mut self, no_font_changes: bool) -> Self {
        if no_font_changes {
            self.flags |= NcFlag::NoFontChanges;
        } else {
            self.flags &= !NcFlag::NoFontChanges;
        }
        self
    }

    /// If `true`, wont handle `SIGINT`, `SIGSEGV`, `SIGABRT` nor `SIGQUIT`.
    ///
    /// See `NcFlag::`[`NoQuitSigHandlers`][NcFlag#associatedconstant.NoQuitSigHandlers].
    pub fn no_quit_sig_handlers(mut self, no_quit: bool) -> Self {
        if no_quit {
            self.flags |= NcFlag::NoQuitSigHandlers;
        } else {
            self.flags &= !NcFlag::NoQuitSigHandlers;
        }
        self
    }

    /// If `true`, wont handle `SIGWINCH`.
    ///
    /// See `NcFlag::`[`NoWinchSigHandler`][NcFlag#associatedconstant.NoWinchSigHandler].
    pub fn no_winch_sig_handler(mut self, no_winch: bool) -> Self {
        if no_winch {
            self.flags |= NcFlag::NoWinchSigHandler;
        } else {
            self.flags &= !NcFlag::NoWinchSigHandler;
        }
        self
    }

    /// If `true`, will initializes the CLI plane’s virtual cursor to match
    /// the physical cursor at context creation time.
    ///
    /// See `NcFlag::`[`PreserveCursor`][NcFlag#associatedconstant.PreserveCursor].
    pub fn preserve_cursor(mut self, preserve: bool) -> Self {
        if preserve {
            self.flags |= NcFlag::PreserveCursor;
        } else {
            self.flags &= !NcFlag::PreserveCursor;
        }
        self
    }

    /// If `true`, will prepare the CLI plane in scrolling mode.
    ///
    /// See `NcFlag::`[`Scrolling`][NcFlag#associatedconstant.Scrolling].
    pub fn scrolling(mut self, scrolling: bool) -> Self {
        if scrolling {
            self.flags |= NcFlag::Scrolling;
        } else {
            self.flags &= !NcFlag::Scrolling;
        }
        self
    }

    /// A shortcut for setting the following options together:
    /// `no_alternate_screen`, `no_clear_bitmaps`, `preserve_cursor` & `scrolling`.
    ///
    /// See `NcFlag::`[`CliMode`][NcFlag#associatedconstant.CliMode].
    pub fn cli_mode(mut self, cli_mode: bool) -> Self {
        if cli_mode {
            self.flags |= NcFlag::CliMode;
        } else {
            self.flags &= !NcFlag::CliMode;
        }
        self
    }

    /// If `true`, wont print banners.
    ///
    /// See `NcFlag::`[`SuppressBanners`][NcFlag#associatedconstant.SuppressBanners].
    pub fn suppress_banners(mut self, suppress_banners: bool) -> Self {
        if suppress_banners {
            self.flags |= NcFlag::SuppressBanners;
        } else {
            self.flags &= !NcFlag::SuppressBanners;
        }
        self
    }
}

/// # methods (settable)
impl NcOptionsBuilder {
    // /// Sets the TERM type.
    // pub fn set_term_type(&mut self, term_type: &str) {
    //     self.termtype = term_type;
    // }

    /// Sets the log level.
    pub fn set_log_level(&mut self, log_level: NcLogLevel) {
        self.loglevel = log_level;
    }

    /// Sets the margins.
    pub fn set_margins(&mut self, top: u32, right: u32, bottom: u32, left: u32) {
        self.margin_t = top;
        self.margin_r = right;
        self.margin_b = bottom;
        self.margin_l = left;
    }

    /// Sets the top margin.
    pub fn set_margin_top(&mut self, top: u32) {
        self.margin_t = top;
    }

    /// Sets the right margin.
    pub fn set_margin_right(&mut self, right: u32) {
        self.margin_r = right;
    }

    /// Sets the bottom margin.
    pub fn set_margin_bottom(&mut self, bottom: u32) {
        self.margin_b = bottom;
    }

    /// Sets the left margin.
    pub fn set_margin_left(&mut self, left: u32) {
        self.margin_l = left;
    }

    // flags

    /// If `true`, Input may be freely dropped.
    ///
    /// This ought be provided when the program does not intend to handle input.
    /// Otherwise, input can accumulate in internal buffers, eventually preventing
    /// Notcurses from processing terminal messages.
    ///
    /// See `NcFlag::`[`DrainInput`][NcFlag#associatedconstant.DrainInput].
    pub fn set_drain_input(&mut self, drain: bool) {
        if drain {
            self.flags |= NcFlag::DrainInput;
        } else {
            self.flags &= !NcFlag::DrainInput;
        }
    }

    /// If `true`, wont call setlocale().
    ///
    /// See `NcFlag::`[`InhibitSetLocale`][NcFlag#associatedconstant.InhibitSetLocale].
    pub fn set_inhibit_set_locale(&mut self, inhibit: bool) {
        if inhibit {
            self.flags |= NcFlag::InhibitSetLocale;
        } else {
            self.flags &= !NcFlag::InhibitSetLocale;
        }
    }

    /// If `true`, wont enter alternate mode.
    ///
    /// See `NcFlag::`[`NoAlternateScreen`][NcFlag#associatedconstant.NoAlternateScreen].
    pub fn set_no_alternate_screen(&mut self, no_alternate: bool) {
        if no_alternate {
            self.flags |= NcFlag::NoAlternateScreen;
        } else {
            self.flags &= !NcFlag::NoAlternateScreen;
        }
    }

    /// If `true`, wont try to clear any preexisting bitmaps.
    ///
    /// See `NcFlag::`[`NoClearBitmaps`][NcFlag#associatedconstant.NoClearBitmaps].
    pub fn set_no_clear_bitmaps(&mut self, no_clear: bool) {
        if no_clear {
            self.flags |= NcFlag::NoClearBitmaps;
        } else {
            self.flags &= !NcFlag::NoClearBitmaps;
        }
    }

    /// If `true`, wont modify the font.
    ///
    /// See `NcFlag::`[`NoFontChanges`][NcFlag#associatedconstant.NoFontChanges].
    pub fn set_no_font_changes(&mut self, no_font_changes: bool) {
        if no_font_changes {
            self.flags |= NcFlag::NoFontChanges;
        } else {
            self.flags &= !NcFlag::NoFontChanges;
        }
    }

    /// If `true`, wont handle `SIGINT`, `SIGSEGV`, `SIGABRT` nor `SIGQUIT`.
    ///
    /// See `NcFlag::`[`NoQuitSigHandlers`][NcFlag#associatedconstant.NoQuitSigHandlers].
    pub fn set_no_quit_sig_handlers(&mut self, no_quit: bool) {
        if no_quit {
            self.flags |= NcFlag::NoQuitSigHandlers;
        } else {
            self.flags &= !NcFlag::NoQuitSigHandlers;
        }
    }

    /// If `true`, wont handle `SIGWINCH`.
    ///
    /// See `NcFlag::`[`NoWinchSigHandler`][NcFlag#associatedconstant.NoWinchSigHandler].
    pub fn set_no_winch_sig_handler(&mut self, no_winch: bool) {
        if no_winch {
            self.flags |= NcFlag::NoWinchSigHandler;
        } else {
            self.flags &= !NcFlag::NoWinchSigHandler;
        }
    }

    /// If `true`, will initializes the CLI plane’s virtual cursor to match
    /// the physical cursor at context creation time.
    ///
    /// See `NcFlag::`[`PreserveCursor`][NcFlag#associatedconstant.PreserveCursor].
    pub fn set_preserve_cursor(&mut self, preserve: bool) {
        if preserve {
            self.flags |= NcFlag::PreserveCursor;
        } else {
            self.flags &= !NcFlag::PreserveCursor;
        }
    }

    /// If `true`, will prepare the CLI plane in scrolling mode.
    ///
    /// See `NcFlag::`[`Scrolling`][NcFlag#associatedconstant.Scrolling].
    pub fn set_scrolling(&mut self, scrolling: bool) {
        if scrolling {
            self.flags |= NcFlag::Scrolling;
        } else {
            self.flags &= !NcFlag::Scrolling;
        }
    }

    /// A shortcut for setting the following options together:
    /// `no_alternate_screen`, `no_clear_bitmaps`, `preserve_cursor` & `scrolling`.
    ///
    /// See `NcFlag::`[`CliMode`][NcFlag#associatedconstant.CliMode].
    pub fn set_cli_mode(&mut self, cli_mode: bool) {
        if cli_mode {
            self.flags |= NcFlag::CliMode;
        } else {
            self.flags &= !NcFlag::CliMode;
        }
    }

    /// If `true`, wont print banners.
    ///
    /// See `NcFlag::`[`SuppressBanners`][NcFlag#associatedconstant.SuppressBanners].
    pub fn set_suppress_banners(&mut self, suppress_banners: bool) {
        if suppress_banners {
            self.flags |= NcFlag::SuppressBanners;
        } else {
            self.flags &= !NcFlag::SuppressBanners;
        }
    }
}

/// # query methods
impl NcOptionsBuilder {
    /// Returns the `(top, right, bottom, left)` margins.
    pub fn get_margins(&self) -> (u32, u32, u32, u32) {
        (self.margin_t, self.margin_r, self.margin_b, self.margin_l)
    }

    /// Returns the log level.
    pub fn get_log_level(&self) -> NcLogLevel {
        self.loglevel
    }

    //

    /// Returns `true` if it has the [`DrainInput`] flag set.
    ///
    /// [`DrainInput`]: NcFlag#associatedconstant.DrainInput
    pub fn is_drain_input(&self) -> bool {
        self.flags & NcFlag::DrainInput != NcFlag::None
    }

    /// Returns `true` if it has the [`InhibitSetLocale`] flag set.
    ///
    /// [`InhibitSetLocale`]: NcFlag#associatedconstant.InhibitSetLocale
    pub fn is_inhibit_set_locale(&self) -> bool {
        self.flags & NcFlag::InhibitSetLocale != NcFlag::None
    }

    /// Returns `true` if it has the [`NoAlternateScreen`] flag set.
    ///
    /// [`NoAlternateScreen`]: NcFlag#associatedconstant.NoAlternateScreen
    pub fn is_no_alternate_screen(&self) -> bool {
        self.flags & NcFlag::NoAlternateScreen != NcFlag::None
    }

    /// Returns `true` if it has the [`NoClearBitmaps`] flag set.
    ///
    /// [`NoClearBitmaps`]: NcFlag#associatedconstant.NoClearBitmaps
    pub fn is_no_clear_bitmaps(&self) -> bool {
        self.flags & NcFlag::NoClearBitmaps != NcFlag::None
    }

    /// Returns `true` if it has the [`NoFontChanges`] flag set.
    ///
    /// [`NoFontChanges`]: NcFlag#associatedconstant.NoFontChanges
    pub fn is_no_font_changes(&self) -> bool {
        self.flags & NcFlag::NoFontChanges != NcFlag::None
    }

    /// Returns `true` if it has the [`NoQuitSigHandlers`] flag set.
    ///
    /// [`NoQuitSigHandlers`]: NcFlag#associatedconstant.NoQuitSigHandlers
    pub fn is_no_quit_sig_handlers(&self) -> bool {
        self.flags & NcFlag::NoQuitSigHandlers != NcFlag::None
    }

    /// Returns `true` if it has the [`NoWinchSigHandler`] flag set.
    ///
    /// [`NoWinchSigHandler`]: NcFlag#associatedconstant.NoWinchSigHandler
    pub fn is_no_winch_sig_handler(&self) -> bool {
        self.flags & NcFlag::NoWinchSigHandler != NcFlag::None
    }

    /// Returns `true` if it has the [`PreserveCursor`] flag set.
    ///
    /// [`PreserveCursor`]: NcFlag#associatedconstant.PreserveCursor
    pub fn is_preserve_cursor(&self) -> bool {
        self.flags & NcFlag::PreserveCursor != NcFlag::None
    }

    /// Returns `true` if it has the [`Scrolling`] flag set.
    ///
    /// [`Scrolling`]: NcFlag#associatedconstant.Scrolling
    pub fn is_scrolling(&self) -> bool {
        self.flags & NcFlag::Scrolling != NcFlag::None
    }

    /// Returns `true` if it has the [`CliMode`] flag set.
    ///
    /// [`CliMode`]: NcFlag#associatedconstant.CliMode
    pub fn is_cli_mode(&self) -> bool {
        self.flags & NcFlag::CliMode != NcFlag::None
    }

    /// Returns `true` if it has the [`SuppressBanners`] flag set.
    ///
    /// [`SuppressBanners`]: NcFlag#associatedconstant.SuppressBanners
    pub fn is_suppress_banners(&self) -> bool {
        self.flags & NcFlag::SuppressBanners != NcFlag::None
    }
}
