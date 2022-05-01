//! `Nc*` methods and associated functions.

use core::ptr::{null, null_mut};

use crate::{
    c_api::{self, notcurses_init},
    cstring, error, error_ref_mut, rstring, rstring_free, Nc, NcAlign, NcBlitter, NcCapabilities,
    NcChannels, NcError, NcFd, NcFile, NcFlag, NcInput, NcLogLevel, NcMiceEvents, NcOptions,
    NcPixelImpl, NcPlane, NcReceived, NcResult, NcRgb, NcScale, NcStats, NcStyle, NcTime, NcVisual,
    NcVisualGeometry, NcVisualOptions,
};

/// # `NcOptions` Constructors
impl NcOptions {
    /// New `NcOptions`.
    pub fn new() -> Self {
        Self::with_all_options(NcLogLevel::Silent, 0, 0, 0, 0, NcFlag::None)
    }

    /// New `NcOptions`, with margins.
    pub fn with_margins(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self::with_all_options(NcLogLevel::Silent, top, right, bottom, left, NcFlag::None)
    }

    /// New `NcOptions`, with flags.
    pub fn with_flags(flags: impl Into<NcFlag>) -> Self {
        Self::with_all_options(NcLogLevel::Silent, 0, 0, 0, 0, flags.into())
    }

    /// New `NcOptions`, with all the options.
    ///
    /// ## Arguments
    ///
    /// - loglevel
    ///
    ///   Progressively higher log levels result in more logging to stderr. By
    ///   default, nothing is printed to stderr once fullscreen service begins.
    ///
    /// - margin_t, margin_r, margin_b, margin_l
    ///
    ///   Desirable margins (top, right, bottom, left).
    ///
    ///   If all are 0 (default), we will render to the entirety of the screen.
    ///   If the screen is too small, we do what we can.
    ///   Absolute coordinates are relative to the rendering area
    ///   ((0, 0) is always the origin of the rendering area).
    ///
    /// - flags
    ///
    ///   General flags; This is expressed as a bitfield so that future options
    ///   can be added without reshaping the struct.
    ///   Undefined bits must be set to 0.
    pub fn with_all_options(
        loglevel: impl Into<NcLogLevel>,
        margin_t: u32,
        margin_r: u32,
        margin_b: u32,
        margin_l: u32,
        flags: impl Into<NcFlag>,
    ) -> Self {
        Self {
            termtype: null(),
            loglevel: loglevel.into().into(),
            margin_t,
            margin_r,
            margin_b,
            margin_l,
            flags: flags.into().into(),
        }
    }
}

/// # `Nc` Constructors and destructors
impl Nc {
    /// New notcurses context.
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn new<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(NcFlag::None)
    }

    /// New notcurses context in CLI mode.
    ///
    /// Has the [`CliMode`] flag enabled.
    ///
    /// [`CliMode`]: NcFlag#associatedconstant.CliMode
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn new_cli<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(NcFlag::CliMode)
    }

    /// New notcurses context, without banners.
    ///
    /// It has the [`SuppressBanners`] flag enabled.
    ///
    /// [`SuppressBanners`]: NcFlag#associatedconstant.SuppressBanners
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn new_silent<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(NcFlag::SuppressBanners)
    }

    /// New notcurses context in CLI mode, without banners.
    ///
    /// It has the [`CliMode`] and [`SuppressBanners`] flags enabled.
    ///
    /// [`CliMode`]: NcFlag#associatedconstant.CliMode
    /// [`SuppressBanners`]: NcFlag#associatedconstant.SuppressBanners
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn new_cli_silent<'a>() -> NcResult<&'a mut Nc> {
        Self::with_flags(NcFlag::CliMode | NcFlag::SuppressBanners)
    }

    /// New notcurses context, expects [`NcFlag`].
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn with_flags<'a>(flags: impl Into<NcFlag>) -> NcResult<&'a mut Nc> {
        Self::with_options(NcOptions::with_flags(flags.into()))
    }

    /// New notcurses context, expects [NcOptions].
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn with_options<'a>(options: NcOptions) -> NcResult<&'a mut Nc> {
        let res = notcurses_init(&options, null_mut());
        error_ref_mut![res, &format!["Nc.with_options({:?})", options]]
    }

    /// New notcurses context, expects [NcLogLevel] and flags.
    ///
    /// # Safety
    /// You can't have multiple simultaneous `Nc` instances in the same thread.
    pub unsafe fn with_debug<'a>(
        loglevel: impl Into<NcLogLevel>,
        flags: impl Into<NcFlag>,
    ) -> NcResult<&'a mut Nc> {
        Self::with_options(NcOptions::with_all_options(
            loglevel.into(),
            0,
            0,
            0,
            0,
            flags.into(),
        ))
    }

    /// Destroys the notcurses context.
    ///
    /// # Safety
    /// You must not call this method repeatedly on the same `Nc` instance.
    ///
    /// *C style function: [notcurses_stop()][c_api::notcurses_stop].*
    pub unsafe fn stop(&mut self) -> NcResult<()> {
        error![c_api::notcurses_stop(self)]
    }
}

