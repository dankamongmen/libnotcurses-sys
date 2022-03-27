//! `ncplane_*` reimplemented functions.

// TOC:
// - Alpha
// - NcChannel_u32
// - RGB components
// - NcRgb_u32
// - Default
// - put & print
// - movement, size & alignment
// - line
// - perimeter
// - box
// - gradient
// - cursor

use core::ptr::null_mut;

use std::ffi::CString;

use crate::{
    c_api::{
        self, nccell_release, NcAlign_u32, NcAlpha_u32, NcBoxMask_u32, NcChannel_u32,
        NcChannels_u64, NcResult_i32, NcRgb_u32, NcStyle_u16, NCRESULT_ERR, NCRESULT_OK,
    },
    cstring, NcCell, NcPlane,
};

// Alpha -----------------------------------------------------------------------

/// Gets the foreground [`NcAlpha_u32`] from the [`NcPlane`], shifted to LSBs.
///
/// *Method: NcPlane.[fg_alpha()][NcPlane#method.fg_alpha].*
#[inline]
pub fn ncplane_fg_alpha(plane: &NcPlane) -> NcAlpha_u32 {
    c_api::ncchannels_fg_alpha(ncplane_channels(plane))
}

/// Gets the background [`NcAlpha_u32`] from the [`NcPlane`], shifted to LSBs.
///
/// *Method: NcPlane.[bg_alpha()][NcPlane#method.bg_alpha].*
#[inline]
pub fn ncplane_bg_alpha(plane: &NcPlane) -> NcAlpha_u32 {
    c_api::ncchannels_bg_alpha(ncplane_channels(plane))
}

// NcChannel_u32 ---------------------------------------------------------------

/// Gets the foreground alpha and coloring bits from an [`NcPlane`] as an [`NcChannel_u32`].
///
/// *Method: NcPlane.[fchannel()][NcPlane#method.fchannel].*
#[inline]
pub fn ncplane_fchannel(plane: &NcPlane) -> NcChannel_u32 {
    c_api::ncchannels_fchannel(ncplane_channels(plane))
}

/// Gets the background alpha and coloring bits from an [`NcPlane`] as an [`NcChannel_u32`].
///
/// *Method: NcPlane.[bchannel()][NcPlane#method.bchannel].*
#[inline]
pub fn ncplane_bchannel(plane: &NcPlane) -> NcChannel_u32 {
    c_api::ncchannels_bchannel(ncplane_channels(plane))
}

/// Sets the background alpha and coloring bits of an [`NcPlane`] from an [`NcChannel_u32`],
/// and returns the new [`NcChannels_u64`].
///
/// *Method: NcPlane.[set_fchannel()][NcPlane#method.set_fchannel].*
#[inline]
pub fn ncplane_set_fchannel(
    plane: &mut NcPlane,
    channel: impl Into<NcChannel_u32>,
) -> NcChannels_u64 {
    unsafe { c_api::ffi::ncplane_set_fchannel(plane, channel.into()) }
}

/// Sets the background alpha and coloring bits of an [`NcPlane`] from an [`NcChannel_u32`],
/// and returns the new [`NcChannels_u64`].
///
/// *Method: NcPlane.[set_bchannel()][NcPlane#method.set_bchannel].*
#[inline]
pub fn ncplane_set_bchannel(
    plane: &mut NcPlane,
    channel: impl Into<NcChannel_u32>,
) -> NcChannels_u64 {
    unsafe { c_api::ffi::ncplane_set_bchannel(plane, channel.into()) }
}

/// Gets the [`NcChannels_u64`] of an [`NcPlane`].
///
/// *Method: NcPlane.[channels()][NcPlane#method.channels].*
#[inline]
pub fn ncplane_channels(plane: &NcPlane) -> NcChannels_u64 {
    unsafe { c_api::ffi::ncplane_channels(plane) }
}

/// Sets the [`NcChannels_u64`] of an [`NcPlane`].
///
/// *Method: NcPlane.[set_channels()][NcPlane#method.set_channels].*
#[inline]
pub fn ncplane_set_channels(plane: &mut NcPlane, channels: impl Into<NcChannels_u64>) {
    unsafe {
        c_api::ffi::ncplane_set_channels(plane, channels.into());
    }
}

// RGB components --------------------------------------------------------------

/// Gets the foreground RGB components from an [`NcPlane`].
/// and returns the background [`NcChannel_u32`].
///
/// *Method: NcPlane.[fg_rgb()][NcPlane#method.fg_rgb].*
#[inline]
pub fn ncplane_fg_rgb8(
    plane: &NcPlane,
    red: &mut u8,
    green: &mut u8,
    blue: &mut u8,
) -> NcChannel_u32 {
    c_api::ncchannels_fg_rgb8(ncplane_channels(plane), red, green, blue)
}

/// Gets the background RGB componentss from an [`NcPlane`],
/// and returns the background [`NcChannel_u32`].
///
/// *Method: NcPlane.[bg_rgb()][NcPlane#method.bg_rgb].*
#[inline]
pub fn ncplane_bg_rgb8(
    plane: &NcPlane,
    red: &mut u8,
    green: &mut u8,
    blue: &mut u8,
) -> NcChannel_u32 {
    c_api::ncchannels_bg_rgb8(ncplane_channels(plane), red, green, blue)
}

