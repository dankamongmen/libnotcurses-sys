//! `NcVisual*` methods and associated functions.

use core::ptr::{null, null_mut};
use libc::c_void;

use crate::{
    c_api, cstring, error, error_ref_mut, rstring_free, Nc, NcBlitter, NcBlitterApi, NcChannel,
    NcComponent, NcDim, NcDirect, NcError, NcIntResult, NcIntResultApi, NcPixel, NcPlane, NcResult,
    NcRgba, NcScale, NcScaleApi, NcTime, NcVGeom, NcVisual, NcVisualOptions,
};

/// # NcVisualOptions Constructors
impl NcVisualOptions {
    /// New empty NcVisualOptions
    pub fn new() -> Self {
        Self {
            n: null_mut(),
            scaling: NcScale::NOSCALE,
            y: 0,
            x: 0,
            begy: 0,
            begx: 0,
            leny: 0,
            lenx: 0,
            blitter: NcBlitter::DEFAULT,
            flags: 0,
            transcolor: 0,
            pxoffy: 0,
            pxoffx: 0,
        }
    }

    // pub fn new_aligned() -> Self {
    //     Self::with_flags_aligned()
    // }

    // TODO:
    // - horizontally aligned
    // - copy from NcPlaneOptions (with_flags_aligned & with_flags,)
    // y is an ncalign_e if NCVISUAL_OPTION_VERALIGNED is provided.
    // x is an ncalign_e value if NCVISUAL_OPTION_HORALIGNED is provided.

    /// Specify an existing plane.
    ///
    /// If [`NcVisualOptions::CHILDPLANE`][NcVisualOptions#associatedconstant.CHILDPLANE]
    /// is used in `flags` then the `plane` is interpreted as the parent
    /// [`NcPlane`] of the new plane created for this [`NcVisual`].
    pub fn with_plane(
        plane: &mut NcPlane,
        scale: NcScale,
        y: NcDim,
        x: NcDim,
        beg_y: NcDim,
        beg_x: NcDim,
        len_y: NcDim,
        len_x: NcDim,
        // pxoff_y: NcDim,
        // pxoff_x: NcDim,
        blitter: NcBlitter,
        flags: u32,
        transcolor: NcRgba,
    ) -> Self {
        Self {
            // provided plane
            n: plane,
            // the source is stretched/scaled relative to the provided ncplane
            scaling: scale,
            y: y as i32,
            x: x as i32,
            // origin of rendered section
            begy: beg_y as i32,
            begx: beg_x as i32,
            // size of rendered section
            leny: len_y as i32,
            lenx: len_x as i32,
            // glyph set to use
            blitter,
            // bitmask over NCVISUAL_OPTION_*
            flags: flags as u64,
            transcolor,
            // WIP
            pxoffy: 0,
            pxoffx: 0,
        }
    }

    // TODO: use Option<> groups for coords
    pub fn without_plane(
        y: NcDim,
        x: NcDim,
        beg_y: NcDim,
        beg_x: NcDim,
        len_y: NcDim,
        len_x: NcDim,
        // pxoff_y: NcDim,
        // pxoff_x: NcDim,
        blitter: NcBlitter,
        flags: u32,
        transcolor: u32,
    ) -> Self {
        Self {
            n: null_mut(),
            scaling: crate::c_api::NCSCALE_NONE,
            // where the created plane will be placed relative to stdplane's origin
            y: y as i32,
            x: x as i32,
            // origin of rendered section
            begy: beg_y as i32,
            begx: beg_x as i32,
            // size of rendered section
            leny: len_y as i32,
            lenx: len_x as i32,
            // glyph set to use
            blitter,
            // bitmask over NCVISUAL_OPTION_*
            flags: flags as u64,
            // This color will be treated as transparent with flag [NCVISUAL_OPTION_ADDALPHA].
            transcolor,
            // pixel offsets within the cell.
            // if NCBLIT_PIXEL is used, the bitmap will be drawn offset from the
            // upper-left cell's origin by these amounts. it is an error if
            // either number exceeds the cell-pixel geometry in its dimension.
            // if NCBLIT_PIXEL is not used, these fields are ignored.
            // this functionality can be used for smooth bitmap movement.
            // WIP
            pxoffy: 0,
            pxoffx: 0,
            // pxoffy: pxoff_y,
            // pxoffx: pxoff_x,
        }
    }

