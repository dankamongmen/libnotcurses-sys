//! `NcDirect` methods and associated functions.

use core::ptr::{null, null_mut};

use crate::{
    cstring, error, error_ref_mut, fns, rstring_free, NcAlign, NcBlitter, NcCapabilities,
    NcChannels, NcComponent, NcDim, NcDirect, NcDirectFlags, NcError, NcInput, NcOffset,
    NcPaletteIndex, NcPlane, NcResult, NcRgb, NcScale, NcStyle, NcTime, NCRESULT_ERR,
};

/// # `NcDirect` constructors and destructors
impl NcDirect {
    /// New NcDirect with the default options.
    ///
    /// Initializes a direct-mode notcurses context on the tty.
    ///
    /// Direct mode supports a limited subset of notcurses routines,
    /// and neither supports nor requires
    /// [notcurses_render()][fns::notcurses_render]. This can be used to add
    /// color and styling to text in the standard output paradigm.
    ///
    /// *C style function: [ncdirect_init()][fns::ncdirect_init].*
    pub fn new<'a>() -> NcResult<&'a mut NcDirect> {
        Self::with_flags(0)
    }

    /// New NcDirect with optional flags.
    ///
    /// `flags` is a bitmask over:
    /// - [NCDIRECT_OPTION_INHIBIT_CBREAK][crate::NCDIRECT_OPTION_INHIBIT_CBREAK]
    /// - [NCDIRECT_OPTION_INHIBIT_SETLOCALE][crate::NCDIRECT_OPTION_INHIBIT_SETLOCALE]
    /// - [NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS][crate::NCDIRECT_OPTION_NO_QUIT_SIGHANDLERS]
    /// - [NCDIRECT_OPTION_VERBOSE][crate::NCDIRECT_OPTION_VERBOSE]
    /// - [NCDIRECT_OPTION_VERY_VERBOSE][crate::NCDIRECT_OPTION_VERY_VERBOSE]
    ///
    /// *C style function: [ncdirect_init()][fns::ncdirect_init].*
    pub fn with_flags<'a>(flags: NcDirectFlags) -> NcResult<&'a mut NcDirect> {
        let res = unsafe { fns::ncdirect_init(null(), null_mut(), flags) };
        error_ref_mut![res, "Initializing NcDirect"]
    }

    /// Releases this NcDirect and any associated resources.
    ///
    /// *C style function: [ncdirect_stop()][fns::ncdirect_stop].*
    pub fn stop(&mut self) -> NcResult<()> {
        error![unsafe { fns::ncdirect_stop(self) }, "NcDirect.stop()"]
    }
}

/// ## NcDirect methods: clear, flush, render
impl NcDirect {
    /// Clears the screen.
    ///
    /// *C style function: [ncdirect_clear()][fns::ncdirect_clear].*
    pub fn clear(&mut self) -> NcResult<()> {
        error![unsafe { fns::ncdirect_clear(self) }, "NcDirect.clear()"]
    }

    /// Forces a flush.
    ///
    /// *C style function: [ncdirect_flush()][fns::ncdirect_flush].*
    pub fn flush(&self) -> NcResult<()> {
        error![unsafe { fns::ncdirect_flush(self) }, "NcDirect.clear()"]
    }

