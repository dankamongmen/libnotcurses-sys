//! `NcTabbed` widget.

// functions already exported by bindgen : 39
// ------------------------------------------
//   nctab_cb
//   nctab_move
//   nctab_move_left
//   nctab_move_right
//   nctab_name
//   nctab_name_width
//   nctab_next
//   nctab_prev
//   nctab_set_cb
//   nctab_set_name
//   nctab_set_userptr
//   nctab_userptr
//   nctabbed_add
//   nctabbed_channels
//   nctabbed_content_plane
//   nctabbed_create
//   nctabbed_del
//   nctabbed_destroy
//   nctabbed_ensure_selected_header_visible
//   nctabbed_leftmost
//   nctabbed_next
//   nctabbed_plane
//   nctabbed_prev
//   nctabbed_redraw
//   nctabbed_rotate
//   nctabbed_select
//   nctabbed_selected
//   nctabbed_separator
//   nctabbed_separator_width
//   nctabbed_set_hdrchan
//   nctabbed_set_selchan
//   nctabbed_set_separator
//   nctabbed_set_sepchan
//   nctabbed_tabcount
//   nctablet_ncplane

// mod methods;

/// A tab for [`NcTabbed`].
///
/// `type in C: nctab (struct)`
pub type NcTab = crate::bindings::ffi::nctab;

/// Tabbed widgets.
///
/// The tab list is displayed at the top or at the bottom of the plane,
/// and only one tab is visible at a time.
///
/// `type in C: nctabbed (struct)`
pub type NcTabbed = crate::bindings::ffi::nctabbed;

/// Options for [`NcTabbed`].
///
/// `type in C: nctabbed_options (struct)`
pub type NcTabbedOptions = crate::bindings::ffi::nctabbed_options;

impl NcTabbedOptions {
    /// To display the tab list at the bottom instead of at the top of the plane.
    pub const BOTTOM: u32 = constants::NCTABBED_OPTION_BOTTOM;
}

pub(crate) mod constants {
    /// To display the tab list at the bottom instead of at the top of the plane.
    pub const NCTABBED_OPTION_BOTTOM: u32 = crate::bindings::ffi::NCTABBED_OPTION_BOTTOM;
}