// NcRgb_u32 -----------------------------------------------------------------------

/// Gets the foreground [`NcRgb_u32`] from an [`NcPlane`], shifted to LSBs.
///
/// *Method: NcPlane.[fg_rgb()][NcPlane#method.fg_rgb].*
#[inline]
pub fn ncplane_fg_rgb(plane: &NcPlane) -> NcRgb_u32 {
    c_api::ncchannels_fg_rgb(ncplane_channels(plane))
}

/// Gets the background [`NcRgb_u32`] from an [`NcPlane`], shifted to LSBs.
///
/// *Method: NcPlane.[bg_rgb()][NcPlane#method.bg_rgb].*
#[inline]
pub fn ncplane_bg_rgb(plane: &NcPlane) -> NcRgb_u32 {
    c_api::ncchannels_bg_rgb(ncplane_channels(plane))
}

// Default ---------------------------------------------------------------------

/// Is the plane's foreground using the "default foreground color"?
///
/// *Method: NcPlane.[fg_default_p()][NcPlane#method.fg_default_p].*
#[inline]
pub fn ncplane_fg_default_p(plane: &NcPlane) -> bool {
    c_api::ncchannels_fg_default_p(ncplane_channels(plane))
}

/// Is the plane's background using the "default background color"?
///
/// *Method: NcPlane.[bg_default_p()][NcPlane#method.bg_default_p].*
#[inline]
pub fn ncplane_bg_default_p(plane: &NcPlane) -> bool {
    c_api::ncchannels_bg_default_p(ncplane_channels(plane))
}

/// Marks both the foreground and background as using the "default color",
/// and returns the new [`NcChannels_u64`].
///
/// *Method: NcPlane.[set_default()][NcPlane#method.set_default].*
//
// Not in the C API.
#[inline]
pub fn ncplane_set_default(plane: &mut NcPlane) -> NcChannels_u64 {
    let channels = c_api::ncchannels_set_default(&mut ncplane_channels(plane));
    ncplane_set_channels(plane, channels);
    channels
}

/// Marks both the foreground and background as NOT using the "default color",
/// and returns the new [`NcChannels_u64`].
///
/// *Method: NcPlane.[set_not_default()][NcPlane#method.set_not_default].*
//
// Not in the C API.
#[inline]
pub fn ncplane_set_not_default(plane: &mut NcPlane) -> NcChannels_u64 {
    let channels = c_api::ncchannels_set_not_default(&mut ncplane_channels(plane));
    c_api::ncplane_set_channels(plane, channels);
    channels
}

/// Marks the foreground as NOT using the "default color",
/// and returns the new [`NcChannels_u64`].
///
/// *Method: NcPlane.[set_fg_not_default()][NcPlane#method.set_fg_not_default].*
//
// Not in the C API.
#[inline]
pub fn ncplane_set_fg_not_default(plane: &NcPlane) -> NcChannels_u64 {
    c_api::ncchannels_set_fg_not_default(&mut ncplane_channels(plane))
}

/// Marks the background as NOT using the "default color",
/// and returns the new [`NcChannels_u64`].
///
/// *Method: NcPlane.[set_bg_not_default()][NcPlane#method.set_bg_not_default].*
//
// Not in the C API.
#[inline]
pub fn ncplane_set_bg_not_default(plane: &NcPlane) -> NcChannels_u64 {
    c_api::ncchannels_set_bg_not_default(&mut ncplane_channels(plane))
}

// put & print -----------------------------------------------------------------

/// Replaces the [`NcCell`] at the current location with the provided `cell`,
/// advancing the cursor by its width (but not past the end of the plane).
///
/// The new `cell` must already be associated with this `NcPlane`.
///
/// On success, returns the number of columns the cursor was advanced.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putc()][NcPlane#method.putc].*
#[inline]
pub fn ncplane_putc(plane: &mut NcPlane, cell: &NcCell) -> NcResult_i32 {
    unsafe { c_api::ncplane_putc_yx(plane, -1, -1, cell) }
}

/// Replaces the [`NcCell`] at the current location with the provided [`char`],
/// using the current style.
///
/// On success, returns the number of columns the cursor was advanced.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putchar()][NcPlane#method.putchar].*
#[inline]
pub fn ncplane_putchar(plane: &mut NcPlane, ch: char) -> NcResult_i32 {
    unsafe {
        let cell = NcCell::from_char(plane, ch);
        if cell.is_err() {
            return NCRESULT_ERR;
        }
        c_api::ncplane_putc_yx(plane, -1, -1, &cell.unwrap())
    }
}