    /// Takes the result of [`render_frame`][NcDirect#method.render_frame]
    /// and writes it to the output.
    ///
    /// *C style function: [ncdirect_raster_frame()][fns::ncdirect_raster_frame].*
    pub fn raster_frame(&mut self, frame: &mut NcPlane, align: NcAlign) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_raster_frame(self, frame, align) },
            "NcDirect.raster_frame()"
        ]
    }

    /// Renders an image using the specified blitter and scaling,
    /// but doesn't write the result.
    ///
    /// The image may be arbitrarily many rows -- the output will scroll --
    /// but will only occupy the column of the cursor, and those to the right.
    ///
    /// To actually write (and free) this, invoke ncdirect_raster_frame().
    ///
    /// `max_y' and 'max_x` (cell geometry, *not* pixel), if greater than 0,
    /// are used for scaling; the terminal's geometry is otherwise used.
    ///
    /// *C style function: [ncdirect_render_frame()][fns::ncdirect_render_frame].*
    pub fn render_frame<'a>(
        &mut self,
        filename: &str,
        blitter: NcBlitter,
        scale: NcScale,
        max_y: NcDim,
        max_x: NcDim,
    ) -> NcResult<&'a mut NcPlane> {
        let res = unsafe {
            fns::ncdirect_render_frame(
                self,
                cstring![filename],
                blitter,
                scale,
                max_y as i32,
                max_x as i32,
            )
        };
        error_ref_mut![
            res,
            &format!(
                "NcDirect.render_frame({:?}, {:?}, {:?})",
                filename, blitter, scale
            )
        ]
    }

    /// Displays an image using the specified blitter and scaling.
    ///
    /// The image may be arbitrarily many rows -- the output will scroll -- but
    /// will only occupy the column of the cursor, and those to the right.
    ///
    /// The render/raster process can be split by using
    /// [render_frame()][#method.render_frame] and
    /// [raster_frame()][#method.raster_frame].
    ///
    /// *C style function: [ncdirect_render_image()][fns::ncdirect_render_image].*
    pub fn render_image(
        &mut self,
        filename: &str,
        align: NcAlign,
        blitter: NcBlitter,
        scale: NcScale,
    ) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_render_image(self, cstring![filename], align, blitter, scale) },
            &format!(
                "NcDirect.render_image({:?}, {:?}, {:?}, {:?})",
                filename, align, blitter, scale
            )
        ]
    }
}

