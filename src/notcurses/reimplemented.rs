//! `notcurses_*` reimplemented functions.

use core::ptr::{null, null_mut};

use crate::{
    c_api, Nc, NcAlign, NcDim, NcError, NcInput, NcIntResult, NcIntResultApi, NcPlane, NcResult,
    NcTime,
};

/// Returns the offset into `avail_u` at which `u` ought be output given
/// the requirements of `align`.
///
/// Returns `-`[`NcIntResult::MAX`][NcIntResult#associatedconstant.MAX] if
/// [NCALIGN_UNALIGNED][c_api::NCALIGN_UNALIGNED] or invalid `align`.
///
/// *Method: Nc.[align()][Nc#method.align].*
#[inline]
pub fn notcurses_align(avail_u: NcDim, align: NcAlign, u: NcDim) -> NcIntResult {
    if align == NcAlign::Left || align == NcAlign::Top {
        return 0;
    }
    if align == NcAlign::Center {
        return ((avail_u - u) / 2) as NcIntResult;
    }
    if align == NcAlign::Right || align == NcAlign::Bottom {
        return (avail_u - u) as NcIntResult;
    }
    -NcIntResult::MAX
}

/// Returns true if we can blit pixel-accurate bitmaps.
///
/// *Method: Nc.[canpixel()][Nc#method.canpixel].*
#[inline]
pub fn notcurses_canpixel(nc: &Nc) -> bool {
    unsafe { c_api::notcurses_check_pixel_support(nc) != c_api::NCPIXEL_NONE }
}

/// Returns true if we can reliably use Unicode Braille.
///
/// *Method: Nc.[canbraille()][Nc#method.canbraille].*
#[inline]
pub fn notcurses_canbraille(nc: &Nc) -> bool {
    notcurses_canutf8(nc) && nc.capabilities().braille
}

/// Returns true if it's possible to set the "hardware" palette.
///
/// Requires the "ccc" terminfo capability.
///
/// *Method: Nc.[canchangecolor()][Nc#method.canchangecolor].*
#[inline]
pub fn notcurses_canchangecolor(nc: &Nc) -> bool {
    c_api::nccapability_canchangecolor(&nc.capabilities())
}

/// Returns true if fading is possible.
///
/// Fading requires either the "rgb" or "ccc" terminfo capability.
///
/// *Method: Nc.[canfade()][Nc#method.canfade].*
#[inline]
pub fn notcurses_canfade(nc: &Nc) -> bool {
    notcurses_canchangecolor(nc) || notcurses_cantruecolor(nc)
}

/// Returns true if it's possible to directly specify RGB values per cell,
/// or false if it's only possible to use palettes.
///
/// *Method: Nc.[cantruecolor()][Nc#method.cantruecolor].*
pub fn notcurses_cantruecolor(nc: &Nc) -> bool {
    nc.capabilities().rgb
}

/// Returns true if the encoding is UTF-8.
///
/// Requires `LANG` being set to a UTF-8 locale.
///
/// *Method: Nc.[canutf8()][Nc#method.canutf8].*
#[inline]
pub fn notcurses_canutf8(nc: &Nc) -> bool {
    nc.capabilities().utf8
}

/// Returns true if we can reliably use Unicode half blocks.
///
/// *Method: Nc.[canhalfblock()][Nc#method.canhalfblock].*
#[inline]
pub fn notcurses_canhalfblock(nc: &Nc) -> bool {
    notcurses_canutf8(nc)
}

/// Returns true if we can reliably use Unicode quadrant blocks.
///
/// *Method: Nc.[canquadrant()][Nc#method.canquadrant].*
#[inline]
pub fn notcurses_canquadrant(nc: &Nc) -> bool {
    notcurses_canutf8(nc) && nc.capabilities().quadrants
}

/// Returns true if we can reliably use Unicode 13 sextants.
///
/// *Method: Nc.[cansextant()][Nc#method.cansextant].*
#[inline]
pub fn notcurses_cansextant(nc: &Nc) -> bool {
    notcurses_canutf8(nc) && nc.capabilities().sextants
}

/// Reads input blocking until an event is processed or a signal is received
/// (including resize events)
///
/// Will optionally write the event details in `input`.
///
/// In case of an invalid read (including on EOF) *-1* is returned.
///
/// *Method: Nc.[get_blocking()][Nc#method.get_blocking].*
#[inline]
pub fn notcurses_get_blocking(nc: &mut Nc, input: Option<&mut NcInput>) -> NcIntResult {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe { c_api::notcurses_get(nc, null(), input_ptr) as NcIntResult }
}