/// Replaces the [`NcCell`] at the specified coordinates with the provided
/// [`char`], using the current style.
///
/// On success, returns the number of columns the cursor was advanced.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putchar_yx()][NcPlane#method.putchar_yx].*
#[inline]
pub fn ncplane_putchar_yx(plane: &mut NcPlane, y: u32, x: u32, ch: char) -> NcResult_i32 {
    unsafe {
        let cell = NcCell::from_char(plane, ch);
        if cell.is_err() {
            return NCRESULT_ERR;
        }
        c_api::ncplane_putc_yx(plane, y as i32, x as i32, &cell.unwrap())
    }
}

/// Replaces the [`NcCell`] at the current location with the provided
/// [`char`], while retaining the previous style.
///
/// On success, returns the number of columns the cursor was advanced.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putchar_stained()][NcPlane#method.putchar_stained].*
#[inline]
pub fn ncplane_putchar_stained(plane: &mut NcPlane, ch: char) -> NcResult_i32 {
    ncplane_putstr_stained(plane, &ch.to_string())
}

/// Replaces the [`NcCell`] at the current location with the provided `egc`,
/// using the current style.
///
/// Advances the cursor by the width of the cluster (but not past the end of
/// the the plane), and this number is returned on success.
///
/// The number of bytes converted from the `egc` can be optionally written to
/// `sbytes`.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putegc()][NcPlane#method.putegc].*
#[inline]
pub fn ncplane_putegc(plane: &mut NcPlane, egc: &str, sbytes: Option<&mut usize>) -> NcResult_i32 {
    let sbytes_ptr = if let Some(sb) = sbytes { sb as *mut _ } else { null_mut() };
    let cs = cstring![egc];
    let egc_ptr = cs.as_ptr() as *const i8; // CHECK why is this needed only here

    unsafe { c_api::ffi::ncplane_putegc_yx(plane, -1, -1, egc_ptr, sbytes_ptr) }
}

/// Replaces the [`NcCell`] at the specified coordinates with the provided `egc`,
/// using the current style.
///
/// Advances the cursor by the width of the cluster (but not past the end of
/// the the plane), and this number is returned on success.
///
/// The number of bytes converted from the `egc` can be optionally written to
/// `sbytes`.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putegc_yx()][NcPlane#method.putegc_yx].*
#[inline]
pub fn ncplane_putegc_yx(
    plane: &mut NcPlane,
    y: Option<u32>,
    x: Option<u32>,
    egc: &str,
    sbytes: Option<&mut usize>,
) -> NcResult_i32 {
    let sbytes_ptr = if let Some(sb) = sbytes { sb as *mut _ } else { null_mut() };
    let cs = cstring![egc];

    unsafe {
        c_api::ffi::ncplane_putegc_yx(
            plane,
            y.unwrap_or(u32::MAX) as i32,
            x.unwrap_or(u32::MAX) as i32,
            cs.as_ptr(),
            sbytes_ptr,
        )
    }
}

/// Replaces the [`NcCell`] at the current location with the provided `egc`,
/// while retaining the previous style.
///
/// Advances the cursor by the width of the cluster (but not past the end of
/// the the plane), and this number is returned on success.
///
/// The number of bytes converted from the `egc` can be optionally written to
/// `sbytes`.
///
/// If the glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putegc_stained()][NcPlane#method.putegc_stained].*
#[inline]
pub fn ncplane_putegc_stained(
    plane: &mut NcPlane,
    egc: &str,
    sbytes: Option<&mut usize>,
) -> NcResult_i32 {
    let sbytes_ptr = if let Some(sb) = sbytes { sb as *mut _ } else { null_mut() };

    let cs = cstring![egc];
    unsafe { c_api::ffi::ncplane_putegc_stained(plane, cs.as_ptr(), sbytes_ptr) }
}

/// Writes a string to the current location, using the current style.
///
/// Advances the cursor by some positive number of columns (though not beyond
/// the end of the plane), and this number is returned on success.
///
/// On error, a non-positive number is returned, indicating the number of
/// columns which were written before the error.
///
/// If a glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putstr()][NcPlane#method.putstr].*
#[inline]
pub fn ncplane_putstr(plane: &mut NcPlane, string: &str) -> NcResult_i32 {
    ncplane_putstr_yx(plane, None, None, string)
}

/// Writes a string to the provided location, using the current style.
///
/// Advances the cursor by some positive number of columns (though not beyond
/// the end of the plane), and this number is returned on success.
///
/// On error, a non-positive number is returned, indicating the number of
/// columns which were written before the error.
///
/// If a glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putstr_yx()][NcPlane#method.putstr_yx].*
#[inline]
pub fn ncplane_putstr_yx(
    plane: &mut NcPlane,
    y: Option<u32>,
    x: Option<u32>,
    string: &str,
) -> NcResult_i32 {
    let cs = cstring![string];
    let mut cs_ptr = cs.as_ptr();

    let (mut y, mut x) = (y, x);
    let mut ret = 0;

    while unsafe { cs_ptr.read() != 0 } {
        let mut wcs = 0;
        let cols = unsafe {
            c_api::ffi::ncplane_putegc_yx(
                plane,
                y.unwrap_or(u32::MAX) as i32,
                x.unwrap_or(u32::MAX) as i32,
                cs_ptr,
                &mut wcs,
            )
        };
        if cols < 0 {
            return -ret;
        }
        if wcs == 0 {
            break;
        }
        // after the first iteration, just let the cursor code control where we
        // print, so that scrolling is taken into account
        y = None;
        x = None;

        cs_ptr = unsafe { cs_ptr.add(wcs) };
        ret += cols;
    }
    ret
}