/// ## NcDirect methods: `NcPaletteIndex`, `NcRgb`, `NcStyle` & default color
impl NcDirect {
    /// Sets the foreground [NcPaletteIndex].
    ///
    /// *C style function: [ncdirect_set_fg_palindex()][fns::ncdirect_set_fg_palindex].*
    pub fn set_fg_palindex(&mut self, index: NcPaletteIndex) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_fg_palindex(self, index as i32) },
            &format!("NcDirect.set_fg_palindex({})", index)
        ]
    }

    /// Sets the background [NcPaletteIndex].
    ///
    /// *C style function: [ncdirect_set_bg_palindex()][fns::ncdirect_set_bg_palindex].*
    pub fn set_bg_palindex(&mut self, index: NcPaletteIndex) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_bg_palindex(self, index as i32) },
            &format!("NcDirect.set_fg_palindex({})", index)
        ]
    }

    /// Returns the number of simultaneous colors claimed to be supported,
    /// if there is color support.
    ///
    /// Note that several terminal emulators advertise more colors than they
    /// actually support, downsampling internally.
    ///
    /// *C style function: [ncdirect_palette_size()][fns::ncdirect_palette_size].*
    pub fn palette_size(&self) -> NcResult<u32> {
        let res = unsafe { fns::ncdirect_palette_size(self) };
        if res == 1 {
            return Err(NcError::with_msg(
                1,
                "No color support ← NcDirect.palette_size()",
            ));
        }
        Ok(res)
    }

    /// Sets the foreground [NcRgb].
    ///
    /// *C style function: [ncdirect_set_fg_rgb()][fns::ncdirect_set_fg_rgb].*
    pub fn set_fg_rgb(&mut self, rgb: NcRgb) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_fg_rgb(self, rgb) },
            &format!("NcDirect.set_fg_rgb({})", rgb)
        ]
    }

    /// Sets the background [NcRgb].
    ///
    /// *C style function: [ncdirect_set_bg_rgb()][fns::ncdirect_set_bg_rgb].*
    pub fn set_bg_rgb(&mut self, rgb: NcRgb) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_bg_rgb(self, rgb) },
            &format!("NcDirect.set_bg_rgb({})", rgb)
        ]
    }

    /// Sets the foreground [NcComponent] components.
    ///
    /// *C style function: [ncdirect_set_fg_rgb8()][fns::ncdirect_set_fg_rgb8].*
    pub fn set_fg_rgb8(
        &mut self,
        red: NcComponent,
        green: NcComponent,
        blue: NcComponent,
    ) -> NcResult<()> {
        error![
            fns::ncdirect_set_fg_rgb8(self, red, green, blue),
            &format!("NcDirect.set_fg_rgb8({}, {}, {})", red, green, blue)
        ]
    }

    /// Sets the background [NcComponent] components.
    ///
    /// *C style function: [ncdirect_set_bg_rgb()][fns::ncdirect_set_bg_rgb].*
    pub fn set_bg_rgb8(
        &mut self,
        red: NcComponent,
        green: NcComponent,
        blue: NcComponent,
    ) -> NcResult<()> {
        error![
            fns::ncdirect_set_bg_rgb8(self, red, green, blue),
            &format!("NcDirect.set_bg_rgb8({}, {}, {})", red, green, blue)
        ]
    }

    /// Returns the current styling.
    ///
    /// *C style function: [ncdirect_styles()][fns::ncdirect_styles].*
    pub fn styles(&self) -> NcStyle {
        unsafe { fns::ncdirect_styles(self) as NcStyle }
    }

    /// Removes the specified styles.
    ///
    /// *C style function: [ncdirect_off_styles()][fns::ncdirect_off_styles].*
    pub fn styles_off(&mut self, stylebits: NcStyle) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_off_styles(self, stylebits.into()) },
            &format!("NcDirect.styles_off({:0X})", stylebits)
        ]
    }

    /// Adds the specified styles.
    ///
    /// *C style function: [ncdirect_on_styles()][fns::ncdirect_on_styles].*
    pub fn styles_on(&mut self, stylebits: NcStyle) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_on_styles(self, stylebits.into()) },
            &format!("NcDirect.styles_on({:0X})", stylebits)
        ]
    }

    /// Sets just the specified styles.
    ///
    /// *C style function: [ncdirect_set_styles()][fns::ncdirect_set_styles].*
    pub fn styles_set(&mut self, stylebits: NcStyle) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_styles(self, stylebits.into()) },
            &format!("NcDirect.styles_set({:0X})", stylebits)
        ]
    }

    /// Returns an [`NcStyle`] with the supported curses-style attributes.
    ///
    /// The attribute is only indicated as supported if the terminal can support
    /// it together with color.
    ///
    /// For more information, see the "ncv" capability in terminfo(5).
    ///
    /// *C style function: [ncdirect_supported_styles()][fns::ncdirect_supported_styles].*
    pub fn supported_styles(&self) -> NcStyle {
        unsafe { fns::ncdirect_supported_styles(self) as NcStyle }
    }

    /// Indicates to use the "default color" for the foreground.
    ///
    /// *C style function: [ncdirect_set_fg_default()][fns::ncdirect_set_fg_default].*
    pub fn set_fg_default(&mut self) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_fg_default(self) },
            "NcDirect.set_fg_default()"
        ]
    }

    /// Indicates to use the "default color" for the background.
    ///
    /// *C style function: [ncdirect_set_bg_default()][fns::ncdirect_set_bg_default].*
    pub fn set_bg_default(&mut self) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_set_bg_default(self) },
            "NcDirect.set_bg_default()"
        ]
    }
}

/// ## NcDirect methods: capabilities, cursor, dimensions
impl NcDirect {
    /// Is there support for acquiring the cursor's current position?
    ///
    /// Requires the u7 terminfo capability, and that we are connected to an
    /// actual terminal.
    pub fn canget_cursor(&self) -> bool {
        unsafe { fns::ncdirect_canget_cursor(self) }
    }

    /// Can we reliably use Unicode braille?
    ///
    /// *C style function: [ncdirect_canbraille()][fns::ncdirect_canbraille].*
    pub fn canbraille(&self) -> bool {
        fns::ncdirect_canbraille(self)
    }

    /// Can we set the "hardware" palette?
    ///
    /// Requires the "ccc" terminfo capability.
    ///
    /// *C style function: [ncdirect_canchangecolor()][fns::ncdirect_canchangecolor].*
    pub fn canchangecolor(&self) -> bool {
        fns::ncdirect_canchangecolor(self)
    }

    /// Can we fade?
    ///
    /// Requires either the "rgb" or "ccc" terminfo capability.
    ///
    /// *C style function: [ncdirect_canfade()][fns::ncdirect_canfade].*
    pub fn canfade(&self) -> bool {
        fns::ncdirect_canfade(self)
    }

    /// Can we reliably use Unicode halfblocks?
    ///
    /// *C style function: [ncdirect_canhalfblock()][fns::ncdirect_canhalfblock].*
    pub fn canhalfblock(&self) -> bool {
        fns::ncdirect_canhalfblock(self)
    }

