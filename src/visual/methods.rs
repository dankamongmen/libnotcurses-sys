//! `NcVisual*` methods and associated functions.

use core::ptr::{null, null_mut};
use libc::c_void;

use crate::{
    c_api::{self, NcChannel_u32, NcResult_i32, NCRESULT_ERR},
    cstring, error, error_ref_mut, Nc, NcBlitter, NcChannel, NcDirect, NcError, NcPixel, NcPlane,
    NcResult, NcRgba, NcScale, NcTime, NcVisual, NcVisualGeometry, NcVisualOptions,
};

/// # NcVisual Constructors & destructors
impl NcVisual {
    /// Like [from_rgba][NcVisual#method.from_rgba], but 'bgra' is arranged as BGRA.
    ///
    /// *C style function: [ncvisual_from_bgra()][c_api::ncvisual_from_bgra].*
    pub fn from_bgra<'a>(
        bgra: &[u8],
        rows: u32,
        rowstride: u32,
        cols: u32,
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

    /// Opens an `NcVisual` at `file`, extracts the codec and parameters and
    /// decodes the first image to memory.
    ///
    /// *C style function: [ncvisual_from_file()][c_api::ncvisual_from_file].*
    pub fn from_file<'a>(file: &str) -> NcResult<&'a mut NcVisual> {
        let cs = cstring![file];
        error_ref_mut![
            unsafe { c_api::ncvisual_from_file(cs.as_ptr()) },
            &format!("NcVisual::from_file({})", file)
        ]
    }