    pub fn fullsize_pixel_without_plane(y: NcDim, x: NcDim, len_y: NcDim, len_x: NcDim) -> Self {
        Self::without_plane(y, x, 0, 0, len_y, len_x, NcBlitter::PIXEL, 0, 0)
    }
}

/// # NcVisual Constructors & destructors
impl NcVisual {
    /// Like [from_rgba][NcVisual#method.from_rgba], but 'bgra' is arranged as BGRA.
    ///
    /// *C style function: [ncvisual_from_bgra()][c_api::ncvisual_from_bgra].*
    pub fn from_bgra<'a>(
        bgra: &[u8],
        rows: NcDim,
        rowstride: NcDim,
        cols: NcDim,
    ) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe {
                c_api::ncvisual_from_bgra(
                    bgra.as_ptr() as *const c_void,
                    rows as i32,
                    rowstride as i32,
                    cols as i32,
                )
            },
            &format![
                "NcVisual::from_bgra(bgra, {}, {}, {})",
                rows, rowstride, cols
            ]
        ]
    }

    /// Opens a visual at `file`, extracts the codec and parameters and
    /// decodes the first image to memory.
    ///
    /// *C style function: [ncvisual_from_file()][c_api::ncvisual_from_file].*
    pub fn from_file<'a>(file: &str) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe { c_api::ncvisual_from_file(cstring![file]) },
            &format!("NcVisual::from_file({})", file)
        ]
    }

    /// Promotes an NcPlane to an NcVisual.
    ///
    /// The plane may contain only spaces, half blocks, and full blocks.
    /// This will be checked, and any other glyph will result in an error.
    ///
    /// This function exists so that planes can be subjected to NcVisual transformations.
    /// If possible, it's better to create the ncvisual from memory using
    /// [from_rgba][NcVisual#method.from_rgba].
    ///
    /// *C style function: [ncvisual_from_plane()][c_api::ncvisual_from_plane].*
    pub fn from_plane<'a>(
        plane: &NcPlane,
        blitter: NcBlitter,
        beg_y: NcDim,
        beg_x: NcDim,
        len_y: NcDim,
        len_x: NcDim,
    ) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe { c_api::ncvisual_from_plane(plane, blitter, beg_y, beg_x, len_y, len_x,) },
            &format!(
                "NcVisual::from_file(plane, {}, {}, {}, {}, {})",
                blitter, beg_y, beg_x, len_y, len_x
            )
        ]
    }

    /// Like [`from_rgba`][NcVisual#method.from_rgba], but the pixels are
    /// 4-byte RGBX. Alpha is filled in throughout using 'alpha'.
    ///
    /// `rowstride` must be a multiple of 4.
    ///
    /// *C style function: [ncvisual_from_rgb_loose()][c_api::ncvisual_from_rgb_loose].*
    pub fn from_rgb_loose<'a>(
        rgb: &[u8],
        rows: NcDim,
        rowstride: NcDim,
        cols: NcDim,
        alpha: NcComponent,
    ) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe {
                c_api::ncvisual_from_rgb_loose(
                    rgb.as_ptr() as *const c_void,
                    rows as i32,
                    rowstride as i32,
                    cols as i32,
                    alpha as i32,
                )
            },
            &format!(
                "NcVisual::from_rgb_loose(rgba, {}, {}, {}, {})",
                rows, rowstride, cols, alpha
            )
        ]
    }

    /// Like [`from_rgba`][NcVisual#method.from_rgba], but the pixels are
    /// 3-byte RGB. Alpha is filled in throughout using 'alpha'.
    ///
    /// *C style function: [ncvisual_from_rgb_packed()][c_api::ncvisual_from_rgb_packed].*
    pub fn from_rgb_packed<'a>(
        rgb: &[u8],
        rows: NcDim,
        rowstride: NcDim,
        cols: NcDim,
        alpha: NcComponent,
    ) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe {
                c_api::ncvisual_from_rgb_packed(
                    rgb.as_ptr() as *const c_void,
                    rows as i32,
                    rowstride as i32,
                    cols as i32,
                    alpha as i32,
                )
            },
            &format!(
                "NcVisual::from_rgb_packed(rgba, {}, {}, {}, {})",
                rows, rowstride, cols, alpha
            )
        ]
    }

    /// Prepares an NcVisual, and its underlying NcPlane, based off RGBA content
    /// in memory at `rgba`.
    ///
    /// `rgba` is laid out as `rows` lines, each of which is `rowstride` bytes in length.
    /// Each line has `cols` 32-bit 8bpc RGBA pixels followed by possible padding
    /// (there will be rowstride - cols * 4 bytes of padding).
    ///
    /// The total size of `rgba` is thus (rows * rowstride) bytes, of which
    /// (rows * cols * 4) bytes are actual non-padding data.
    ///
    /// *C style function: [ncvisual_from_rgba()][c_api::ncvisual_from_rgba].*
    pub fn from_rgba<'a>(
        rgba: &[u8],
        rows: NcDim,
        rowstride: NcDim,
        cols: NcDim,
    ) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe {
                c_api::ncvisual_from_rgba(
                    rgba.as_ptr() as *const c_void,
                    rows as i32,
                    rowstride as i32,
                    cols as i32,
                )
            },
            &format!(
                "NcVisual::from_rgba(rgba, {}, {}, {})",
                rows, rowstride, cols
            )
        ]
    }

    /// Like [`from_rgba`][NcVisual#method.from_rgba], but `data` is
    /// `pstride`-byte palette-indexed pixels, arranged in `rows` lines of
    /// `rowstride` bytes each, composed of `cols` pixels.
    ///
    /// `palette` is an array of at least `palsize` [`NcChannel`]s.
    ///
    /// *C style function: [ncvisual_from_palidx()][c_api::ncvisual_from_palidx].*
    //
    // API ALLOC struct ncvisual* ncvisual_from_palidx(const void* data, int rows,
    // int rowstride, int cols, int palsize, int pstride, const uint32_t* palette)
    // pub fn ncvisual_from_palidx(
    //     data: *const cty::c_void,
    //     rows: cty::c_int,
    //     rowstride: cty::c_int,
    //     cols: cty::c_int,
    //     palsize: cty::c_int,
    //     pstride: cty::c_int,
    //     palette: *const u32,
    // ) -> *mut ncvisual;
    pub fn from_palidx<'a>(
        data: &[u8],
        rows: NcDim,
        rowstride: NcDim,
        cols: NcDim,
        palsize: u8,
        pstride: NcDim,
        palette: &[NcChannel],
    ) -> NcResult<&'a mut NcVisual> {
        // assert![];
        error_ref_mut![
            unsafe {
                c_api::ncvisual_from_palidx(
                    data.as_ptr() as *const c_void,
                    rows as i32,
                    rowstride as i32,
                    cols as i32,
                    palsize as i32,
                    pstride as i32,
                    palette.as_ptr() as *const NcChannel,
                )
            },
            &format!(
                "NcVisual::from_palidx(data, {}, {}, {}, {}, {}, palette)",
                rows, rowstride, cols, palsize, pstride
            )
        ]
    }

    /// Destroys this NcVisual.
    ///
    /// Rendered elements will not be disrupted, but the visual can be neither
    /// decoded nor rendered any further.
    ///
    /// *C style function: [ncvisual_destroy()][c_api::ncvisual_destroy].*
    pub fn destroy(&mut self) {
        unsafe { c_api::ncvisual_destroy(self) }
    }
}