    /// Can we load images?
    ///
    /// Requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [ncdirect_canopen_images()][fns::ncdirect_canopen_images].*
    pub fn canopen_images(&self) -> bool {
        unsafe { fns::ncdirect_canopen_images(self) }
    }

    /// Can we load videos?
    ///
    /// Requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [ncdirect_canopen_videos()][fns::ncdirect_canopen_videos].*
    pub fn canopen_videos(&self) -> bool {
        fns::ncdirect_canopen_videos(self)
    }

    /// Can we reliably use Unicode quadrants?
    ///
    /// *C style function: [ncdirect_canquadrant()][fns::ncdirect_canquadrant].*
    pub fn canquadrant(&self) -> bool {
        fns::ncdirect_canquadrant(self)
    }

    /// Can we reliably use Unicode sextants?
    ///
    /// *C style function: [ncdirect_cansextant()][fns::ncdirect_cansextant].*
    pub fn cansextant(&self) -> bool {
        fns::ncdirect_cansextant(self)
    }

    /// Can we directly specify RGB values per cell, or only use palettes?
    ///
    /// *C style function: [ncdirect_cantruecolor()][fns::ncdirect_cantruecolor].*
    pub fn cantruecolor(&self) -> bool {
        fns::ncdirect_cantruecolor(self)
    }

    /// Is our encoding UTF-8?
    ///
    /// Requires LANG being set to a UTF8 locale.
    ///
    /// *C style function: [ncdirect_canutf8()][fns::ncdirect_canutf8].*
    pub fn canutf8(&self) -> bool {
        unsafe { fns::ncdirect_canutf8(self) }
    }

    /// Returns the [`NcCapabilities`].
    ///
    /// *C style function: [ncdirect_capabilities()][fns::ncdirect_capabilities].*
    pub fn capabilities(&self) -> NcCapabilities {
        fns::ncdirect_capabilities(self)
    }

    /// Checks for pixel support.
    ///
    /// Returns `false` for no support, or `true` if pixel output is supported.
    ///
    /// This function must successfully return before NCBLIT_PIXEL is available.
    ///
    /// Must not be called concurrently with either input or rasterization.
    ///
    /// *C style function: [ncdirect_check_pixel_support()][fns::ncdirect_check-pixel_support].*
    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn check_pixel_support(&self) -> NcResult<bool> {
        let res = unsafe { fns::ncdirect_check_pixel_support(self) };
        match res {
            0 => Ok(false),
            1 => Ok(true),
            NCRESULT_ERR | _ => Err(NcError::with_msg(res, "NcDirect.check_pixel_support()")),
        }
    }