/// Writes a string to the provided location, using the current style
/// and [`NcAlign_u32`]ed on *x*.
///
/// Advances the cursor by some positive number of columns (though not
/// beyond the end of the plane); this number is returned on success.
///
/// On error, a non-positive number is returned, indicating the number of
/// columns which were written before the error.
///
/// If a glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putstr_aligned()][NcPlane#method.putstr_aligned].*
#[inline]
pub fn ncplane_putstr_aligned(
    plane: &mut NcPlane,
    y: Option<u32>,
    align: impl Into<NcAlign_u32>,
    string: &str,
) -> NcResult_i32 {
    let (mut validbytes, mut validwidth) = (0, 0);
    let cs = cstring![string];

    // we'll want to do the partial write if there's an error somewhere within
    unsafe {
        c_api::ncstrwidth(cs.as_ptr(), &mut validbytes, &mut validwidth);
    }

    let xpos = ncplane_halign(plane, align.into(), validwidth as u32);
    if xpos < 0 {
        NCRESULT_ERR
    } else {
        ncplane_putstr_yx(plane, y, Some(xpos as u32), string)
    }
}

/// Writes a string to the current location, retaining the previous style.
///
/// Advances the cursor by some positive number of columns (though not
/// beyond the end of the plane); this number is returned on success.
///
/// On error, a non-positive number is returned, indicating the number of
/// columns which were written before the error.
///
/// If a glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putstr_stained()][NcPlane#method.putstr_stained].*
#[inline]
pub fn ncplane_putstr_stained(plane: &mut NcPlane, string: &str) -> NcResult_i32 {
    let cstring = CString::new(string).unwrap();
    let mut cstring_ptr = cstring.as_ptr();

    let mut ret = 0;
    while unsafe { cstring_ptr.read() != 0 } {
        let mut wcs = 0;
        let cols = unsafe { c_api::ffi::ncplane_putegc_stained(plane, cstring_ptr, &mut wcs) };

        if cols < 0 {
            return -ret;
        }
        if wcs == 0 {
            break;
        }
        cstring_ptr = unsafe { cstring_ptr.add(wcs) };
        ret += cols;
    }
    ret
}

/// Writes a series of EGCs to the provided location, using the current style,
/// and no more than `num_bytes` bytes will be written.
///
/// Advances the cursor by some positive number of columns (though not beyond
/// the end of the plane); this number is returned on success.
///
/// On error, a non-positive number is returned, indicating the number of
/// columns which were written before the error.
///
/// If a glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putnstr()][NcPlane#method.putnstr].*
#[inline]
pub fn ncplane_putnstr_yx(
    plane: &mut NcPlane,
    y: Option<u32>,
    x: Option<u32>,
    num_bytes: usize,
    string: &str,
) -> NcResult_i32 {
    let cstring = CString::new(string).unwrap();
    let cstring_bytes_len = cstring.as_bytes().len();
    let cstring_ptr = cstring.as_ptr();

    let (ret, mut offset) = (0, 0);
    let (mut y, mut x) = (y, x);

    while offset < num_bytes && offset < cstring_bytes_len {
        let mut wcs = 0;
        let cols = unsafe {
            c_api::ffi::ncplane_putegc_yx(
                plane,
                y.unwrap_or(u32::MAX) as i32,
                x.unwrap_or(u32::MAX) as i32,
                cstring_ptr.add(offset),
                &mut wcs,
            )
        };
        if cols < 0 {
            return c_api::NCRESULT_ERR;
        }
        if wcs == 0 {
            break;
        }

        // after the first iteration, just let the cursor code control where we
        // print, so that scrolling is taken into account
        y = None;
        x = None;
        offset += wcs as usize;
    }
    ret
}

/// Writes a string to the current location, using the current style,
/// and no more than `num_bytes` bytes will be written.
///
/// Advances the cursor by some positive number of columns (though not beyond
/// the end of the plane); this number is returned on success.
///
/// On error, a non-positive number is returned, indicating the number of
/// columns which were written before the error.
///
/// If a glyph can not fit in the current line, it is an error, unless
/// scrolling is enabled.
///
/// *Method: NcPlane.[putnstr()][NcPlane#method.putnstr].*
#[inline]
pub fn ncplane_putnstr(plane: &mut NcPlane, num_bytes: usize, string: &str) -> NcResult_i32 {
    c_api::ncplane_putnstr_yx(plane, None, None, num_bytes, string)
}

// movement, size & alignment --------------------------------------------------