/// # `Nc` methods
impl Nc {
    /// Returns the offset into `availcols` at which `cols` ought be output given
    /// the requirements of `align`.
    ///
    /// Returns `-`[NCRESULT_MAX][c_api::NCRESULT_MAX] if
    /// [`NcAlign::Unaligned`].
    ///
    /// *C style function: [notcurses_align()][c_api::notcurses_align].*
    pub fn align(availcols: u32, align: impl Into<NcAlign> + Copy, cols: u32) -> NcResult<u32> {
        let res = c_api::notcurses_align(availcols, align.into(), cols);
        error![
            res,
            &format!("Nc.align({:?}, {})", align.into(), cols),
            res as u32
        ]
    }

    /// Retrieves the current contents of the specified [`NcCell`][crate::NcCell]
    /// as last rendered, returning the `EGC` (or None on error) and writing
    /// out the [`NcStyle`] and the [`NcChannels`].
    ///
    /// *C style function: [notcurses_at_yx()][c_api::notcurses_at_yx].*
    pub fn at_yx(
        &mut self,
        y: u32,
        x: u32,
        stylemask: &mut NcStyle,
        channels: &mut NcChannels,
    ) -> Option<String> {
        let egc = unsafe { c_api::notcurses_at_yx(self, y, x, stylemask.into(), &mut channels.0) };
        if egc.is_null() {
            return None;
        }
        Some(rstring_free![egc])
    }

    /// Returns the detected capabilities of the current terminal.
    ///
    /// *C style function: [notcurses_capabilities()][c_api::notcurses_capabilities].*
    pub fn capabilities(&self) -> NcCapabilities {
        unsafe { *c_api::notcurses_capabilities(self) }
    }

    /// Returns true if we can reliably use Unicode Braille.
    ///
    /// See also [`NcBlitter::Braille`].
    ///
    /// *C style function: [notcurses_canbraille()][c_api::notcurses_canbraille].*
    pub fn canbraille(&self) -> bool {
        c_api::notcurses_canbraille(self)
    }

    /// Returns true if it's possible to set the "hardware" palette.
    ///
    /// Requires the "ccc" terminfo capability.
    ///
    /// *C style function: [notcurses_canchangecolor()][c_api::notcurses_canchangecolor].*
    pub fn canchangecolor(&self) -> bool {
        c_api::notcurses_canchangecolor(self)
    }

    /// Returns true if fading is possible.
    ///
    /// Fading requires either the "rgb" or "ccc" terminfo capability.
    ///
    /// *C style function: [notcurses_canfade()][c_api::notcurses_canfade].*
    pub fn canfade(&self) -> bool {
        c_api::notcurses_canfade(self)
    }

    /// Returns true if we can reliably use Unicode half blocks.
    ///
    /// See also [`NcBlitter::Half`].
    ///
    /// *C style function: [notcurses_canhalfblock()][c_api::notcurses_canhalfblock].*
    pub fn canhalfblock(&self) -> bool {
        c_api::notcurses_canhalfblock(self)
    }

    /// Returns true if loading images is possible.
    ///
    /// This requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [notcurses_canopen_images()][c_api::notcurses_canopen_images].*
    pub fn canopen_images(&self) -> bool {
        unsafe { c_api::notcurses_canopen_images(self) }
    }

    /// Returns true if loading videos is possible.
    ///
    /// This requires being built against FFmpeg.
    ///
    /// *C style function: [notcurses_canopen_videos()][c_api::notcurses_canopen_videos].*
    pub fn canopen_videos(&self) -> bool {
        unsafe { c_api::notcurses_canopen_videos(self) }
    }

    /// Returns true if we can blit pixel-accurate bitmaps.
    ///
    /// See also [`check_pixel_support`][Nc#method.check_pixel_support].
    ///
    /// *C style function: [notcurses_canpixel()][c_api::notcurses_canpixel].*
    pub fn canpixel(&self) -> bool {
        c_api::notcurses_canpixel(self)
    }

