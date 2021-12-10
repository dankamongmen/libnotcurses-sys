//! `NcDirect` methods and associated functions.

use core::ptr::{null, null_mut};

use crate::{
    c_api, cstring, error, error_ref_mut, rstring_free, NcAlign, NcBlitter, NcCapabilities,
    NcChannels, NcComponent, NcDim, NcDirect, NcDirectFlags, NcError, NcInput, NcOffset,
    NcPaletteIndex, NcPlane, NcResult, NcRgb, NcScale, NcStyle, NcTime,
};

/// # `NcDirect` constructors and destructors
impl NcDirect {
    /// New NcDirect with the default options.
    ///
    /// Initializes a direct-mode notcurses context on the tty.
    ///
    /// Direct mode supports a limited subset of notcurses routines,
    /// and neither supports nor requires
    /// [notcurses_render()][c_api::notcurses_render]. This can be used to add
    /// color and styling to text in the standard output paradigm.
    ///
    /// # Safety
    /// You must not create multiple `NcDirect` instances at the same time, on
    /// the same thread. You must [`stop`][NcDirect#method.stop] the current one
    /// before creating a new one.
    ///
    /// *C style function: [ncdirect_init()][c_api::ncdirect_init].*
    pub unsafe fn new<'a>() -> NcResult<&'a mut NcDirect> {
        Self::with_flags(0)
    }

    /// New NcDirect with optional flags.
    ///
    /// # Safety
    /// You must not create multiple `NcDirect` instances at the same time, on
    /// the same thread. You must [`stop`][NcDirect#method.stop] the current one
    /// before creating a new one.
    ///
    /// *C style function: [ncdirect_init()][c_api::ncdirect_init].*
    pub unsafe fn with_flags<'a>(flags: NcDirectFlags) -> NcResult<&'a mut NcDirect> {
        let res = c_api::ncdirect_init(null(), null_mut(), flags);
        error_ref_mut![res, "Initializing NcDirect"]
    }

    /// Releases this NcDirect and any associated resources.
    ///
    /// # Safety
    /// You must not call this method repeatedly on the same `NcDirect` instance.
    ///
    /// *C style function: [ncdirect_stop()][c_api::ncdirect_stop].*
    pub unsafe fn stop(&mut self) -> NcResult<()> {
        error![c_api::ncdirect_stop(self), "NcDirect.stop()"]
    }
}