    /// Disables the terminal's cursor, if supported.
    ///
    /// *C style function: [ncdirect_cursor_disable()][fns::ncdirect_cursor_disable].*
    pub fn cursor_disable(&mut self) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_cursor_disable(self) },
            "NcDirect.cursor_disable()"
        ]
    }

    /// Enables the terminal's cursor, if supported.
    ///
    /// *C style function: [ncdirect_cursor_enable()][fns::ncdirect_cursor_enable].*
    pub fn cursor_enable(&mut self) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_cursor_enable(self) },
            "NcDirect.cursor_enable()"
        ]
    }

    /// Moves the cursor down any number of rows.
    ///
    /// *C style function: [ncdirect_cursor_down()][fns::ncdirect_cursor_down].*
    pub fn cursor_down(&mut self, rows: NcOffset) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_cursor_down(self, rows as i32) },
            &format!("NcDirect.cursor_down({})", rows)
        ]
    }

    /// Moves the cursor left any number of columns.
    ///
    /// *C style function: [ncdirect_cursor_left()][fns::ncdirect_cursor_left].*
    pub fn cursor_left(&mut self, cols: NcOffset) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_cursor_left(self, cols as i32) },
            &format!("NcDirect.cursor_left({})", cols)
        ]
    }

    /// Moves the cursor right any number of columns.
    ///
    /// *C style function: [ncdirect_cursor_right()][fns::ncdirect_cursor_right].*
    pub fn cursor_right(&mut self, cols: NcOffset) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_cursor_right(self, cols as i32) },
            &format!("NcDirect.cursor_right({})", cols)
        ]
    }

    /// Moves the cursor up any number of rows.
    ///
    /// *C style function: [ncdirect_cursor_up()][fns::ncdirect_cursor_up].*
    pub fn cursor_up(&mut self, rows: NcOffset) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_cursor_up(self, rows as i32) },
            &format!("NcDirect.cursor_up({})", rows)
        ]
    }

    /// Sets the cursor to the specified row `y`, column `x`.
    ///
    /// *C style function: [ncdirect_cursor_move_yx()][fns::ncdirect_cursor_move_yx].*
    pub fn cursor_set_yx(&mut self, y: NcDim, x: NcDim) -> NcResult<()> {
        error![unsafe { fns::ncdirect_cursor_move_yx(self, y as i32, x as i32) }]
    }

    /// Sets the cursor to the specified row `y`.
    ///
    /// *(No equivalent C style function)*
    pub fn cursor_set_y(&mut self, y: NcDim) -> NcResult<()> {
        error![unsafe { fns::ncdirect_cursor_move_yx(self, y as i32, -1) }]
    }

    /// Sets the cursor to the specified column `x`.
    ///
    /// *(No equivalent C style function)*
    pub fn cursor_set_x(&mut self, x: NcDim) -> NcResult<()> {
        error![unsafe { fns::ncdirect_cursor_move_yx(self, -1, x as i32) }]
    }

    /// Gets the cursor (y, x) position, when supported.
    ///
    /// This requires writing to the terminal, and then reading from it.
    /// If the terminal doesn't reply, or doesn't reply in a way we understand,
    /// the results might be detrimental.
    ///
    /// *C style function: [ncdirect_cursor_yx()][fns::ncdirect_cursor_yx].*
    pub fn cursor_yx(&mut self) -> NcResult<(NcDim, NcDim)> {
        let (mut y, mut x) = (0, 0);
        error![
            unsafe { fns::ncdirect_cursor_yx(self, &mut y, &mut x) },
            "",
            (y as NcDim, x as NcDim)
        ]
    }

    /// Pushes the cursor location to the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    ///
    /// *C style function: [ncdirect_cursor_push()][fns::ncdirect_cursor_push].*
    pub fn cursor_push(&mut self) -> NcResult<()> {
        error![unsafe { fns::ncdirect_cursor_push(self) }]
    }

    /// Pops the cursor location from the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    ///
    /// *C style function: [ncdirect_cursor_pop()][fns::ncdirect_cursor_pop].*
    pub fn cursor_pop(&mut self) -> NcResult<()> {
        error![unsafe { fns::ncdirect_cursor_pop(self) }]
    }

    /// Gets the current number of rows.
    ///
    /// *C style function: [ncdirect_dim_y()][fns::ncdirect_dim_y].*
    pub fn dim_y(&mut self) -> NcDim {
        unsafe { fns::ncdirect_dim_y(self) as NcDim }
    }

    /// Gets the current number of columns.
    ///
    /// *C style function: [ncdirect_dim_x()][fns::ncdirect_dim_x].*
    pub fn dim_x(&mut self) -> NcDim {
        unsafe { fns::ncdirect_dim_x(self) as NcDim }
    }

    /// Gets the current number of rows and columns.
    ///
    /// *C style function: [ncdirect_dim_y()][fns::ncdirect_dim_y].*
    pub fn dim_yx(&mut self) -> (NcDim, NcDim) {
        let y = unsafe { fns::ncdirect_dim_y(self) as NcDim };
        let x = unsafe { fns::ncdirect_dim_x(self) as NcDim };
        (y, x)
    }

    /// Returns the name of the detected terminal.
    ///
    /// *C style function: [ncdirect_detected_terminal()][fns::ncdirect_detected_terminal].*
    pub fn detected_terminal(&self) -> String {
        rstring_free![fns::ncdirect_detected_terminal(self)]
    }
}

/// ## NcDirect methods: I/O
impl NcDirect {
    #[doc(hidden)]
    #[deprecated = "use `get` method instead"]
    pub fn getc(&mut self, time: Option<NcTime>, input: Option<&mut NcInput>) -> NcResult<char> {
        self.get(time, input)
    }