/// # NcVisual Methods
impl NcVisual {
    /// Gets the specified pixel from this NcVisual.
    ///
    /// *C style function: [ncvisual_at_yx()][c_api::ncvisual_at_yx].*
    pub fn at_yx(&self, y: NcDim, x: NcDim) -> NcResult<NcPixel> {
        let mut pixel = 0;
        let res = unsafe { c_api::ncvisual_at_yx(self, y as i32, x as i32, &mut pixel) };
        error![res, "NcVisual.at_yx()", pixel]
    }

    /// Extracts the next frame from the NcVisual.
    ///
    /// Returns 0 for normal frames, and 1 to indicate EOF.
    ///
    /// *C style function: [ncvisual_decode()][c_api::ncvisual_decode].*
    pub fn decode(&mut self) -> NcResult<NcIntResult> {
        let res = unsafe { c_api::ncvisual_decode(self) };
        if res == NcIntResult::ERR {
            Err(NcError::with_msg(res, "NcVisual.decode()"))
        } else {
            Ok(res)
        }
    }

    /// Extracts the next frame from the NcVisual, ala [decode][NcVisual#method.decode],
    /// but if we have reached the end, rewinds to the first frame.
    ///
    /// *A subsequent [NcVisual.render]() will render the first frame,
    /// as if the ncvisual had been closed and reopened.*
    ///
    /// Returns 0 for normal frames and 1 to indicate EOF.
    ///
    /// *C style function: [ncvisual_decode_loop()][c_api::ncvisual_decode_loop].*
    pub fn decode_loop(&mut self) -> NcResult<NcIntResult> {
        let res = unsafe { c_api::ncvisual_decode_loop(self) };
        if res == NcIntResult::ERR {
            Err(NcError::with_msg(res, "NcVisual.decode_loop()"))
        } else {
            Ok(res)
        }
    }