/// ## NcDirect methods: clear, flush, render
impl NcDirect {
    /// Clears the screen.
    ///
    /// *C style function: [ncdirect_clear()][c_api::ncdirect_clear].*
    pub fn clear(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_clear(self) }, "NcDirect.clear()"]
    }

    /// Forces a flush.
    ///
    /// *C style function: [ncdirect_flush()][c_api::ncdirect_flush].*
    pub fn flush(&self) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_flush(self) }, "NcDirect.clear()"]
    }

    /// Takes the result of [`render_frame`][NcDirect#method.render_frame]
    /// and writes it to the output.
    ///
    /// *C style function: [ncdirect_raster_frame()][c_api::ncdirect_raster_frame].*
    pub fn raster_frame(&mut self, frame: &mut NcPlane, align: NcAlign) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_raster_frame(self, frame, align.into()) },
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
    /// *C style function: [ncdirect_render_frame()][c_api::ncdirect_render_frame].*
    pub fn render_frame<'a>(
        &mut self,
        filename: &str,
        blitter: NcBlitter,
        scale: NcScale,
        max_y: NcDim,
        max_x: NcDim,
    ) -> NcResult<&'a mut NcPlane> {
        let res = unsafe {
            c_api::ncdirect_render_frame(
                self,
                cstring![filename],
                blitter.into(),
                scale.into(),
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
    /// *C style function: [ncdirect_render_image()][c_api::ncdirect_render_image].*
    pub fn render_image(
        &mut self,
        filename: &str,
        align: NcAlign,
        blitter: NcBlitter,
        scale: NcScale,
    ) -> NcResult<()> {
        error![
            unsafe {
                c_api::ncdirect_render_image(
                    self,
                    cstring![filename],
                    align.into(),
                    blitter.into(),
                    scale.into(),
                )
            },
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
    /// *C style function: [ncdirect_set_fg_palindex()][c_api::ncdirect_set_fg_palindex].*
    pub fn set_fg_palindex(&mut self, index: NcPaletteIndex) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_fg_palindex(self, index as i32) },
            &format!("NcDirect.set_fg_palindex({})", index)
        ]
    }

    /// Sets the background [NcPaletteIndex].
    ///
    /// *C style function: [ncdirect_set_bg_palindex()][c_api::ncdirect_set_bg_palindex].*
    pub fn set_bg_palindex(&mut self, index: NcPaletteIndex) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_bg_palindex(self, index as i32) },
            &format!("NcDirect.set_fg_palindex({})", index)
        ]
    }

    /// Returns the number of simultaneous colors claimed to be supported,
    /// if there is color support.
    ///
    /// Note that several terminal emulators advertise more colors than they
    /// actually support, downsampling internally.
    ///
    /// *C style function: [ncdirect_palette_size()][c_api::ncdirect_palette_size].*
    pub fn palette_size(&self) -> NcResult<u32> {
        let res = unsafe { c_api::ncdirect_palette_size(self) };
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
    /// *C style function: [ncdirect_set_fg_rgb()][c_api::ncdirect_set_fg_rgb].*
    pub fn set_fg_rgb(&mut self, rgb: NcRgb) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_fg_rgb(self, rgb) },
            &format!("NcDirect.set_fg_rgb({})", rgb)
        ]
    }

    /// Sets the background [NcRgb].
    ///
    /// *C style function: [ncdirect_set_bg_rgb()][c_api::ncdirect_set_bg_rgb].*
    pub fn set_bg_rgb(&mut self, rgb: NcRgb) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_bg_rgb(self, rgb) },
            &format!("NcDirect.set_bg_rgb({})", rgb)
        ]
    }

    /// Sets the foreground [NcComponent] components.
    ///
    /// *C style function: [ncdirect_set_fg_rgb8()][c_api::ncdirect_set_fg_rgb8].*
    pub fn set_fg_rgb8(
        &mut self,
        red: NcComponent,
        green: NcComponent,
        blue: NcComponent,
    ) -> NcResult<()> {
        error![
            c_api::ncdirect_set_fg_rgb8(self, red, green, blue),
            &format!("NcDirect.set_fg_rgb8({}, {}, {})", red, green, blue)
        ]
    }

    /// Sets the background [NcComponent] components.
    ///
    /// *C style function: [ncdirect_set_bg_rgb()][c_api::ncdirect_set_bg_rgb].*
    pub fn set_bg_rgb8(
        &mut self,
        red: NcComponent,
        green: NcComponent,
        blue: NcComponent,
    ) -> NcResult<()> {
        error![
            c_api::ncdirect_set_bg_rgb8(self, red, green, blue),
            &format!("NcDirect.set_bg_rgb8({}, {}, {})", red, green, blue)
        ]
    }

    /// Returns the current styling.
    ///
    /// *C style function: [ncdirect_styles()][c_api::ncdirect_styles].*
    pub fn styles(&self) -> NcStyle {
        unsafe { c_api::ncdirect_styles(self) as NcStyle }
    }

    /// Removes the specified styles.
    ///
    /// *C style function: [ncdirect_off_styles()][c_api::ncdirect_off_styles].*
    pub fn styles_off(&mut self, stylebits: NcStyle) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_off_styles(self, stylebits.into()) },
            &format!("NcDirect.styles_off({:0X})", stylebits)
        ]
    }

    /// Adds the specified styles.
    ///
    /// *C style function: [ncdirect_on_styles()][c_api::ncdirect_on_styles].*
    pub fn styles_on(&mut self, stylebits: NcStyle) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_on_styles(self, stylebits.into()) },
            &format!("NcDirect.styles_on({:0X})", stylebits)
        ]
    }

    /// Sets just the specified styles.
    ///
    /// *C style function: [ncdirect_set_styles()][c_api::ncdirect_set_styles].*
    pub fn styles_set(&mut self, stylebits: NcStyle) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_styles(self, stylebits.into()) },
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
    /// *C style function: [ncdirect_supported_styles()][c_api::ncdirect_supported_styles].*
    pub fn supported_styles(&self) -> NcStyle {
        unsafe { c_api::ncdirect_supported_styles(self) as NcStyle }
    }

    /// Indicates to use the "default color" for the foreground.
    ///
    /// *C style function: [ncdirect_set_fg_default()][c_api::ncdirect_set_fg_default].*
    pub fn set_fg_default(&mut self) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_fg_default(self) },
            "NcDirect.set_fg_default()"
        ]
    }

    /// Indicates to use the "default color" for the background.
    ///
    /// *C style function: [ncdirect_set_bg_default()][c_api::ncdirect_set_bg_default].*
    pub fn set_bg_default(&mut self) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_set_bg_default(self) },
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
        unsafe { c_api::ncdirect_canget_cursor(self) }
    }

    /// Can we reliably use Unicode braille?
    ///
    /// *C style function: [ncdirect_canbraille()][c_api::ncdirect_canbraille].*
    pub fn canbraille(&self) -> bool {
        c_api::ncdirect_canbraille(self)
    }

    /// Can we set the "hardware" palette?
    ///
    /// Requires the "ccc" terminfo capability.
    ///
    /// *C style function: [ncdirect_canchangecolor()][c_api::ncdirect_canchangecolor].*
    pub fn canchangecolor(&self) -> bool {
        c_api::ncdirect_canchangecolor(self)
    }

    /// Can we fade?
    ///
    /// Requires either the "rgb" or "ccc" terminfo capability.
    ///
    /// *C style function: [ncdirect_canfade()][c_api::ncdirect_canfade].*
    pub fn canfade(&self) -> bool {
        c_api::ncdirect_canfade(self)
    }

    /// Can we reliably use Unicode halfblocks?
    ///
    /// *C style function: [ncdirect_canhalfblock()][c_api::ncdirect_canhalfblock].*
    pub fn canhalfblock(&self) -> bool {
        c_api::ncdirect_canhalfblock(self)
    }

    /// Can we load images?
    ///
    /// Requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [ncdirect_canopen_images()][c_api::ncdirect_canopen_images].*
    pub fn canopen_images(&self) -> bool {
        c_api::ncdirect_canopen_images(self)
    }

    /// Can we load videos?
    ///
    /// Requires being built against FFmpeg/OIIO.
    ///
    /// *C style function: [ncdirect_canopen_videos()][c_api::ncdirect_canopen_videos].*
    pub fn canopen_videos(&self) -> bool {
        c_api::ncdirect_canopen_videos(self)
    }

    /// Can we reliably use Unicode quadrants?
    ///
    /// *C style function: [ncdirect_canquadrant()][c_api::ncdirect_canquadrant].*
    pub fn canquadrant(&self) -> bool {
        c_api::ncdirect_canquadrant(self)
    }

    /// Can we reliably use Unicode sextants?
    ///
    /// *C style function: [ncdirect_cansextant()][c_api::ncdirect_cansextant].*
    pub fn cansextant(&self) -> bool {
        c_api::ncdirect_cansextant(self)
    }

    /// Can we directly specify RGB values per cell, or only use palettes?
    ///
    /// *C style function: [ncdirect_cantruecolor()][c_api::ncdirect_cantruecolor].*
    pub fn cantruecolor(&self) -> bool {
        c_api::ncdirect_cantruecolor(self)
    }

    /// Is our encoding UTF-8?
    ///
    /// Requires LANG being set to a UTF8 locale.
    ///
    /// *C style function: [ncdirect_canutf8()][c_api::ncdirect_canutf8].*
    pub fn canutf8(&self) -> bool {
        unsafe { c_api::ncdirect_canutf8(self) }
    }

    /// Returns the [`NcCapabilities`].
    ///
    /// *C style function: [ncdirect_capabilities()][c_api::ncdirect_capabilities].*
    pub fn capabilities(&self) -> NcCapabilities {
        c_api::ncdirect_capabilities(self)
    }

    /// Checks for pixel support.
    ///
    /// Returns `false` for no support, or `true` if pixel output is supported.
    ///
    /// This function must successfully return before NCBLIT_PIXEL is available.
    ///
    /// Must not be called concurrently with either input or rasterization.
    ///
    /// *C style function: [ncdirect_check_pixel_support()][c_api::ncdirect_check-pixel_support].*
    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn check_pixel_support(&self) -> NcResult<bool> {
        let res = unsafe { c_api::ncdirect_check_pixel_support(self) };
        match res {
            0 => Ok(false),
            1 => Ok(true),
            c_api::NCRESULT_ERR | _ => {
                Err(NcError::with_msg(res, "NcDirect.check_pixel_support()"))
            }
        }
    }

    /// Disables the terminal's cursor, if supported.
    ///
    /// *C style function: [ncdirect_cursor_disable()][c_api::ncdirect_cursor_disable].*
    pub fn cursor_disable(&mut self) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_cursor_disable(self) },
            "NcDirect.cursor_disable()"
        ]
    }

    /// Enables the terminal's cursor, if supported.
    ///
    /// *C style function: [ncdirect_cursor_enable()][c_api::ncdirect_cursor_enable].*
    pub fn cursor_enable(&mut self) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_cursor_enable(self) },
            "NcDirect.cursor_enable()"
        ]
    }

    /// Moves the cursor down any number of rows.
    ///
    /// *C style function: [ncdirect_cursor_down()][c_api::ncdirect_cursor_down].*
    pub fn cursor_down(&mut self, rows: NcOffset) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_cursor_down(self, rows as i32) },
            &format!("NcDirect.cursor_down({})", rows)
        ]
    }

    /// Moves the cursor left any number of columns.
    ///
    /// *C style function: [ncdirect_cursor_left()][c_api::ncdirect_cursor_left].*
    pub fn cursor_left(&mut self, cols: NcOffset) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_cursor_left(self, cols as i32) },
            &format!("NcDirect.cursor_left({})", cols)
        ]
    }

    /// Moves the cursor right any number of columns.
    ///
    /// *C style function: [ncdirect_cursor_right()][c_api::ncdirect_cursor_right].*
    pub fn cursor_right(&mut self, cols: NcOffset) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_cursor_right(self, cols as i32) },
            &format!("NcDirect.cursor_right({})", cols)
        ]
    }

    /// Moves the cursor up any number of rows.
    ///
    /// *C style function: [ncdirect_cursor_up()][c_api::ncdirect_cursor_up].*
    pub fn cursor_up(&mut self, rows: NcOffset) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_cursor_up(self, rows as i32) },
            &format!("NcDirect.cursor_up({})", rows)
        ]
    }

    /// Sets the cursor to the specified row `y`, column `x`.
    ///
    /// *C style function: [ncdirect_cursor_move_yx()][c_api::ncdirect_cursor_move_yx].*
    pub fn cursor_set_yx(&mut self, y: NcDim, x: NcDim) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_cursor_move_yx(self, y as i32, x as i32) }]
    }

    /// Sets the cursor to the specified row `y`.
    ///
    /// *(No equivalent C style function)*
    pub fn cursor_set_y(&mut self, y: NcDim) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_cursor_move_yx(self, y as i32, -1) }]
    }

    /// Sets the cursor to the specified column `x`.
    ///
    /// *(No equivalent C style function)*
    pub fn cursor_set_x(&mut self, x: NcDim) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_cursor_move_yx(self, -1, x as i32) }]
    }

    /// Gets the cursor (y, x) position, when supported.
    ///
    /// This requires writing to the terminal, and then reading from it.
    /// If the terminal doesn't reply, or doesn't reply in a way we understand,
    /// the results might be detrimental.
    ///
    /// *C style function: [ncdirect_cursor_yx()][c_api::ncdirect_cursor_yx].*
    pub fn cursor_yx(&mut self) -> NcResult<(NcDim, NcDim)> {
        let (mut y, mut x) = (0, 0);
        error![
            unsafe { c_api::ncdirect_cursor_yx(self, &mut y, &mut x) },
            "",
            (y as NcDim, x as NcDim)
        ]
    }

    /// Pushes the cursor location to the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    ///
    /// *C style function: [ncdirect_cursor_push()][c_api::ncdirect_cursor_push].*
    pub fn cursor_push(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_cursor_push(self) }]
    }

    /// Pops the cursor location from the terminal's stack.
    ///
    /// The depth of this stack, and indeed its existence, is terminal-dependent.
    ///
    /// *C style function: [ncdirect_cursor_pop()][c_api::ncdirect_cursor_pop].*
    pub fn cursor_pop(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_cursor_pop(self) }]
    }

    /// Gets the current number of rows.
    ///
    /// *C style function: [ncdirect_dim_y()][c_api::ncdirect_dim_y].*
    pub fn dim_y(&mut self) -> NcDim {
        unsafe { c_api::ncdirect_dim_y(self) as NcDim }
    }

    /// Gets the current number of columns.
    ///
    /// *C style function: [ncdirect_dim_x()][c_api::ncdirect_dim_x].*
    pub fn dim_x(&mut self) -> NcDim {
        unsafe { c_api::ncdirect_dim_x(self) as NcDim }
    }

    /// Gets the current number of rows and columns.
    ///
    /// *C style function: [ncdirect_dim_y()][c_api::ncdirect_dim_y].*
    pub fn dim_yx(&mut self) -> (NcDim, NcDim) {
        let y = unsafe { c_api::ncdirect_dim_y(self) as NcDim };
        let x = unsafe { c_api::ncdirect_dim_x(self) as NcDim };
        (y, x)
    }

    /// Returns the name of the detected terminal.
    ///
    /// *C style function: [ncdirect_detected_terminal()][c_api::ncdirect_detected_terminal].*
    pub fn detected_terminal(&self) -> String {
        rstring_free![c_api::ncdirect_detected_terminal(self)]
    }
}

