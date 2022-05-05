//! `Nc`

// total: 55
// ---------------------------------------------------
// (X)  1 : wont do
// (…)  4 : TODO / WIP
//
// (f) 36 : unsafe ffi function exported by bindgen
// (w)  0 : safely wrapped ffi function
// (r) 15 : static function manually reimplemented
//
// (m) 42 : method implemented
//
// (t) 13 : unit test done for the function
// (T)  0 : unit test done also for the method
// ---------------------------------------------------
// fm  notcurses_at_yx
// rm  notcurses_bottom
// rm  notcurses_canbraille
// rmt notcurses_canchangecolor
// rmt notcurses_canfade
// fmt notcurses_canopen_images
// fmt notcurses_canopen_videos
// rmt notcurses_cansextant
// rmt notcurses_cantruecolor
// rmt notcurses_canutf8
// fm  notcurses_check_pixel_support
//~f   notcurses_core_init
// fm  notcurses_cursor_disable
// fm  notcurses_cursor_enable
// f   notcurses_cursor_yx
// fmt notcurses_debug
// fm  notcurses_default_background
// fm  notcurses_default_foreground
//~f   notcurses_detected_terminal
// fmt notcurses_drop_planes
// fm  notcurses_get
// fm  notcurses_getvec
// fmt notcurses_init
// fm  notcurses_inputready_fd
// fm  notcurses_lex_blitter
// fm  notcurses_lex_margins
// fm  notcurses_lex_scalemode
// fm  notcurses_linesigs_disable
// fm  notcurses_linesigs_enable
// fm  notcurses_mice_enable
// rm  notcurses_mice_disable
// fm  notcurses_osversion
// fm  notcurses_palette_size
// fm  notcurses_refresh
// rm  notcurses_render
// fm  notcurses_stats
// fm  notcurses_stats_alloc
// fm  notcurses_stats_reset
// fm  notcurses_stdplane
// fm  notcurses_stdplane_const
// fmt notcurses_stop
// fm  notcurses_str_blitter
// fm  notcurses_str_scalemode
// fm  notcurses_supported_styles
// rm  notcurses_top
//X    notcurses_ucs32_to_utf8 (not needed in rust)
// fmt notcurses_version
// fm  notcurses_version_components
// rmt notcurses_align
// rm  notcurses_canpixel
// rm  notcurses_get_blocking
// rm  notcurses_get_nblock
//~r   notcurses_stddim_yx           // multiple mutable references errors
//~r   notcurses_stddim_yx_const     //
// rm  notcurses_term_dim_yx

mod methods;

pub(crate) mod helpers;
pub(crate) mod options;
pub(crate) mod reimplemented;

#[cfg(test)]
mod test;

pub use options::{NcFlag, NcOptions, NcOptionsBuilder};

/// Notcurses state for a given terminal, composed of [`NcPlane`]s.
///
/// It's built atop the terminfo abstraction layer to provide reasonably
/// portable vivid character displays.
///
/// [`NcPlane`]: crate::NcPlane
pub type Nc = crate::c_api::ffi::notcurses;