    /// An all-purpose `NcVisual` geometry solver, returns [`NcVGeom`].
    ///
    /// if `nc` is `None`, only `pixy`/`pixx` will be filled in, with the true
    /// pixel geometry of the current `NcVisual`.
    ///
    /// `cdimy`/`cdimx` and `maxpixely`/`maxpixelx` are only ever filled in if
    /// we know them.
    ///
    /// See also: [`Nc.visual_geom`][Nc#method.visual_geom]
    ///
    /// *C style function: [ncvisual_geom()][c_api::ncvisual_geom].*
    pub fn geom(&self, vopts: &NcVisualOptions, nc: Option<&Nc>) -> NcResult<NcVGeom> {
        let mut vgeom = NcVGeom::new();

        let nc_ptr: *const Nc;
        if let Some(nc) = nc {
            nc_ptr = nc;
        } else {
            nc_ptr = null();
        }

        let res = unsafe { crate::c_api::ncvisual_geom(nc_ptr, self, vopts, &mut vgeom) };
        error![res, "NcVisual.geom()", vgeom];
    }

    /// Gets the size and ratio of NcVisual pixels to output cells along the
    /// vertical and horizontal axes.
    ///
    /// Returns `NcBlitterGeometry`.
    ///
    /// An NcVisual of `y` by `x` pixels will require
    /// (`y` * `to_y`) by (`x` * `to_x`) cells for full output.
    ///
    /// Errors on invalid blitter in `options`. Scaling is taken into consideration.
    ///
    /// *C style function: [ncvisual_blitter_geom()][c_api::ncvisual_blitter_geom].*
    #[deprecated]
    #[allow(deprecated)]
    pub fn blitter_geom(
        &self,
        nc: &Nc,
        options: &NcVisualOptions,
    ) -> NcResult<crate::blitter::geometry::NcBlitterGeometry> {
        let mut y = 0;
        let mut x = 0;
        let mut scale_y = 0;
        let mut scale_x = 0;
        let mut blitter = 0;

        let res = unsafe {
            c_api::ncvisual_blitter_geom(
                nc,
                self,
                options,
                &mut y,
                &mut x,
                &mut scale_y,
                &mut scale_x,
                &mut blitter,
            )
        };
        #[allow(deprecated)]
        let bgeom = crate::blitter::geometry::NcBlitterGeometry {
            x: x as NcDim,
            y: y as NcDim,
            scale_y: scale_y as NcDim,
            scale_x: scale_x as NcDim,
            blitter,
        };
        error![res, "NcVisual.blitter_geom()", bgeom];
    }