    /// Returns a [char] representing a single unicode point.
    ///
    /// If an event is processed, the return value is the `id` field from that
    /// event.
    ///
    /// Provide a None `time` to block at length, a `time` of 0 for non-blocking
    /// operation, and otherwise a timespec to bound blocking.
    ///
    /// *C style function: [ncdirect_get()][fns::ncdirect_get].*
    // CHECK returns 0 on a timeout.
    pub fn get(&mut self, time: Option<NcTime>, input: Option<&mut NcInput>) -> NcResult<char> {
        let ntime;
        if let Some(time) = time {
            ntime = &time as *const _;
        } else {
            ntime = null();
        }
        let ninput;
        if let Some(input) = input {
            ninput = input as *mut _;
        } else {
            ninput = null_mut();
        }

        let res = unsafe { fns::ncdirect_get(self, ntime, ninput) };
        core::char::from_u32(res)
            .ok_or_else(|| NcError::with_msg(res as i32, &format!["Nc.get(time: {:?})", time]))
    }

    /// Reads input blocking until an event is processed or a signal is received.
    ///
    /// Will optionally write the event details in `input`.
    ///
    /// In the case of a valid read, a [`char`] is returned.
    ///
    /// *C style function: [ncdirect_getc_blocking()][fns::ncdirect_getc_blocking].*
    pub fn getc_blocking(&mut self, input: Option<&mut NcInput>) -> NcResult<char> {
        let res = fns::ncdirect_getc_blocking(self, input);
        core::char::from_u32(res as u32)
            .ok_or_else(|| NcError::with_msg(res, "NcDirect.getc_blocking()"))
    }

    /// Reads input without blocking.
    ///
    /// In the case of a valid read, a [`char`] is returned.
    ///
    /// If no event is ready, returns 0.
    ///
    /// *C style function: [ncdirect_getc_nblock()][fns::ncdirect_getc_nblock].*
    pub fn getc_nblock(&mut self, input: Option<&mut NcInput>) -> NcResult<char> {
        let res = fns::ncdirect_getc_nblock(self, input);
        core::char::from_u32(res as u32)
            .ok_or_else(|| NcError::with_msg(res, "NcDirect.getc_nblock()"))
    }

    /// Get a file descriptor suitable for input event poll()ing.
    ///
    /// When this descriptor becomes available, you can call
    /// [getc_nblock()][NcDirect#method.getc_nblock], and input ought be ready.
    ///
    /// This file descriptor is not necessarily the file descriptor associated
    /// with stdin (but it might be!).
    ///
    /// *C style function: [ncdirect_inputready_fd()][fns::ncdirect_inputready_fd].*
    pub fn inputready_fd(&mut self) -> NcResult<()> {
        error![unsafe { fns::ncdirect_inputready_fd(self) }]
    }

    /// Outputs the `string` according to the `channels`, and
    /// returns the total number of characters written on success.
    ///
    /// Note that it does not explicitly flush output buffers, so it will not
    /// necessarily be immediately visible.
    ///
    /// It will fail if the NcDirect context and the foreground channel
    /// are both marked as using the default color.
    ///
    /// *C style function: [ncdirect_putstr()][fns::ncdirect_putstr].*
    pub fn putstr(&mut self, channels: NcChannels, string: &str) -> NcResult<()> {
        error![
            unsafe { fns::ncdirect_putstr(self, channels, cstring![string]) },
            &format!("NcDirect.putstr({:0X}, {:?})", channels, string)
        ]
    }

    /// Reads a (heap-allocated) line of text using the Readline library.
    ///
    /// Initializes Readline the first time it's called.
    ///
    /// For input to be echoed to the terminal, it is necessary that the flag
    /// [NCDIRECT_OPTION_INHIBIT_CBREAK][crate::NCDIRECT_OPTION_INHIBIT_CBREAK]
    /// be provided to the constructor.
    ///
    /// *C style function: [ncdirect_readline()][fns::ncdirect_readline].*
    //
    // FIXME: memory leak still reported by valgrind
    pub fn readline(&mut self, prompt: &str) -> NcResult<String> {
        let res = unsafe { fns::ncdirect_readline(self, cstring![prompt]) };
        if !res.is_null() {
            return Ok(rstring_free![res]);
        } else {
            Err(NcError::with_msg(
                NCRESULT_ERR,
                &format!["NcDirect.readline({})", prompt],
            ))
        }
    }