/// Reads input without blocking.
///
/// Will optionally write the event details in `input`.
///
/// If no event is immediately ready, returns 0.
///
/// In case of an invalid read (including on EOF) *-1* is returned.
///
/// *Method: Nc.[get_nblock()][Nc#method.get_nblock].*
#[inline]
pub fn notcurses_get_nblock(nc: &mut Nc, input: Option<&mut NcInput>) -> NcIntResult {
    let input_ptr;
    if let Some(i) = input {
        input_ptr = i as *mut _;
    } else {
        input_ptr = null_mut();
    }
    unsafe {
        let ts = NcTime::new(0, 0);
        c_api::notcurses_get(nc, &ts, input_ptr) as NcIntResult
    }
}

/// Renders and rasterizes the standard pile in one shot. Blocking call.
///
/// *Method: Nc.[render()][Nc#method.render].*
#[inline]
pub fn notcurses_render(nc: &mut Nc) -> NcIntResult {
    let stdplane = unsafe { c_api::notcurses_stdplane(nc) };
    if unsafe { c_api::ncpile_render(stdplane) } == NcIntResult::ERR {
        return NcIntResult::ERR;
    }
    unsafe { c_api::ncpile_rasterize(stdplane) }
}

/// [*notcurses_stdplane*][c_api::notcurses_stdplane], plus free bonus
/// dimensions written to non-NULL y/x!
///
/// *Method: Nc.[stddim_yx()][Nc#method.stddim_yx].*
#[inline]
pub fn notcurses_stddim_yx<'a>(
    nc: &'a mut Nc,
    y: &mut NcDim,
    x: &mut NcDim,
) -> NcResult<&'a mut NcPlane> {
    unsafe {
        let sp = c_api::notcurses_stdplane(nc);
        if !sp.is_null() {
            c_api::ncplane_dim_yx(sp, y, x);
            return Ok(&mut *sp);
        }
    }
    Err(NcError::new())
}

/// [*notcurses_stdplane_const*][c_api::notcurses_stdplane_const], plus free
/// bonus dimensions written to non-NULL y/x!
///
/// *Method: Nc.[stddim_yx_const()][Nc#method.stddim_yx_const].*
#[inline]
pub fn notcurses_stddim_yx_const<'a>(
    nc: &'a Nc,
    y: &mut NcDim,
    x: &mut NcDim,
) -> NcResult<&'a NcPlane> {
    unsafe {
        let sp = c_api::notcurses_stdplane_const(nc);
        if !sp.is_null() {
            c_api::ncplane_dim_yx(sp, y, x);
            return Ok(&*sp);
        }
    }
    Err(NcError::new())
}

/// Returns our current idea of the terminal dimensions in rows and cols.
///
/// *Method: Nc.[term_dim_yx()][Nc#method.term_dim_yx].*
#[inline]
pub fn notcurses_term_dim_yx(nc: &Nc) -> (NcDim, NcDim) {
    let (mut y, mut x) = (0, 0);
    unsafe {
        c_api::ncplane_dim_yx(c_api::notcurses_stdplane_const(nc), &mut y, &mut x);
    }
    (y as NcDim, x as NcDim)
}

/// Disables all mice tracking.
#[inline]
pub fn notcurses_mice_disable(nc: &mut Nc) -> NcIntResult {
    unsafe { c_api::notcurses_mice_enable(nc, c_api::NCMICE_NO_EVENTS) }
}

/// Returns the bottommost [`NcPlane`] on the standard pile,
/// of which there is always at least one.
///
/// *Method: Nc.[bottom()][Nc#method.bottom].*
#[inline]
pub fn notcurses_bottom(nc: &mut Nc) -> &mut NcPlane {
    unsafe { &mut *c_api::ncpile_bottom(c_api::notcurses_stdplane(nc)) }
}

/// Returns the topmost [`NcPlane`] on the standard pile,
/// of which there is always at least one.
///
/// *Method: Nc.[top()][Nc#method.top].*
#[inline]
pub fn notcurses_top(nc: &mut Nc) -> &mut NcPlane {
    unsafe { &mut *c_api::ncpile_top(c_api::notcurses_stdplane(nc)) }
}