    /// Gets the default media (not plot) blitter for this environment when using
    /// the specified scaling method.
    ///
    /// Currently, this means:
    /// - if lacking UTF-8, `NcBlitter::BLIT_1x1`.
    /// - otherwise, if not [`NcScale::STRETCH`][NcScale#associatedconstant.STRETCH], `BLIT_2x1`.
    /// - otherwise, if sextants are not known to be good, `NcBlitter::BLIT_2x2`.
    /// - otherwise `NCBLIT_3x2`, `NCBLIT_2x2` and `NcBlitter::BLIT_3x2` both
    ///   distort the original aspect ratio, thus `NCBLIT_2x1` is used
    ///   outside of [`NcScale::STRETCH`][NcScale#associatedconstant.STRETCH].
    ///
    /// *C style function: [ncvisual_media_defblitter()][c_api::ncvisual_media_defblitter].*
    pub fn media_defblitter(nc: &Nc, scale: NcScale) -> NcBlitter {
        unsafe { c_api::ncvisual_media_defblitter(nc, scale) }
    }

    /// Polyfills at the specified location using `rgba`.
    ///
    /// *C style function: [ncvisual_polyfill_yx()][c_api::ncvisual_polyfill_yx].*
    pub fn polyfill_yx(&mut self, y: NcDim, x: NcDim, rgba: NcRgba) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_polyfill_yx(self, y as i32, x as i32, rgba) },
            &format!["NcVisual.polyfill_yx({}, {}, {})", y, x, rgba]
        ]
    }

    /// Renders the decoded frame to the [`NcPlane`] specified in `options`.
    ///
    /// If a plane is not provided, it will be created, having the exact size
    /// necessary to display the visual.
    ///
    /// See [`NcVisualOptions`].
    ///
    /// *C style function: [ncvisual_render()][c_api::ncvisual_render].*
    #[deprecated]
    pub fn render(&mut self, nc: &mut Nc, options: &NcVisualOptions) -> NcResult<&mut NcPlane> {
        error_ref_mut![
            unsafe { c_api::ncvisual_render(nc, self, options) },
            "NcVisual.render(Nc, &NcVisualOptions)"
        ]
    }

    /// Renders the decoded frame according to the provided `options`.
    ///
    /// There are 3 options for choosing the the plane used for rendering:
    /// 1. if the `options` have set the flag
    /// [`NcVisualOptions::CHILDPLANE`][NcVisualOptions#associatedconstant.CHILDPLANE]
    /// then there must be a plane, which will be the father of the one created.
    /// 2. if the flag is not set and there is no plane, a new plane is created
    ///    as root of a new pile.
    /// 3. if the flag is not set and there is a plane, we render to it.
    ///
    /// A subregion of the visual can be rendered using `beg_y`, `beg_x`,
    /// `len_y`, and `len_x`.
    ///
    /// It is an error to specify any region beyond the boundaries of the frame.
    ///
    /// Returns the (possibly newly-created) plane to which we drew.
    ///
    /// Pixels may not be blitted to the standard plane.
    ///
    /// *C style function: [ncvisual_blit()][c_api::ncvisual_blit].*
    pub fn blit(
        &mut self,
        nc: &mut Nc,
        options: Option<&NcVisualOptions>,
    ) -> NcResult<&mut NcPlane> {
        let options_ptr = if let Some(o) = options { o } else { null() };
        error_ref_mut![
            unsafe { c_api::ncvisual_blit(nc, self, options_ptr) },
            "NcVisual.blit"
        ]
    }

    /// Resizes the visual to `cols` X `rows` pixels.
    ///
    /// This is a lossy transformation, unless the size is unchanged.
    ///
    /// *C style function: [ncvisual_resize()][c_api::ncvisual_resize].*
    pub fn resize(&mut self, rows: NcDim, cols: NcDim) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_resize(self, rows as i32, cols as i32) },
            &format!["NcVisual.resize({}, {})", rows, cols]
        ]
    }

    /// Resizes the visual to  in the image to `rows` X `cols` pixels, without
    /// interpolating the color values.
    ///
    /// The original color is retained.
    ///
    /// *C style function:
    /// [ncvisual_resize_noninterpolative()][c_api::ncvisual_resize_noninterpolative].*
    pub fn resize_noninterpolative(&mut self, rows: NcDim, cols: NcDim) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_resize_noninterpolative(self, rows as i32, cols as i32) },
            &format!["NcVisual.resize_noninterpolative({}, {})", cols, rows]
        ]
    }

    /// Rotates the visual `rads` radians.
    ///
    /// Only M_PI/2 and -M_PI/2 are supported at the moment,
    /// but this will change. (FIXME)
    ///
    /// *C style function: [ncvisual_rotate()][c_api::ncvisual_rotate].*
    pub fn rotate(&mut self, rads: f64) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_rotate(self, rads) },
            &format!["NcVisual.rotate({})", rads]
        ]
    }

    /// Sets the specified pixel.
    ///
    /// *C style function: [ncvisual_set_yx()][c_api::ncvisual_set_yx].*
    pub fn set_yx(&mut self, y: NcDim, x: NcDim, pixel: NcPixel) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_set_yx(self, y as i32, x as i32, pixel) },
            &format!["NcVisual.set_yx({}, {}, {})", y, x, pixel]
        ]
    }

    /// Displays frames.
    ///
    /// *Provide as an argument to ncvisual_stream().*
    ///
    /// If you'd like subtitles to be decoded, provide an ncplane as the curry.
    /// If the curry is NULL, subtitles will not be displayed.
    ///
    /// *C style function: [ncvisual_simple_streamer()][c_api::ncvisual_simple_streamer].*
    pub fn simple_streamer(
        &mut self,
        options: &mut NcVisualOptions,
        time: &NcTime,
        curry: Option<&mut NcPlane>,
    ) -> NcResult<()> {
        if let Some(plane) = curry {
            error![
                unsafe {
                    c_api::ncvisual_simple_streamer(
                        self,
                        options,
                        time,
                        plane as *mut _ as *mut libc::c_void,
                    )
                },
                &format![
                    "NcVisual.simple_streamer({:?}, {:?}, ncplane)",
                    options, time
                ]
            ]
        } else {
            error![
                unsafe { c_api::ncvisual_simple_streamer(self, options, time, null_mut()) },
                &format!["NcVisual.simple_streamer({:?}, {:?}, null)", options, time]
            ]
        }
    }

    // // TODO
    //
    // /// Streams the entirety of the media, according to its own timing.
    // ///
    // /// Blocking, obviously.
    // ///
    // /// If `streamer` is provided it will be called for each frame, and its return
    // /// value handled as outlined for streamcb.
    // /// If streamer() returns non-zero, the stream is aborted, and that value is
    // /// returned.  By convention, return a positive number to indicate intentional
    // /// abort from within streamer().
    // ///
    // /// `timescale` allows the frame duration time to be scaled.
    // /// For a visual naturally running at 30FPS, a 'timescale' of 0.1 will result
    // /// in 300 FPS, and a `timescale` of 10 will result in 3 FPS.
    // /// It is an error to supply `timescale` less than or equal to 0.
    // ///
    // /// *C style function: [ncvisual_streamer()][c_api::ncvisual_streamer].*
    // //
    // // TODO: add streamcb
    // // INFO: QUESTION: is curry also optional like in simple_streamer?
    // //
    // pub fn simple_streamer(
    //     &mut self,
    //     nc: &mut Nc,
    //     timescale: f32,
    //     //streamer: Option<streamcb>
    //     options: &NcVisualOptions,
    //     curry: Option<&mut NcPlane>,
    // ) -> NcResult<()> {
    // }

    /// If a subtitle ought be displayed at this time, returns a heap-allocated
    /// copy of the UTF8 text.
    ///
    /// *C style function: [ncvisual_subtitle()][c_api::ncvisual_subtitle].*
    #[deprecated]
    pub fn subtitle(&self) -> NcResult<String> {
        let res = unsafe { c_api::ncvisual_subtitle(self) };
        if !res.is_null() {
            Ok(rstring_free![res])
        } else {
            Err(NcError::with_msg(NcIntResult::ERR, "NcVisual.subtitle()"))
        }
    }

    /// If a subtitle ought be displayed at this time, return a new plane
    ///
    /// The returned plane is bound to `parent` and contains the subtitle,
    /// which might be text or graphics (depending on the input format).
    ///
    /// *C style function: [ncvisual_subtitle_plane()][c_api::ncvisual_subtitle_plane].*
    pub fn subtitle_plane(&self, parent: &mut NcPlane) -> NcResult<&mut NcPlane> {
        error_ref_mut![unsafe { c_api::ncvisual_subtitle_plane(parent, self) }]
    }
}