    /// Returns true if we can reliably use Unicode quadrant blocks.
    ///
    /// See also [`NcBlitter::Quadrant`].
    ///
    /// *C style function: [notcurses_canquadrant()][c_api::notcurses_canquadrant].*
    pub fn canquadrant(&self) -> bool {
        c_api::notcurses_canquadrant(self)
    }

    /// Returns true if we can reliably use Unicode 13 sextants.
    ///
    /// See also [`NcBlitter::Sextant`].
    ///
    /// *C style function: [notcurses_cansextant()][c_api::notcurses_cansextant].*
    pub fn cansextant(&self) -> bool {
        c_api::notcurses_cansextant(self)
    }

    /// Returns true if it's possible to directly specify RGB values per cell,
    /// or false if it's only possible to use palettes.
    ///
    /// *C style function: [notcurses_cantruecolor()][c_api::notcurses_cantruecolor].*
    pub fn cantruecolor(&self) -> bool {
        c_api::notcurses_cantruecolor(self)
    }

    /// Returns true if the encoding is UTF-8.
    ///
    /// Requires `LANG` being set to a UTF-8 locale.
    ///
    /// *C style function: [notcurses_canutf8()][c_api::notcurses_canutf8].*
    pub fn canutf8(&self) -> bool {
        c_api::notcurses_canutf8(self)
    }

    /// Checks for pixel support.
    ///
    /// Returns [`NcPixelImpl`] with a non-zero constant corresponding to some
    /// pixel-blitting mechanism if bitmap support (via any mechanism) has been
    /// detected, or else 0 (NCPIXEL_NONE).
    ///
    /// *C style function: [notcurses_check_pixel_support()][c_api::notcurses_check_pixel_support].*
    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn check_pixel_support(&self) -> NcPixelImpl {
        unsafe { c_api::notcurses_check_pixel_support(self) }.into()
    }

    /// Returns the default foreground color, if it is known.
    pub fn default_foreground(&self) -> Option<NcRgb> {
        let mut fg = 0;
        let res = unsafe { c_api::notcurses_default_foreground(self, &mut fg) };
        if res == c_api::NCRESULT_ERR {
            None
        } else {
            Some(fg.into())
        }
    }

    /// Returns the default background color, if it is known.
    pub fn default_background(&self) -> Option<NcRgb> {
        let mut bg = 0;
        let res = unsafe { c_api::notcurses_default_background(self, &mut bg) };
        if res == c_api::NCRESULT_ERR {
            None
        } else {
            Some(bg.into())
        }
    }