/// Moves this `NcPlane` relative to its current location.
///
/// Negative values move up and left, respectively.
/// Pass 0 to hold an axis constant.
///
/// It is an error to attempt to move the standard plane.
///
/// *Method: NcPlane.[move_rel()][NcPlane#method.move_rel].*
#[inline]
pub fn ncplane_moverel(plane: &mut NcPlane, rows: i32, cols: i32) -> NcResult_i32 {
    let (mut orig_y, mut orig_x) = (0, 0);
    unsafe {
        c_api::ncplane_yx(plane, &mut orig_y, &mut orig_x);
        c_api::ncplane_move_yx(plane, orig_y + rows, orig_x + cols)
    }
}

/// Relocates this `NcPlane` at the bottom of the z-buffer.
///
/// *Method: NcPlane.[move_bottom()][NcPlane#method.move_bottom].*
#[inline]
pub fn ncplane_move_bottom(plane: &mut NcPlane) {
    unsafe {
        c_api::ncplane_move_above(plane, null_mut());
    }
}

/// Relocates this `NcPlane` at the top of the z-buffer.
///
/// *Method: NcPlane.[move_top()][NcPlane#method.move_top].*
#[inline]
pub fn ncplane_move_top(plane: &mut NcPlane) {
    unsafe {
        c_api::ncplane_move_below(plane, null_mut());
    }
}

/// Splices this plane and its bound planes out of the z-buffer,
/// and reinserts them at the top.
///
/// Relative order will be maintained between the reinserted planes.
///
/// For a plane E bound to C, with z-ordering A B C D E, moving the C family
/// to the top results in C E A B D.
///
/// *Method: NcPlane.[move_family_top()][NcPlane#method.move_family_top].*
#[inline]
pub fn ncplane_move_family_top(plane: &mut NcPlane) {
    unsafe {
        c_api::ncplane_move_family_below(plane, null_mut());
    }
}

/// Splices this plane and its bound planes out of the z-buffer,
/// and reinserts them at the bottom.
///
/// Relative order will be maintained between the reinserted planes.
///
/// For a plane E bound to C, with z-ordering A B C D E, moving the C family
/// to the bottom results in A B D C E.
///
/// *Method: NcPlane.[move_family_bottom()][NcPlane#method.move_family_bottom].*
#[inline]
pub fn ncplane_move_family_bottom(plane: &mut NcPlane) {
    unsafe {
        c_api::ncplane_move_family_above(plane, null_mut());
    }
}

/// Gets the columns of the [`NcPlane`].
///
/// *Method: NcPlane.[dim_x()][NcPlane#method.dim_x].*
#[inline]
pub fn ncplane_dim_x(plane: &NcPlane) -> u32 {
    unsafe {
        let mut x = 0;
        c_api::ncplane_dim_yx(plane, null_mut(), &mut x);
        x as u32
    }
}

/// Gets the rows of the [`NcPlane`].
///
/// *Method: NcPlane.[dim_y()][NcPlane#method.dim_y].*
#[inline]
pub fn ncplane_dim_y(plane: &NcPlane) -> u32 {
    unsafe {
        let mut y = 0;
        c_api::ncplane_dim_yx(plane, &mut y, null_mut());
        y as u32
    }
}

/// Resizes the plane, retaining what data we can (everything, unless we're
/// shrinking in some dimension). Keep the origin where it is.
///
/// *Method: NcPlane.[resize_simple()][NcPlane#method.resize_simple].*
#[inline]
pub fn ncplane_resize_simple(plane: &mut NcPlane, len_y: u32, len_x: u32) -> NcResult_i32 {
    let (mut old_y, mut old_x) = (0, 0);
    unsafe {
        c_api::ncplane_dim_yx(plane, &mut old_y, &mut old_x);
    }
    let keep_len_y = {
        if old_y > len_y {
            len_y
        } else {
            old_y
        }
    };
    let keep_len_x = {
        if old_x > len_x {
            len_x
        } else {
            old_x
        }
    };
    unsafe {
        c_api::ncplane_resize(
            plane,
            0,
            0,
            keep_len_y as u32,
            keep_len_x as u32,
            0,
            0,
            len_y,
            len_x,
        )
    }
}

/// Returns the column at which `numcols` columns ought start in order to be
/// aligned according to `align` within the `plane`.
///
/// Returns `-`[`NCRESULT_MAX`][c_api::NCRESULT_MAX] if
/// [NCALIGN_UNALIGNED][c_api::NCALIGN_UNALIGNED].
///
/// *Method: NcPlane.[halign()][NcPlane#method.halign].*
#[inline]
pub fn ncplane_halign(
    plane: &NcPlane,
    align: impl Into<NcAlign_u32> + Copy,
    numcols: u32,
) -> NcResult_i32 {
    c_api::notcurses_align(ncplane_dim_x(plane), align, numcols)
}

/// Returns the row at which `numrows` rows ought start in order to be aligned
/// according to `align` within this NcPlane.
///
/// Returns `-`[`NCRESULT_MAX`][c_api::NCRESULT_MAX] if
/// [NCALIGN_UNALIGNED][c_api::NCALIGN_UNALIGNED].
///
/// *Method: NcPlane.[valign()][NcPlane#method.valign].*
#[inline]
pub fn ncplane_valign(
    plane: &NcPlane,
    align: impl Into<NcAlign_u32> + Copy,
    numrows: u32,
) -> NcResult_i32 {
    c_api::notcurses_align(ncplane_dim_y(plane), align, numrows)
}