    /// Promotes an `NcPlane` to an `NcVisual`.
    ///
    /// The plane may contain only spaces, half blocks, and full blocks.
    /// This will be checked, and any other glyph will result in an error.
    ///
    /// This function exists so that planes can be subjected to `NcVisual`
    /// transformations.
    ///
    /// If possible, it's better to create the `NcVisual` from memory using
    /// [`from_rgba`][NcVisual#method.from_rgba].
    ///
    /// Use `None` for either or both of `beg_y` and `beg_x` in order to
    /// use the current cursor position along that axis.
    ///
    /// Use `None` for either or both of `len_y` and `len_x` in order to
    /// go through the boundary of the plane in that axis (same as `0`).
    ///
    /// *C style function: [ncvisual_from_plane()][c_api::ncvisual_from_plane].*
    pub fn from_plane<'a>(
        plane: &NcPlane,
        blitter: impl Into<NcBlitter> + Copy,
        beg_y: Option<u32>,
        beg_x: Option<u32>,
        len_y: Option<u32>,
        len_x: Option<u32>,
    ) -> NcResult<&'a mut NcVisual> {
        error_ref_mut![
            unsafe {
                c_api::ncvisual_from_plane(
                    plane,
                    blitter.into().into(),
                    beg_y.unwrap_or(u32::MAX) as i32,
                    beg_x.unwrap_or(u32::MAX) as i32,
                    len_y.unwrap_or(0),
                    len_x.unwrap_or(0),
                )
            },
            &format!(
                "NcVisual::from_file(plane, {}, {:?}, {:?}, {:?}, {:?})",
                blitter.into(),
                beg_y,
                beg_x,
                len_y,
                len_x
            )
        ]
    }

    /// Constructs an `NcVisual` from a nul-terminated Sixel control `sequence`.
    ///
    /// *C style function: [ncvisual_from_sixel()][c_api::ncvisual_from_sixel].*
    pub fn from_sixel<'a>(sequence: &str, len_y: u32, len_x: u32) -> NcResult<&'a mut NcVisual> {
        let cs = cstring![sequence];
        error_ref_mut![
            unsafe { c_api::ncvisual_from_file(cs.as_ptr()) },
            &format!("NcVisual::from_sixel({}, {}, {})", sequence, len_y, len_x)
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
        rows: u32,
        rowstride: u32,
        cols: u32,
        alpha: u8,
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
        rows: u32,
        rowstride: u32,
        cols: u32,
        alpha: u8,
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

    /// Prepares an `NcVisual`, and its underlying `NcPlane`,
    /// based off RGBA content in memory at `rgba`.
    ///
    /// `rgba` is laid out as `rows` lines, each of which is `rowstride` bytes
    /// in length. Each line has `cols` 32-bit 8bpc RGBA pixels followed by
    /// possible padding (there will be rowstride - cols * 4 bytes of padding).
    ///
    /// The total size of `rgba` is thus (rows * rowstride) bytes, of which
    /// (rows * cols * 4) bytes are actual non-padding data.
    ///
    /// *C style function: [ncvisual_from_rgba()][c_api::ncvisual_from_rgba].*
    pub fn from_rgba<'a>(
        rgba: &[u8],
        rows: u32,
        rowstride: u32,
        cols: u32,
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
        rows: u32,
        rowstride: u32,
        cols: u32,
        palsize: u8,
        pstride: u32,
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
                    palette.as_ptr() as *const NcChannel_u32,
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
    /// Extracts the next frame from the NcVisual.
    ///
    /// Returns 0 for normal frames, and 1 to indicate EOF.
    ///
    /// *C style function: [ncvisual_decode()][c_api::ncvisual_decode].*
    pub fn decode(&mut self) -> NcResult<NcResult_i32> {
        let res = unsafe { c_api::ncvisual_decode(self) };
        if res == NCRESULT_ERR {
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
    pub fn decode_loop(&mut self) -> NcResult<NcResult_i32> {
        let res = unsafe { c_api::ncvisual_decode_loop(self) };
        if res == NCRESULT_ERR {
            Err(NcError::with_msg(res, "NcVisual.decode_loop()"))
        } else {
            Ok(res)
        }
    }

    /// Returns [`NcVisualGeometry`].
    ///
    /// if [`Nc`] is not provided, only [`pix_yx`] will be filled in, with the
    /// true pixel geometry of the current `NcVisual`.
    ///
    /// Additionally [`cdim_yx`] and [`maxpixel_yx`] are only ever filled in if we
    /// know them, and `maxpixel_yx` is only defined for `NcBlitter`::PIXEL.
    ///
    /// # See also
    /// - [`Nc.visual_geom`][Nc#method.visual_geom]
    ///
    /// [`pix_yx`]: NcVisualGeometry#structfield.pix_yx
    /// [`cdim_yx`]: NcVisualGeometry#structfield.cdim_yx
    /// [`scale_yx`]: NcVisualGeometry#structfield.scale_yx
    /// [`maxpixel_yx`]: NcVisualGeometry#structfield.maxpixel_yx
    /// [`blitter`]: NcVisualGeometry#structfield.blitter
    ///
    /// *C style function: [ncvisual_geom()][c_api::ncvisual_geom].*
    pub fn geom(
        &self,
        nc: Option<&Nc>,
        vopts: Option<&NcVisualOptions>,
    ) -> NcResult<NcVisualGeometry> {
        let mut vg = c_api::NcVGeom::new();

        let nc_ptr: *const Nc = if let Some(nc) = nc { nc } else { null() };
        let vo_ptr: *const NcVisualOptions =
            if let Some(o) = vopts { o } else { &NcVisualOptions::default() };

        let res = unsafe { crate::c_api::ncvisual_geom(nc_ptr, self, vo_ptr, &mut vg) };
        if res <= c_api::NCRESULT_ERR {
            return Err(NcError::with_msg(
                res,
                &format!["NcVisual.geom({:?}, {:?})", vopts, nc],
            ));
        }

        let (pix_yx, cdim_yx, rpix_yx, rcell_yx, scale_yx, maxpixel_yx, beg_yx, len_yx);

        // if an `Nc` context is not provided, only `pix_yx` will be filled in.
        if nc.is_none() {
            pix_yx = Some((vg.pixy as u32, vg.pixx as u32));

            cdim_yx = None;
            rpix_yx = None;
            rcell_yx = None;
            scale_yx = None;
            maxpixel_yx = None;
            beg_yx = None;
            len_yx = None;
        } else {
            // `maxpixel_yx` only is defined for `Ncblitter::PIXEL`.
            if vg.blitter == NcBlitter::Pixel.into() {
                maxpixel_yx = Some((vg.maxpixely as u32, vg.maxpixelx as u32));
            } else {
                maxpixel_yx = None;
            }

            // `beg_yx` & `len_yx` can be safely ignored if they're both all 0.
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

    /// Gets the default media (not plot) blitter for this environment when using
    /// the specified scaling method.
    ///
    /// Currently, this means:
    /// - if lacking UTF-8, [`NcBlitter::Ascii`].
    /// - otherwise, if not using *[`NcScale::Stretch`]* then [`NcBlitter::Half`].
    /// - otherwise, if sextants are not known to be good, [`NcBlitter::Quadrant`].
    /// - otherwise [`NcBlitter::Sextant`]
    ///
    /// [`QUADRANT`] and [`SEXTANT`] both distort the original aspect ratio,
    /// thus they are only used alongside *[`NcScale::Stretch`]*, while [`Half`]
    /// is used otherwise.
    ///
    /// *C style function: [ncvisual_media_defblitter()][c_api::ncvisual_media_defblitter].*
    ///
    /// [`Half`]: NcBlitter::Half
    /// [`Quadrant`]: NcBlitter::Quadrant
    /// [`Sextant`]: NcBlitter::Sextant
    pub fn media_defblitter(nc: &Nc, scale: impl Into<NcScale>) -> NcBlitter {
        unsafe { c_api::ncvisual_media_defblitter(nc, scale.into().into()).into() }
    }

    /// Polyfills at the specified location using `rgba`.
    ///
    /// *C style function: [ncvisual_polyfill_yx()][c_api::ncvisual_polyfill_yx].*
    pub fn polyfill_yx(&mut self, y: u32, x: u32, rgba: impl Into<NcRgba>) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_polyfill_yx(self, y, x, rgba.into().into()) },
            &format!["NcVisual.polyfill_yx({}, {}, rgba)", y, x]
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
    /// # Safety
    /// You must be careful not to end up with multiple exclusive references
    /// to the returned `NcPlane`, or with one exclusive reference
    /// and one or more shared references.
    ///
    /// *C style function: [ncvisual_blit()][c_api::ncvisual_blit].*
    pub unsafe fn blit(
        &mut self,
        nc: &mut Nc,
        options: Option<&NcVisualOptions>,
    ) -> NcResult<&mut NcPlane> {
        let options_ptr = if let Some(o) = options { o } else { null() };
        error_ref_mut![c_api::ncvisual_blit(nc, self, options_ptr), "NcVisual.blit"]
    }

    /// Resizes the visual to `cols` X `rows` pixels.
    ///
    /// This is a lossy transformation, unless the size is unchanged.
    ///
    /// *C style function: [ncvisual_resize()][c_api::ncvisual_resize].*
    pub fn resize(&mut self, rows: u32, cols: u32) -> NcResult<()> {
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
    pub fn resize_noninterpolative(&mut self, rows: u32, cols: u32) -> NcResult<()> {
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

    /// Gets the specified pixel from this NcVisual.
    ///
    /// *C style function: [ncvisual_at_yx()][c_api::ncvisual_at_yx].*
    pub fn at_yx(&self, y: u32, x: u32) -> NcResult<NcPixel> {
        let mut pixel = 0;
        let res = unsafe { c_api::ncvisual_at_yx(self, y, x, &mut pixel) };
        error![res, "NcVisual.at_yx()", pixel.into()]
    }

    /// Sets the specified pixel.
    ///
    /// *C style function: [ncvisual_set_yx()][c_api::ncvisual_set_yx].*
    pub fn set_yx(&mut self, y: u32, x: u32, pixel: impl Into<NcPixel> + Copy) -> NcResult<()> {
        error![
            unsafe { c_api::ncvisual_set_yx(self, y, x, pixel.into().into()) },
            &format!["NcVisual.set_yx({}, {}, {:?})", y, x, pixel.into()]
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
    // /// For an NcVisual naturally running at 30FPS, a 'timescale' of 0.1
    // /// will result in 300 FPS, and a `timescale` of 10 will result in 3 FPS.
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
        let cs = cstring![file];
        error_ref_mut![
            unsafe { c_api::ncdirectf_from_file(ncd, cs.as_ptr()) },
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
    /// *C style function: [ncdirectf_render()][c_api::ncdirectf_render].*
    pub fn ncdirectf_render(
        &mut self,
        ncd: &mut NcDirect,
        options: &NcVisualOptions,
    ) -> NcResult<&mut NcPlane> {
        error_ref_mut![
            unsafe { c_api::ncdirectf_render(ncd, self, options) },
            "NcVisual.ncdirectf_render()"
        ]
    }
    /// Having loaded the `frame`, get the geometry of a potential render.
    ///
    /// *C style function: [ncdirectf_geom()][c_api::ncdirectf_geom].*
    pub fn ncdirectf_geom(
        &mut self,
        ncd: &mut NcDirect,
        options: &NcVisualOptions,
    ) -> NcResult<NcVisualGeometry> {
        let mut geom = c_api::NcVGeom::new();

        let res = unsafe { c_api::ncdirectf_geom(ncd, self, options, &mut geom) };
        error![res, "NcVisual.ncdirectf_geom()", geom.into()];
    }
}