    /// Draws a box with its upper-left corner at the current cursor position,
    /// having dimensions `ylen` * `xlen`.
    ///
    /// See NcPlane.[box()][NcPlane#method.box] for more information.
    ///
    /// The minimum box size is 2x2, and it cannot be drawn off-screen.
    ///
    /// `wchars` is an array of 6 characters: UL, UR, LL, LR, HL, VL.
    ///
    /// *C style function: [ncdirect_box()][fns::ncdirect_box].*
    // TODO: CHECK, specially wchars.
    pub fn r#box(
        &mut self,
        ul: NcChannels,
        ur: NcChannels,
        ll: NcChannels,
        lr: NcChannels,
        wchars: &[char; 6],
        y_len: NcDim,
        x_len: NcDim,
        ctlword: u32,
    ) -> NcResult<()> {
        error![
            unsafe {
                let wchars = core::mem::transmute(wchars);
                fns::ncdirect_box(
                    self,
                    ul,
                    ur,
                    ll,
                    lr,
                    wchars,
                    y_len as i32,
                    x_len as i32,
                    ctlword,
                )
            },
            &format!(
                "NcDirect.box({:0X}, {:0X}, {:0X}, {:0X}, {:?}, {}, {}, {})",
                ul, ur, ll, lr, wchars, y_len, x_len, ctlword
            )
        ]
    }

    /// NcDirect.[box()][NcDirect#method.box] with the double box-drawing characters.
    ///
    /// *C style function: [ncdirect_double_box()][fns::ncdirect_double_box].*
    pub fn double_box(
        &mut self,
        ul: NcChannels,
        ur: NcChannels,
        ll: NcChannels,
        lr: NcChannels,
        y_len: NcDim,
        x_len: NcDim,
        ctlword: u32,
    ) -> NcResult<()> {
        error![unsafe {
            fns::ncdirect_double_box(self, ul, ur, ll, lr, y_len as i32, x_len as i32, ctlword)
        }]
    }

    /// NcDirect.[box()][NcDirect#method.box] with the rounded box-drawing characters.
    ///
    /// *C style function: [ncdirect_rounded_box()][fns::ncdirect_rounded_box].*
    pub fn rounded_box(
        &mut self,
        ul: NcChannels,
        ur: NcChannels,
        ll: NcChannels,
        lr: NcChannels,
        y_len: NcDim,
        x_len: NcDim,
        ctlword: u32,
    ) -> NcResult<()> {
        error![unsafe {
            fns::ncdirect_rounded_box(self, ul, ur, ll, lr, y_len as i32, x_len as i32, ctlword)
        }]
    }

    /// Draws horizontal lines using the specified [NcChannels]s, interpolating
    /// between them as we go.
    ///
    /// All lines start at the current cursor position.
    ///
    /// The string at `egc` may not use more than one column.
    ///
    /// For a horizontal line, `len` cannot exceed the screen width minus the
    /// cursor's offset.
    ///
    /// *C style function: [ncdirect_hline_interp()][fns::ncdirect_hline_interp].*
    #[inline]
    pub fn hline_interp(
        &mut self,
        egc: &str,
        len: NcDim,
        h1: NcChannels,
        h2: NcChannels,
    ) -> NcResult<()> {
        error![fns::ncdirect_hline_interp(self, egc, len, h1, h2)]
    }

    /// Draws horizontal lines using the specified [NcChannels]s, interpolating
    /// between them as we go.
    ///
    /// All lines start at the current cursor position.
    ///
    /// The string at `egc` may not use more than one column.
    ///
    /// For a vertical line, `len` may be as long as you'd like; the screen
    /// will scroll as necessary.
    ///
    /// *C style function: [ncdirect_vline_interp()][fns::ncdirect_vline_interp].*
    #[inline]
    pub fn vline_interp(
        &mut self,
        egc: &str,
        len: NcDim,
        h1: NcChannels,
        h2: NcChannels,
    ) -> NcResult<()> {
        error![fns::ncdirect_vline_interp(self, egc, len, h1, h2)]
    }
}