// line ------------------------------------------------------------------------

/// Draws horizontal lines using the specified NcCell, starting at the current
/// cursor position.
///
/// The cursor will end at the cell following the last cell output,
/// just as if [`ncplane_putc`] was called at that spot.
///
/// Returns the number of cells drawn on success. On error, returns the negative
/// number of cells drawn.
///
/// *Method: NcPlane.[hline()][NcPlane#method.hline].*
#[inline]
pub fn ncplane_hline(plane: &mut NcPlane, cell: &NcCell, len: u32) -> NcResult_i32 {
    unsafe { c_api::ncplane_hline_interp(plane, cell, len, cell.channels, cell.channels) }
}

/// Draws vertical lines using the specified `cell`, starting at the current
/// cursor position.
///
/// The cursor will end at the cell following the last cell output,
/// just as if [`ncplane_putc`] was called at that spot.
///
/// Returns the number of cells drawn on success. On error, returns the negative
/// number of cells drawn.
///
/// *Method: NcPlane.[vline()][NcPlane#method.vline].*
#[inline]
pub fn ncplane_vline(plane: &mut NcPlane, cell: &NcCell, len: u32) -> NcResult_i32 {
    unsafe { c_api::ncplane_vline_interp(plane, cell, len, cell.channels, cell.channels) }
}

// perimeter -------------------------------------------------------------------

/// Draws the perimeter around `plane`.
///
/// *Method: NcPlane.[perimeter()][NcPlane#method.perimeter].*
#[inline]
pub fn ncplane_perimeter(
    plane: &mut NcPlane,
    ul: &NcCell,
    ur: &NcCell,
    ll: &NcCell,
    lr: &NcCell,
    hline: &NcCell,
    vline: &NcCell,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    unsafe {
        c_api::ncplane_cursor_move_yx(plane, 0, 0);
        let (mut dimy, mut dimx) = (0, 0);
        c_api::ncplane_dim_yx(plane, &mut dimy, &mut dimx);
        ncplane_box_sized(
            plane,
            ul,
            ur,
            ll,
            lr,
            hline,
            vline,
            dimy as u32,
            dimx as u32,
            boxmask.into(),
        )
    }
}

/// Like [`ncplane_perimeter`] with the double box-drawing characters.
///
/// *Method: NcPlane.[perimeter_double()][NcPlane#method.perimeter_double].*
#[inline]
pub fn ncplane_perimeter_double(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    if unsafe { c_api::ncplane_cursor_move_yx(plane, 0, 0) } != NCRESULT_OK {
        return NCRESULT_ERR;
    }
    let (mut dimy, mut dimx) = (0, 0);
    unsafe {
        c_api::ncplane_dim_yx(plane, &mut dimy, &mut dimx);
    }
    let mut ul = NcCell::new();
    let mut ur = NcCell::new();
    let mut ll = NcCell::new();
    let mut lr = NcCell::new();
    let mut hl = NcCell::new();
    let mut vl = NcCell::new();
    if c_api::nccells_double_box(
        plane, stylemask, channels, &mut ul, &mut ur, &mut ll, &mut lr, &mut hl, &mut vl,
    ) != NCRESULT_OK
    {
        return NCRESULT_ERR;
    }
    let ret = ncplane_box_sized(
        plane,
        &ul,
        &ur,
        &ll,
        &lr,
        &hl,
        &vl,
        dimy as u32,
        dimx as u32,
        boxmask,
    );
    unsafe {
        nccell_release(plane, &mut ul);
        nccell_release(plane, &mut ur);
        nccell_release(plane, &mut ll);
        nccell_release(plane, &mut lr);
        nccell_release(plane, &mut hl);
        nccell_release(plane, &mut vl);
    }
    ret
}

/// Like [`ncplane_perimeter`] with the rounded box-drawing characters.
///
/// *Method: NcPlane.[perimeter_rounded()][NcPlane#method.perimeter_rounded].*
#[inline]
pub fn ncplane_perimeter_rounded(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    if unsafe { c_api::ncplane_cursor_move_yx(plane, 0, 0) } != NCRESULT_OK {
        return NCRESULT_ERR;
    }
    let (mut dimy, mut dimx) = (0, 0);
    unsafe {
        c_api::ncplane_dim_yx(plane, &mut dimy, &mut dimx);
    }
    let mut ul = NcCell::new();
    let mut ur = NcCell::new();
    let mut ll = NcCell::new();
    let mut lr = NcCell::new();
    let mut hl = NcCell::new();
    let mut vl = NcCell::new();
    if c_api::nccells_rounded_box(
        plane, stylemask, channels, &mut ul, &mut ur, &mut ll, &mut lr, &mut hl, &mut vl,
    ) != NCRESULT_OK
    {
        return NCRESULT_ERR;
    }
    let ret = ncplane_box_sized(
        plane,
        &ul,
        &ur,
        &ll,
        &lr,
        &hl,
        &vl,
        dimy as u32,
        dimx as u32,
        boxmask,
    );
    unsafe {
        nccell_release(plane, &mut ul);
        nccell_release(plane, &mut ur);
        nccell_release(plane, &mut ll);
        nccell_release(plane, &mut lr);
        nccell_release(plane, &mut hl);
        nccell_release(plane, &mut vl);
    }
    ret
}