    /// Disables the terminal's cursor, if supported.
    ///
    /// Immediate effect (no need for a call to notcurses_render()).
    ///
    /// *C style function: [notcurses_cursor_disable()][c_api::notcurses_cursor_disable].*
    pub fn cursor_disable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_cursor_disable(self) }]
    }

    /// Enables the terminal's cursor, if supported, placing it at `y`, `x`.
    ///
    /// Immediate effect (no need for a call to notcurses_render()).
    /// It is an error if `y`, `x` lies outside the standard plane.
    ///
    /// *C style function: [notcurses_cursor_enable()][c_api::notcurses_cursor_enable].*
    pub fn cursor_enable(&mut self, y: u32, x: u32) -> NcResult<()> {
        error![unsafe { c_api::notcurses_cursor_enable(self, y as i32, x as i32) }]
    }

    /// Shifts to the alternate screen, if available.
    ///
    /// If already using the alternate screen, this returns Ok(()) immediately.
    ///
    /// If the alternate screen is not available, returns an Error immediately.
    ///
    /// Entering the alternate screen turns off scrolling for the standard plane.
    ///
    /// *C style function:
    /// [notcurses_enter_alternate_screen()][c_api::notcurses_enter_alternate_screen].*
    pub fn enter_alternate_screen(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_enter_alternate_screen(self) }]
    }

    /// Exits the alternate screen.
    ///
    /// Immediately returns Ok(()) if not currently using the alternate screen.
    ///
    /// *C style function:
    /// [notcurses_leave_alternate_screen()][c_api::notcurses_leave_alternate_screen].*
    pub fn leave_alternate_screen(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_leave_alternate_screen(self) }]
    }

    /// Dumps notcurses state to the supplied `debugfp`.
    ///
    /// Output is freeform, and subject to change. It includes geometry of all
    /// planes, from all piles.
    ///
    /// *C style function: [notcurses_debug()][c_api::notcurses_debug].*
    pub fn debug(&mut self, debugfp: &mut NcFile) {
        unsafe {
            c_api::notcurses_debug(self, debugfp.as_nc_ptr());
        }
    }

    /// Returns the name of the user under which we are running.
    ///
    /// *C style function: [notcurses_accountname()][c_api::notcurses_accountname].*
    pub fn accountname() -> String {
        rstring_free![c_api::notcurses_accountname()]
    }

    /// Returns the name of the local hostname.
    ///
    /// *C style function: [notcurses_hostname()][c_api::notcurses_hostname].*
    pub fn hostname() -> String {
        rstring_free![c_api::notcurses_hostname()]
    }

    /// Returns the name of the detected terminal.
    ///
    /// *C style function: [notcurses_detected_terminal()][c_api::notcurses_detected_terminal].*
    pub fn detected_terminal(&self) -> String {
        rstring_free![c_api::notcurses_detected_terminal(self)]
    }

    /// Returns the name of the detected OS version.
    ///
    /// *C style function: [notcurses_osversion()][c_api::notcurses_osversion].*
    pub fn osversion(&self) -> String {
        rstring_free![c_api::notcurses_detected_terminal(self)]
    }

    /// Destroys all [`NcPlane`]s other than the stdplane.
    ///
    /// *C style function: [notcurses_drop_planes()][c_api::notcurses_drop_planes].*
    pub fn drop_planes(&mut self) {
        unsafe {
            c_api::notcurses_drop_planes(self);
        }
    }

    /// Reads input.
    ///
    /// Provide `None` in `time` to block at length, and otherwise
    /// `Some(`[`NcTime`]`)` to bound blocking.
    ///
    /// `time` is an a delay bound against `CLOCK_MONOTONIC`
    /// (see [*pthread_cond_clockwait(3)*](https://linux.die.net/man/3/pthread_cond_wait)).
    ///
    /// *C style function: [notcurses_get()][c_api::notcurses_get].*
    pub fn get(
        &mut self,
        time: Option<NcTime>,
        input: Option<&mut NcInput>,
    ) -> NcResult<NcReceived> {
        let ntime = if let Some(time) = time { &time as *const _ } else { null() };
        let ninput = if let Some(input) = input { input as *mut _ } else { null_mut() };

        let res = unsafe { c_api::notcurses_get(self, ntime, ninput) };
        if res == c_api::NCRESULT_ERR as u32 {
            Err(NcError::new_msg(&format!["Nc.get({:?})", time]))
        } else {
            Ok(NcReceived::from(res))
        }
    }

    /// Reads input blocking until an event is processed or a signal is received.
    ///
    /// Will optionally write the event details in `input`.
    ///
    /// *C style function: [notcurses_get_blocking()][c_api::notcurses_get_blocking].*
    pub fn get_blocking(&mut self, input: Option<&mut NcInput>) -> NcResult<NcReceived> {
        let res = c_api::notcurses_get_blocking(self, input);
        if res == c_api::NCRESULT_ERR {
            Err(NcError::new_msg("Nc.get_blocking()"))
        } else {
            Ok(NcReceived::from(res as u32))
        }
    }

    /// Reads input without blocking.
    ///
    /// *C style function: [notcurses_get_nblock()][c_api::notcurses_get_nblock].*
    pub fn get_nblock(&mut self, input: Option<&mut NcInput>) -> NcResult<NcReceived> {
        let res = c_api::notcurses_get_nblock(self, input);
        if res == c_api::NCRESULT_ERR {
            Err(NcError::new_msg("Nc.get_nblock()"))
        } else {
            Ok(NcReceived::from(res as u32))
        }
    }

    /// Acquire up to 'vcount' [`NcInput`]s at the vector 'ni'.
    ///
    /// The number read will be returned, or 0 on timeout.
    ///
    /// *C style function: [notcurses_getvec()][c_api::notcurses_getvec].*
    pub fn getvec(
        &mut self,
        time: Option<NcTime>,
        ni: &mut Vec<NcInput>,
        vcount: u32,
    ) -> NcResult<u32> {
        let ntime = if let Some(time) = time { &time as *const _ } else { null() };
        let nivec = ni.as_mut_ptr() as *mut NcInput;

        let res = unsafe { c_api::notcurses_getvec(self, ntime, nivec, vcount as i32) };
        error![res, "", res as u32]
    }

    /// Gets a file descriptor suitable for input event poll()ing.
    ///
    /// When this descriptor becomes available, you can call
    /// [get_nblock()][Nc#method.get_nblock], and input ought be ready.
    ///
    /// This file descriptor is not necessarily the file descriptor associated
    /// with stdin (but it might be!).
    ///
    /// *C style function: [notcurses_inputready_fd()][c_api::notcurses_inputready_fd].*
    pub fn inputready_fd(&mut self) -> NcResult<NcFd> {
        let res = unsafe { c_api::notcurses_inputready_fd(self) };
        error![res, "", res]
    }

    /// Returns an [`NcBlitter`] from a string representation.
    ///
    /// *C style function: [notcurses_lex_blitter()][c_api::notcurses_lex_blitter].*
    pub fn lex_blitter(blitter_str: &str) -> NcResult<NcBlitter> {
        let mut blitter = 0;
        let cs = cstring![blitter_str];
        error![
            unsafe { c_api::notcurses_lex_blitter(cs.as_ptr(), &mut blitter) },
            "Invalid blitter name",
            blitter.into()
        ]
    }

    /// Lexes a margin argument according to the standard notcurses definition.
    ///
    /// There can be either a single number, which will define all margins equally,
    /// or there can be four numbers separated by commas.
    ///
    /// *C style function: [notcurses_lex_margins()][c_api::notcurses_lex_margins].*
    pub fn lex_margins(margins_str: &str, options: &mut NcOptions) -> NcResult<()> {
        let cs = cstring![margins_str];
        error![unsafe { c_api::notcurses_lex_margins(cs.as_ptr(), options) }]
    }

    /// Returns an [`NcScale`] from a string representation.
    ///
    /// *C style function: [notcurses_lex_scalemode()][c_api::notcurses_lex_scalemode].*
    pub fn lex_scalemode(scale_str: &str) -> NcResult<NcScale> {
        let mut scale = 0;
        let cs = cstring![scale_str];
        error![
            unsafe { c_api::notcurses_lex_scalemode(cs.as_ptr(), &mut scale) },
            "",
            scale.into()
        ]
    }

    /// Returns an [`NcStyle`] from a string representation.
    ///
    /// It is case-insensitive, and supports multiple styles separated by
    /// spaces.
    ///
    /// The supported styles are: `italic`, `underline`, `undercurl`,
    /// `struck`, `bold`, and `none`.
    ///
    /// If a style is are not recognized returns an error.
    ///
    /// *(No equivalent C style function)*
    pub fn lex_styles(styles_str: &str) -> NcResult<NcStyle> {
        let mut style = NcStyle::None;
        let mut errstr = String::new();

        for s in styles_str.split(' ') {
            match s.to_lowercase().as_str() {
                "italic" => style.add(NcStyle::Italic),
                "underline" => style.add(NcStyle::Underline),
                "undercurl" => style.add(NcStyle::Undercurl),
                "struck" => style.add(NcStyle::Struck),
                "bold" => style.add(NcStyle::Bold),
                "none" => (),
                _ => {
                    errstr.push_str(s);
                    errstr.push(' ');
                }
            }
        }
        if errstr.is_empty() {
            Ok(style)
        } else {
            let _ = errstr.pop();
            Err(NcError::new_msg(&format![
                "the following styles are not recognized: '{}'",
                errstr
            ]))
        }
    }

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    ///
    /// *C style function: [notcurses_linesigs_disable()][c_api::notcurses_linesigs_disable].*
    pub fn linesigs_disable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_linesigs_disable(self) }]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    ///
    /// *C style function: [notcurses_linesigs_enable()][c_api::notcurses_linesigs_enable].*
    pub fn linesigs_enable(&mut self) -> NcResult<()> {
        error![unsafe { c_api::notcurses_linesigs_enable(self) }]
    }

    /// Disables mice events.
    ///
    /// *C style function: [notcurses_mice_disable()][c_api::notcurses_mice_disable].*
    pub fn mice_disable(&mut self) -> NcResult<()> {
        self.mice_enable(NcMiceEvents::None)
    }

    /// Enables mice events according to `eventmask`.
    ///
    /// An eventmask of 0 will disable all mice tracking.
    ///
    /// On success mouse events will be published to `notcurses_get`.
    ///
    /// *C style function: [notcurses_mice_enable()][c_api::notcurses_mice_enable].*
    pub fn mice_enable(&mut self, eventmask: NcMiceEvents) -> NcResult<()> {
        error![
            unsafe { c_api::notcurses_mice_enable(self, eventmask.into()) },
            "Nc.mice_enable()"
        ]
    }

    /// Returns the number of simultaneous colors claimed to be supported,
    /// if there is color support.
    ///
    /// Note that several terminal emulators advertise more colors than they
    /// actually support, downsampling internally.
    ///
    /// *C style function: [notcurses_palette_size()][c_api::notcurses_palette_size].*
    pub fn palette_size(&self) -> NcResult<u32> {
        let res = unsafe { c_api::notcurses_palette_size(self) };
        if res == 1 {
            return Err(NcError::with_msg(1, "No color support ← Nc.palette_size()"));
        }
        Ok(res)
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [`render`][crate::Nc#method.render]).
    ///
    /// Returns the current screen geometry (`y`, `x`).
    ///
    /// This is primarily useful if the screen is externally corrupted, or if an
    /// [NcKey::Resize][crate::NcKey#associatedconstant.Resize] event
    /// has been read and you're not yet ready to render.
    ///
    /// *C style function: [notcurses_refresh()][c_api::notcurses_refresh].*
    pub fn refresh(&mut self) -> NcResult<(u32, u32)> {
        let (mut y, mut x) = (0, 0);
        error![
            unsafe { c_api::notcurses_refresh(self, &mut y, &mut x) },
            "",
            (y as u32, x as u32)
        ]
    }

    /// Renders and rasterizes the standard pile in one shot. Blocking call.
    ///
    /// *C style function: [notcurses_render()][c_api::notcurses_render].*
    pub fn render(&mut self) -> NcResult<()> {
        error![c_api::notcurses_render(self), "Nc.render()"]
    }

    /// Acquires an atomic snapshot of the notcurses object's stats.
    ///
    /// *C style function: [notcurses_stats()][c_api::notcurses_stats].*
    pub fn stats(&mut self, stats: &mut NcStats) {
        unsafe {
            c_api::notcurses_stats(self, stats);
        }
    }

    /// Allocates an [`NcStats`] object.
    ///
    /// Use this rather than allocating your own, since future versions of
    /// notcurses might enlarge this structure.
    ///
    /// *C style function: [notcurses_stats_alloc()][c_api::notcurses_stats_alloc].*
    pub fn stats_alloc(&mut self) -> &mut NcStats {
        unsafe { &mut *c_api::notcurses_stats_alloc(self) }
    }

    /// Resets all cumulative stats (immediate ones, such as fbbytes, are not reset).
    ///
    /// *C style function: [notcurses_stats_reset()][c_api::notcurses_stats_reset].*
    pub fn stats_reset(&mut self, stats: &mut NcStats) {
        unsafe {
            c_api::notcurses_stats_reset(self, stats);
        }
    }

    // TODO: decide what to do with these two:
    //
    // /// [notcurses_stdplane()][c_api::notcurses_stdplane], plus free bonus
    // /// dimensions written to non-NULL y/x!
    // ///
    // /// *C style function: [notcurses_stddim_yx()][c_api::notcurses_stddim_yx].*
    // #[inline]
    // pub fn stddim_yx<'a>(
    //     &'a mut self,
    //     y: &mut u32,
    //     x: &mut u32,
    // ) -> NcResult<&'a mut NcPlane> {
    //     c_api::notcurses_stddim_yx(self, y, x)
    // }

    // /// [stdplane_const()][Nc#method.stdplane_const], plus free
    // /// bonus dimensions written to non-NULL y/x!
    // ///
    // /// *C style function: [notcurses_stddim_yx()][c_api::notcurses_stddim_yx].*
    // #[inline]
    // pub fn stddim_yx_const<'a>(
    //     &'a self,
    //     y: &mut u32,
    //     x: &mut u32,
    // ) -> NcResult<&'a NcPlane> {
    //     c_api::notcurses_stddim_yx_const(self, y, x)
    // }

    /// Returns a mutable reference to the standard [`NcPlane`] for this terminal.
    ///
    /// The standard plane always exists, and its origin is always at the
    /// uppermost, leftmost cell.
    ///
    /// # Safety
    /// You must be careful not to end up with multiple exclusive references
    /// to the standard plane, or with one exclusive reference and one or more
    /// shared references.
    ///
    /// *C style function: [notcurses_stdplane()][c_api::notcurses_stdplane].*
    pub unsafe fn stdplane<'a>(&mut self) -> &'a mut NcPlane {
        &mut *c_api::notcurses_stdplane(self)
    }

    /// Returns a reference to the standard [`NcPlane`] for this terminal.
    ///
    /// The standard plane always exists, and its origin is always at the
    /// uppermost, leftmost cell.
    ///
    /// # Safety
    /// You must be careful not to end up with a mix of exclusive references
    /// and shared references to the standard plane.
    ///
    /// *C style function: [notcurses_stdplane_const()][c_api::notcurses_stdplane_const].*
    pub unsafe fn stdplane_const<'a>(&self) -> &'a NcPlane {
        &*c_api::notcurses_stdplane_const(self)
    }

    /// Gets the name of an [`NcBlitter`] blitter.
    ///
    /// *C style function: [notcurses_str_blitter()][c_api::notcurses_str_blitter].*
    pub fn str_blitter(blitter: impl Into<NcBlitter>) -> String {
        rstring![c_api::notcurses_str_blitter(blitter.into().into())].to_string()
    }

    /// Gets the name of an [`NcScale`] scaling mode.
    ///
    /// *C style function: [notcurses_str_scalemode()][c_api::notcurses_str_scalemode].*
    pub fn str_scalemode(scale: impl Into<NcScale>) -> String {
        rstring![c_api::notcurses_str_scalemode(scale.into().into())].to_string()
    }

    /// Gets the lowercase name (or names) of the styles included in an [`NcStyle`].
    ///
    /// *(No equivalent C style function)*
    pub fn str_styles(style: impl Into<NcStyle>) -> String {
        let mut string = String::new();
        for s in style.into().to_vec() {
            string.push_str(match s {
                NcStyle::Italic => "italic",
                NcStyle::Underline => "underline",
                NcStyle::Undercurl => "undercurl",
                NcStyle::Struck => "struck",
                NcStyle::Bold => "bold",
                #[allow(unreachable_patterns)] // FIXME
                NcStyle::None => "none",
                _ => "none",
            });
            string.push(' ');
        }
        let _ = string.pop();
        string
    }

    /// Returns an [`NcStyle`] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    ///
    /// For more information, see the "ncv" capability in terminfo(5).
    ///
    /// *C style function: [notcurses_supported_styles()][c_api::notcurses_supported_styles].*
    pub fn supported_styles(&self) -> NcStyle {
        unsafe { c_api::notcurses_supported_styles(self).into() }
    }

    /// Returns our current idea of the terminal dimensions in rows and cols.
    ///
    /// *C style function: [notcurses_term_dim_yx()][c_api::notcurses_term_dim_yx].*
    pub fn term_dim_yx(&self) -> (u32, u32) {
        c_api::notcurses_term_dim_yx(self)
    }

    /// Returns the bottommost [`NcPlane`] on the standard pile,
    /// of which there is always at least one.
    ///
    /// *C style function: [notcurses_bottom()][c_api::notcurses_bottom].*
    pub fn bottom(&mut self) -> &mut NcPlane {
        c_api::notcurses_bottom(self)
    }

    /// Returns the topmost [`NcPlane`], of which there is always at least one.
    ///
    /// *C style function: [notcurses_top()][c_api::notcurses_top].*
    pub fn top(&mut self) -> &mut NcPlane {
        c_api::notcurses_top(self)
    }

    /// Returns a human-readable string describing the running notcurses version.
    ///
    /// *C style function: [notcurses_version()][c_api::notcurses_version].*
    pub fn version() -> String {
        rstring![c_api::notcurses_version()].to_string()
    }

    /// Returns the running notcurses version components
    /// (major, minor, patch, tweak).
    ///
    /// *C style function: [notcurses_version_components()][c_api::notcurses_version_components].*
    pub fn version_components() -> (u32, u32, u32, u32) {
        let (mut major, mut minor, mut patch, mut tweak) = (0, 0, 0, 0);
        unsafe {
            c_api::notcurses_version_components(&mut major, &mut minor, &mut patch, &mut tweak);
        }
        (major as u32, minor as u32, patch as u32, tweak as u32)
    }

    /// Returns [`NcVisualGeometry`].
    ///
    /// If an [`NcVisual`] is not provided, only the [`cdim_yx`], [`blitter`],
    /// [`scale_yx`], and [`maxpixel_yx`] fields will be filled in.
    ///
    /// If an [`NcVisualOptions`] is not provided, a default one will be used.
    ///
    /// Additionally `cdim_yx` and `maxpixel_yx` are only ever filled in if we
    /// know them, and `maxpixel_yx` is only defined for [`NcBlitter::Pixel`].
    ///
    /// # See also
    /// - [`NcVisual.geom`][NcVisual#method.geom]
    ///
    /// [`cdim_yx`]: NcVisualGeometry#structfield.cdim_yx
    /// [`blitter`]: NcVisualGeometry#structfield.blitter
    /// [`scale_yx`]: NcVisualGeometry#structfield.scale_yx
    /// [`maxpixel_yx`]: NcVisualGeometry#structfield.maxpixel_yx
    ///
    /// *C style function: [ncvisual_geom()][c_api::ncvisual_geom].*
    pub fn visual_geom(
        &self,
        visual: Option<&NcVisual>,
        vopts: Option<&NcVisualOptions>,
    ) -> NcResult<NcVisualGeometry> {
        let mut vg = c_api::NcVGeom::new();

        let v_ptr: *const NcVisual = if let Some(v) = visual { v } else { null() };
        let vo_ptr: *const NcVisualOptions =
            if let Some(o) = vopts { o } else { &NcVisualOptions::default() };

        let res = unsafe { crate::c_api::ncvisual_geom(self, v_ptr, vo_ptr, &mut vg) };
        if res <= c_api::NCRESULT_ERR {
            return Err(NcError::with_msg(
                res,
                &format!["Nc.visual_geom({:?}, {:?})", visual, vopts],
            ));
        }

        let (pix_yx, cdim_yx, rpix_yx, rcell_yx, scale_yx, maxpixel_yx, beg_yx, len_yx);

        // if an `NcVisual` is not provided, only `maxpixel_yx`, `cdim_yx` and
        // `scale_yx` can be non-zero.
        if visual.is_none() {
            cdim_yx = Some((vg.cdimy as u32, vg.cdimx as u32));
            scale_yx = Some((vg.scaley as u32, vg.scalex as u32));

            // pixel blitter only is defined for Ncblitter::PIXEL
            if vg.blitter == NcBlitter::Pixel.into() {
                maxpixel_yx = Some((vg.maxpixely as u32, vg.maxpixelx as u32));
            } else {
                maxpixel_yx = None;
            }

            pix_yx = None;
            rpix_yx = None;
            rcell_yx = None;
            beg_yx = None;
            len_yx = None;
        } else {
            // `maxpixel_yx` only is defined for `Ncblitter`::PIXEL.
            if vg.blitter == NcBlitter::Pixel.into() {
                maxpixel_yx = Some((vg.maxpixely as u32, vg.maxpixelx as u32));
            } else {
                maxpixel_yx = None;
            }

            // `beg_yx` & `len_yx` can be safely ignored if they're all 0.
            if vg.begy | vg.begx | vg.leny | vg.lenx == 0 {
                beg_yx = None;
                len_yx = None;
            } else {
                beg_yx = Some((vg.begy as u32, vg.begx as u32));
                len_yx = Some((vg.leny as u32, vg.lenx as u32));
            }

            // valid values for the following fields can't be 0 either:
            if vg.pixy | vg.pixx == 0 {
                pix_yx = None;
            } else {
                pix_yx = Some((vg.pixy as u32, vg.pixx as u32));
            }
            if vg.cdimy | vg.cdimx == 0 {
                cdim_yx = None;
            } else {
                cdim_yx = Some((vg.cdimy as u32, vg.cdimx as u32));
            }
            if vg.scaley | vg.scalex == 0 {
                scale_yx = None;
            } else {
                scale_yx = Some((vg.scaley as u32, vg.scalex as u32));
            }
            if vg.rpixy | vg.rpixx == 0 {
                rpix_yx = None;
            } else {
                rpix_yx = Some((vg.rpixy as u32, vg.rpixx as u32));
            }
            if vg.rcelly | vg.rcellx == 0 {
                rcell_yx = None;
            } else {
                rcell_yx = Some((vg.rcelly as u32, vg.rcellx as u32));
            }
        }

        let vgeometry = NcVisualGeometry {
            pix_yx,
            cdim_yx,
            rpix_yx,
            rcell_yx,
            scale_yx,
            maxpixel_yx,
            beg_yx,
            len_yx,
            blitter: (vg.blitter as crate::c_api::NcBlitter_u32).into(),
        };
        Ok(vgeometry)
    }

    /// Like [`visual_geom`] but auto-fills the `NcVisualOptions` with
    /// `NcBlitter::Pixel` in order to get the maximum available resolution
    /// for `scale_yx`, which determines the minimum dot-size for an `NcVisual`.
    ///
    /// [`visual_geom`]: Nc#method.visual_geom
    pub fn visual_geom_with_pixel(&self, visual: Option<&NcVisual>) -> NcResult<NcVisualGeometry> {
        Self::visual_geom(
            self,
            visual,
            Some(&NcVisualOptions::builder().pixel().build()),
        )
    }
}