/// # `NcDirectF` Constructors & destructors
impl NcVisual {
    /// Loads media from disk, but do not yet renders it (presumably because you
    /// want to get its geometry via [ncdirectf_geom()][0], or to use the same
    /// file with [ncdirectf_render()][1] multiple times).
    ///
    /// You must destroy the result with [ncdirectf_free()][2];
    ///
    /// [0]: NcVisual#method.ncdirectf_geom
    /// [1]: NcVisual#method.ncdirectf_render
    /// [2]: NcVisual#method.ncdirectf_free
    ///
    /// *C style function: [ncdirectf_from_file()][c_api::ncdirectf_from_file].*
    pub fn ncdirectf_from_file<'a>(ncd: &mut NcDirect, file: &str) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe { c_api::ncdirectf_from_file(ncd, cstring![file]) },
            &format!("NcVisual::ncdirectf_from_file(ncd, {})", file)
        ]
    }

    /// Frees a [`NcVisual`] returned from [ncdirectf_from_file()][0].
    ///
    /// [0]: NcVisual#method.ncdirectf_from_file
    ///
    /// *C style function: [ncdirectf_free()][c_api::ncdirectf_free].*
    pub fn ncdirectf_free(&mut self) {
        unsafe { c_api::ncdirectf_free(self) };
    }
}