// box -------------------------------------------------------------------------

/// Like [`ncplane_box`][c_api::ncplane_box] using only ASCII characters.
///
/// *Method: NcPlane.[ascii_box()][NcPlane#method.ascii_box].*
#[inline]
pub fn ncplane_ascii_box(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    end_y: u32,
    end_x: u32,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    #[allow(unused_assignments)]
    let mut ret = NCRESULT_OK;

    let mut ul = NcCell::new();
    let mut ur = NcCell::new();
    let mut ll = NcCell::new();
    let mut lr = NcCell::new();
    let mut hl = NcCell::new();
    let mut vl = NcCell::new();

    unsafe {
        ret = c_api::nccells_ascii_box(
            plane, stylemask, channels, &mut ul, &mut ur, &mut ll, &mut lr, &mut hl, &mut vl,
        );
        if ret == NCRESULT_OK {
            ret = c_api::ncplane_box(
                plane,
                &ul,
                &ur,
                &ll,
                &lr,
                &hl,
                &vl,
                end_y,
                end_x,
                boxmask.into(),
            );
        }

        nccell_release(plane, &mut ul);
        nccell_release(plane, &mut ur);
        nccell_release(plane, &mut ll);
        nccell_release(plane, &mut lr);
        nccell_release(plane, &mut hl);
        nccell_release(plane, &mut vl);
    }
    ret
}

/// Draws a box with its upper-left corner at the current cursor position,
/// having dimensions `len_y` * `len_x`.
///
/// The minimum box size is 2x2, and it cannot be drawn off-screen.
///
/// See [ncplane_box()](c_api::ncplane_box) for more information.
///
/// *Method: NcPlane.[box_sized()][NcPlane#method.box_sized].*
#[inline]
pub fn ncplane_box_sized(
    plane: &mut NcPlane,
    ul: &NcCell,
    ur: &NcCell,
    ll: &NcCell,
    lr: &NcCell,
    hline: &NcCell,
    vline: &NcCell,
    len_y: u32,
    len_x: u32,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    let (mut y, mut x) = (0, 0);
    unsafe {
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
        c_api::ncplane_box(
            plane,
            ul,
            ur,
            ll,
            lr,
            hline,
            vline,
            (y + len_y - 1) as u32,
            (x + len_x - 1) as u32,
            boxmask.into(),
        )
    }
}

/// Like [`ncplane_box`][c_api::ncplane_box] with the double box-drawing
/// characters.
///
/// *Method: NcPlane.[double_box()][NcPlane#method.double_box].*
#[inline]
pub fn ncplane_double_box(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    end_y: u32,
    end_x: u32,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    #[allow(unused_assignments)]
    let mut ret = NCRESULT_OK;

    let mut ul = NcCell::new();
    let mut ur = NcCell::new();
    let mut ll = NcCell::new();
    let mut lr = NcCell::new();
    let mut hl = NcCell::new();
    let mut vl = NcCell::new();

    unsafe {
        ret = c_api::nccells_double_box(
            plane, stylemask, channels, &mut ul, &mut ur, &mut ll, &mut lr, &mut hl, &mut vl,
        );
        if ret == NCRESULT_OK {
            ret = c_api::ncplane_box(
                plane,
                &ul,
                &ur,
                &ll,
                &lr,
                &hl,
                &vl,
                end_y,
                end_x,
                boxmask.into(),
            );
        }

        nccell_release(plane, &mut ul);
        nccell_release(plane, &mut ur);
        nccell_release(plane, &mut ll);
        nccell_release(plane, &mut lr);
        nccell_release(plane, &mut hl);
        nccell_release(plane, &mut vl);
    }
    ret
}

/// Like [`ncplane_box_sized`] with the double box-drawing characters.
///
/// *Method: NcPlane.[double_box_sized()][NcPlane#method.double_box_sized].*
#[inline]
pub fn ncplane_double_box_sized(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    len_y: u32,
    len_x: u32,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    let (mut y, mut x) = (0, 0);
    unsafe {
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
    }
    c_api::ncplane_double_box(
        plane,
        stylemask,
        channels,
        y as u32 + len_y - 1,
        x as u32 + len_x - 1,
        boxmask,
    )
}