/// ## NcDirect methods: I/O
impl NcDirect {
    /// Returns a [char] representing a single unicode point.
    ///
    /// If an event is processed, the return value is the `id` field from that
    /// event.
    ///
    /// Provide a None `time` to block at length, a `time` of 0 for non-blocking
    /// operation, and otherwise a timespec to bound blocking.
    ///
    /// *C style function: [ncdirect_get()][c_api::ncdirect_get].*
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

        let res = unsafe { c_api::ncdirect_get(self, ntime, ninput) };
        core::char::from_u32(res)
            .ok_or_else(|| NcError::with_msg(res as i32, &format!["Nc.get(time: {:?})", time]))
    }

    /// Reads input blocking until an event is processed or a signal is received.
    ///
    /// Will optionally write the event details in `input`.
    ///
    /// In the case of a valid read, a [`char`] is returned.
    ///
    /// *C style function: [ncdirect_get_blocking()][c_api::ncdirect_get_blocking].*
    pub fn get_blocking(&mut self, input: Option<&mut NcInput>) -> NcResult<char> {
        let res = c_api::ncdirect_get_blocking(self, input);
        core::char::from_u32(res as u32)
            .ok_or_else(|| NcError::with_msg(res, "NcDirect.get_blocking()"))
    }

    /// Reads input without blocking.
    ///
    /// In the case of a valid read, a [`char`] is returned.
    ///
    /// If no event is ready, returns 0.
    ///
    /// *C style function: [ncdirect_get_nblock()][c_api::ncdirect_get_nblock].*
    pub fn get_nblock(&mut self, input: Option<&mut NcInput>) -> NcResult<char> {
        let res = c_api::ncdirect_get_nblock(self, input);
        core::char::from_u32(res as u32)
            .ok_or_else(|| NcError::with_msg(res, "NcDirect.get_nblock()"))
    }

    /// Get a file descriptor suitable for input event poll()ing.
    ///
    /// When this descriptor becomes available, you can call
    /// [get_nblock()][NcDirect#method.get_nblock], and input ought be ready.
    ///
    /// This file descriptor is not necessarily the file descriptor associated
    /// with stdin (but it might be!).
    ///
    /// *C style function: [ncdirect_inputready_fd()][c_api::ncdirect_inputready_fd].*
    pub fn inputready_fd(&mut self) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_inputready_fd(self) }]
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
    /// *C style function: [ncdirect_putstr()][c_api::ncdirect_putstr].*
    pub fn putstr(&mut self, channels: NcChannels, string: &str) -> NcResult<()> {
        error![
            unsafe { c_api::ncdirect_putstr(self, channels, cstring![string]) },
            &format!("NcDirect.putstr({:0X}, {:?})", channels, string)
        ]
    }

    /// Reads a (heap-allocated) line of text using the Readline library.
    ///
    /// Initializes Readline the first time it's called.
    ///
    /// For input to be echoed to the terminal, it is necessary that the flag
    /// [`NcDirectFlags::INHIBIT_CBREAK`][NcDirectFlags#associatedconstant.INHIBIT_CBREAK]
    /// be provided to the constructor.
    ///
    /// *C style function: [ncdirect_readline()][c_api::ncdirect_readline].*
    //
    // FIXME: memory leak still reported by valgrind
    pub fn readline(&mut self, prompt: &str) -> NcResult<String> {
        let res = unsafe { c_api::ncdirect_readline(self, cstring![prompt]) };
        if !res.is_null() {
            return Ok(rstring_free![res]);
        } else {
            Err(NcError::with_msg(
                c_api::NCRESULT_ERR,
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
    /// *C style function: [ncdirect_box()][c_api::ncdirect_box].*
    // TODO: CHECK, specially wchars.
    pub fn r#box(
        &mut self,
        ul: NcChannels,
        ur: NcChannels,
        ll: NcChannels,
        lr: NcChannels,
        wchars: &[char; 6],
        len_y: NcDim,
        len_x: NcDim,
        ctlword: u32,
    ) -> NcResult<()> {
        error![
            unsafe {
                let wchars = core::mem::transmute(wchars);
                c_api::ncdirect_box(self, ul, ur, ll, lr, wchars, len_y, len_x, ctlword)
            },
            &format!(
                "NcDirect.box({:0X}, {:0X}, {:0X}, {:0X}, {:?}, {}, {}, {})",
                ul, ur, ll, lr, wchars, len_y, len_x, ctlword
            )
        ]
    }

    /// NcDirect.[box()][NcDirect#method.box] with the double box-drawing characters.
    ///
    /// *C style function: [ncdirect_double_box()][c_api::ncdirect_double_box].*
    pub fn double_box(
        &mut self,
        ul: NcChannels,
        ur: NcChannels,
        ll: NcChannels,
        lr: NcChannels,
        len_y: NcDim,
        len_x: NcDim,
        ctlword: u32,
    ) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_double_box(self, ul, ur, ll, lr, len_y, len_x, ctlword) }]
    }

    /// NcDirect.[box()][NcDirect#method.box] with the rounded box-drawing characters.
    ///
    /// *C style function: [ncdirect_rounded_box()][c_api::ncdirect_rounded_box].*
    pub fn rounded_box(
        &mut self,
        ul: NcChannels,
        ur: NcChannels,
        ll: NcChannels,
        lr: NcChannels,
        len_y: NcDim,
        len_x: NcDim,
        ctlword: u32,
    ) -> NcResult<()> {
        error![unsafe { c_api::ncdirect_rounded_box(self, ul, ur, ll, lr, len_y, len_x, ctlword) }]
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
    /// *C style function: [ncdirect_hline_interp()][c_api::ncdirect_hline_interp].*
    #[inline]
    pub fn hline_interp(
        &mut self,
        egc: &str,
        len: NcDim,
        h1: NcChannels,
        h2: NcChannels,
    ) -> NcResult<()> {
        error![c_api::ncdirect_hline_interp(self, egc, len, h1, h2)]
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
    /// *C style function: [ncdirect_vline_interp()][c_api::ncdirect_vline_interp].*
    #[inline]
    pub fn vline_interp(
        &mut self,
        egc: &str,
        len: NcDim,
        h1: NcChannels,
        h2: NcChannels,
    ) -> NcResult<()> {
        error![c_api::ncdirect_vline_interp(self, egc, len, h1, h2)]
    }
}
