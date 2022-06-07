//! `NcOptions`

use crate::{c_api::ffi, NcLogLevel};
use std::ptr::null;

mod builder;
pub(crate) mod flags;

pub use builder::NcOptionsBuilder;
pub use flags::NcFlag;

/// Options struct for [`Nc`][crate::Nc].
///
/// # Fields
/// - [`termtype`]: The name of the terminfo database entry describing this terminal.
/// - [`loglevel`]: stderr output [`NcLogLevel`][crate::NcLogLevel].
/// - [`margin_t`]: Desirable top margin.
/// - [`margin_r`]: Desirable right margin.
/// - [`margin_b`]: Desirable bottom margin.
/// - [`margin_l`]: Desirable left margin.
/// - [`flags`]: [`NcFlag`] flags.
///
/// [`termtype`]: ffi::notcurses_options#structfield.termtype
/// [`loglevel`]: ffi::notcurses_options#structfield.loglevel
/// [`margin_t`]: ffi::notcurses_options#structfield.margin_t
/// [`margin_r`]: ffi::notcurses_options#structfield.margin_r
/// [`margin_b`]: ffi::notcurses_options#structfield.margin_b
/// [`margin_l`]: ffi::notcurses_options#structfield.margin_l
/// [`flags`]: NcOptions#associatedconstant.flags
pub type NcOptions = ffi::notcurses_options;

/// # constructors
impl NcOptions {
    /// Returns a default `Nc` options builder.
    pub fn builder() -> NcOptionsBuilder {
        NcOptionsBuilder::default()
    }

    /// Returns a builder object from the current `Nc` options.
    pub fn to_builder(&self) -> NcOptionsBuilder {
        NcOptionsBuilder::from_options(self)
    }

    //

    /// New `NcOptions`.
    pub fn new() -> Self {
        Self::with_all_options(NcLogLevel::Silent, Some((0, 0, 0, 0)), NcFlag::None)
    }

    /// New `NcOptions`, with margins.
    pub fn with_margins(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self::with_all_options(
            NcLogLevel::Silent,
            Some((top, right, bottom, left)),
            NcFlag::None,
        )
    }

    /// New `NcOptions`, with flags.
    pub fn with_flags(flags: NcFlag) -> Self {
        Self::with_all_options(NcLogLevel::Silent, Some((0, 0, 0, 0)), flags)
    }

    /// New `NcOptions`, with flags
    pub fn with_all_options(
        loglevel: NcLogLevel,
        trbl_margins: Option<(u32, u32, u32, u32)>,
        flags: NcFlag,
    ) -> NcOptions {
        let (margin_t, margin_r, margin_b, margin_l) = trbl_margins.unwrap_or((0, 0, 0, 0));
        NcOptions {
            termtype: null(),
            loglevel: loglevel.into(),
            margin_t,
            margin_r,
            margin_b,
            margin_l,
            flags: flags.into(),
        }
    }
}

/// # methods
impl NcOptions {
    /// Returns the `(top, right, bottom, left)` margins.
    pub fn margins(&self) -> (u32, u32, u32, u32) {
        (self.margin_t, self.margin_r, self.margin_b, self.margin_l)
    }

    /// Returns the log level.
    pub fn log_level(&self) -> NcLogLevel {
        self.loglevel.into()
    }

    // flags

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
    /// [`NoFontChanges`]: NcFlag#associatedconstant.NoFontChanges
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
    // CHECK boolean logic
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