/// Like [`ncplane_box`][c_api::ncplane_box] with the rounded box-drawing
/// characters.
///
/// *Method: NcPlane.[rounded_box()][NcPlane#method.rounded_box].*
#[inline]
pub fn ncplane_rounded_box(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    end_y: u32,
    end_x: u32,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    #[allow(unused_assignments)]
    let mut ret = NCRESULT_OK;

    let mut ul = NcCell::new();
    let mut ur = NcCell::new();
    let mut ll = NcCell::new();
    let mut lr = NcCell::new();
    let mut hl = NcCell::new();
    let mut vl = NcCell::new();

    unsafe {
        ret = c_api::nccells_rounded_box(
            plane, stylemask, channels, &mut ul, &mut ur, &mut ll, &mut lr, &mut hl, &mut vl,
        );
        if ret == NCRESULT_OK {
            ret = c_api::ncplane_box(
                plane,
                &ul,
                &ur,
                &ll,
                &lr,
                &hl,
                &vl,
                end_y,
                end_x,
                boxmask.into(),
            );
        }
        nccell_release(plane, &mut ul);
        nccell_release(plane, &mut ur);
        nccell_release(plane, &mut ll);
        nccell_release(plane, &mut lr);
        nccell_release(plane, &mut hl);
        nccell_release(plane, &mut vl);
    }
    ret
}

/// Like [`ncplane_box_sized`] with the rounded box-drawing characters.
///
/// *Method: NcPlane.[rounded_box_sized()][NcPlane#method.rounded_box_sized].*
#[inline]
pub fn ncplane_rounded_box_sized(
    plane: &mut NcPlane,
    stylemask: impl Into<NcStyle_u16>,
    channels: impl Into<NcChannels_u64>,
    len_y: u32,
    len_x: u32,
    boxmask: impl Into<NcBoxMask_u32>,
) -> NcResult_i32 {
    let (mut y, mut x) = (0, 0);
    unsafe {
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
    }
    ncplane_rounded_box(
        plane,
        stylemask,
        channels,
        y as u32 + len_y - 1,
        x as u32 + len_x - 1,
        boxmask,
    )
}

// gradient --------------------------------------------------------------------

/// Draws a gradient with its upper-left corner at the current cursor position,
/// stopping at `end_y`×`end_x`.
///
/// Use `None` for either or all of `beg_y` and `beg_x` in order to
/// use the current cursor position along that axis.
///
/// Use `None` for either or both of `len_y` and `len_x` in order to
/// go through the boundary of the plane in that axis (same as `0`).
///
/// The glyph composed of `egc` and `stylemask` is used for all cells. The
/// `NcChannels_u64` specified by `ul`, `ur`, `ll`, and `lr` are composed into
/// foreground and background gradients.
///
/// - To do a vertical gradient, `ul` ought equal `ur` and `ll` ought equal `lr`.
/// - To do a horizontal gradient, `ul` ought equal `ll` and `ur` ought equal `ul`.
/// - To color everything the same, all four channels should be equivalent. The
/// resulting alpha values are equal to incoming alpha values. Returns the number
/// of cells filled on success, or -1 on failure.
///
/// Palette-indexed color is not supported.
///
/// Preconditions for gradient operations (error otherwise):
/// - all: only RGB colors, unless all four channels match as default
/// - all: all alpha values must be the same
/// - 1x1: all four colors must be the same
/// - 1xN: both top and both bottom colors must be the same (vertical gradient)
/// - Nx1: both left and both right colors must be the same (horizontal gradient)
///
/// *Method: NcPlane.[gradient()][NcPlane#method.gradient].*
#[inline]
pub fn ncplane_gradient(
    plane: &mut NcPlane,
    y: Option<u32>,
    x: Option<u32>,
    len_y: Option<u32>,
    len_x: Option<u32>,
    egc: &str,
    stylemask: impl Into<NcStyle_u16>,
    ul: impl Into<NcChannels_u64>,
    ur: impl Into<NcChannels_u64>,
    ll: impl Into<NcChannels_u64>,
    lr: impl Into<NcChannels_u64>,
) -> NcResult_i32 {
    let cs = cstring![egc];

    #[cfg(any(target_arch = "armv7l", target_arch = "i686"))]
    let egc_ptr = cs.as_ptr() as *const i8;
    #[cfg(not(any(target_arch = "armv7l", target_arch = "i686")))]
    let egc_ptr = cs.as_ptr();

    unsafe {
        c_api::ffi::ncplane_gradient(
            plane,
            y.unwrap_or(u32::MAX) as i32,
            x.unwrap_or(u32::MAX) as i32,
            len_y.unwrap_or(0),
            len_x.unwrap_or(0),
            egc_ptr,
            stylemask.into(),
            ul.into(),
            ur.into(),
            ll.into(),
            lr.into(),
        )
    }
}

// cursor --------------------------------------------------------------------

/// Returns the current row of the cursor within this `NcPlane`.
///
/// *Method: NcPlane.[gradient()][NcPlane#method.gradient].*
#[inline]
pub fn ncplane_cursor_y(plane: &NcPlane) -> u32 {
    let mut y = 0;
    unsafe {
        c_api::ncplane_cursor_yx(plane, &mut y, null_mut());
    }
    y
}

/// Returns the current row of the cursor within this `NcPlane`.
///
/// *Method: NcPlane.[gradient()][NcPlane#method.gradient].*
#[inline]
pub fn ncplane_cursor_x(plane: &NcPlane) -> u32 {
    let mut x = 0;
    unsafe {
        c_api::ncplane_cursor_yx(plane, null_mut(), &mut x);
    }
    x
}
