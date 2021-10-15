//! `NcReader` widget.

// functions already exported by bindgen : 11
// ------------------------------------------
//  ncreader_clear
//  ncreader_contents
//  ncreader_create
//  ncreader_destroy
//  ncreader_move_down
//  ncreader_move_left
//  ncreader_move_right
//  ncreader_move_up
//  ncreader_offer_input
//  ncreader_plane
//  ncreader_write_egc

mod methods;

/// Provides a freeform input in a (possibly multiline) region
///
/// Supports optional readline keybindings (opt out using
/// `NCREADER_OPTION_NOCMDKEYS` flag)
///
/// Takes ownership of its [`NcPlane`][crate::NcPlane], destroying it on any
/// error (otherwise [`destroy`][NcReader#method.destroy] destroys the plane).
///
/// `type in C: ncreader (struct)`
pub type NcReader = crate::bindings::ffi::ncreader;

/// Options struct for [`NcReader`]
///
/// `type in C: ncreader_options (struct)`
pub type NcReaderOptions = crate::bindings::ffi::ncreader_options;

impl NcReaderOptions {
    /// Makes the terminal cursor visible across the lifetime of the ncreader, and
    /// have the ncreader manage the cursor's placement.
    pub const CURSOR: u32 = constants::NCREADER_OPTION_CURSOR;

    /// Enables horizontal scrolling. Virtual lines can then grow arbitrarily long.
    pub const HORSCROLL: u32 = constants::NCREADER_OPTION_HORSCROLL;

    /// Disables all editing shortcuts. By default, emacs-style keys are available.
    pub const NOCMDKEYS: u32 = constants::NCREADER_OPTION_NOCMDKEYS;

    /// Enables vertical scrolling. You can then use arbitrarily many virtual lines.
    pub const VERSCROLL: u32 = constants::NCREADER_OPTION_VERSCROLL;
}

pub(crate) mod constants {
    /// Makes the terminal cursor visible across the lifetime of the ncreader, and
    /// have the ncreader manage the cursor's placement.
    pub const NCREADER_OPTION_CURSOR: u32 = crate::bindings::ffi::NCREADER_OPTION_CURSOR;

    /// Enables horizontal scrolling. Virtual lines can then grow arbitrarily long.
    pub const NCREADER_OPTION_HORSCROLL: u32 = crate::bindings::ffi::NCREADER_OPTION_HORSCROLL;

    /// Disables all editing shortcuts. By default, emacs-style keys are available.
    pub const NCREADER_OPTION_NOCMDKEYS: u32 = crate::bindings::ffi::NCREADER_OPTION_NOCMDKEYS;

    /// Enables vertical scrolling. You can then use arbitrarily many virtual lines.
    pub const NCREADER_OPTION_VERSCROLL: u32 = crate::bindings::ffi::NCREADER_OPTION_VERSCROLL;
}