/// # `NcDirectF` Methods
impl NcVisual {
    /// Same as [`NcDirect.render_frame()`][0], except `frame` must already have
    /// been loaded.
    ///
    /// A loaded frame may be rendered in different ways before it is destroyed.
    ///
    /// [0]: NcDirect#method.render_frame
    ///
    /// *C style function: [ncvisual_render()][c_api::ncvisual_render].*
    pub fn ncdirectf_render(
        &mut self,
        ncd: &mut NcDirect,
        options: &NcVisualOptions,
    ) -> NcResult<&mut NcPlane> {
        error_ref_mut![
            unsafe { c_api::ncdirectf_render(ncd, self, options) },
            "NcVisual.render()"
        ]
    }
    /// Having loaded the `frame`, get the geometry of a potential render.
    ///
    /// *C style function: [ncdirectf_geom()][c_api::ncdirectf_geom].*
    pub fn ncdirectf_geom(
        &mut self,
        ncd: &mut NcDirect,
        options: &NcVisualOptions,
    ) -> NcResult<NcVGeom> {
        let mut geom = NcVGeom::new();

        let res = unsafe { c_api::ncdirectf_geom(ncd, self, options, &mut geom) };
        error![res, "NcVisual.ncdirectf_geom()", geom];
    }
}

/// # NcVGeom Constructors
impl NcVGeom {
    /// Returns a new `NcVGeom` with zeroed fields.
    pub fn new() -> Self {
        Self {
            pixy: 0,
            pixx: 0,
            cdimy: 0,
            cdimx: 0,
            rpixy: 0,
            rpixx: 0,
            rcelly: 0,
            rcellx: 0,
            scaley: 0,
            scalex: 0,
            maxpixely: 0,
            maxpixelx: 0,
            begy: 0,
            begx: 0,
            leny: 0,
            lenx: 0,
            blitter: NcBlitter::DEFAULT,
        }
    }
}
