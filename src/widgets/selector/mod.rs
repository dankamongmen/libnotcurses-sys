//! `NcSelector` widget.
//!                                 ╭──────────────────────────╮
//!                                 │This is the primary header│
//!   ╭──────────────────────this is the secondary header──────╮
//!   │        ↑                                               │
//!   │ option1 Long text #1                                   │
//!   │ option2 Long text #2                                   │
//!   │ option3 Long text #3                                   │
//!   │ option4 Long text #4                                   │
//!   │ option5 Long text #5                                   │
//!   │ option6 Long text #6                                   │
//!   │        ↓                                               │
//!   ╰────────────────────────────────────here's the footer───╯
//!
//! selection widget -- an ncplane with a title header and a body section. the
//! body section supports infinite scrolling up and down.
//!
//! At all times, exactly one item is selected.

use core::ptr::null_mut;

use crate::{
    c_api::{
        ncselector_additem, ncselector_create, ncselector_delitem, ncselector_destroy,
        ncselector_nextitem, ncselector_offer_input, ncselector_plane, ncselector_previtem,
        ncselector_selected,
    },
    cstring, cstring_mut, error, error_ref_mut, error_str, NcChannels, NcChannelsApi, NcInput,
    NcPlane, NcResult,
};

/// High-level widget for selecting one item from a set
pub type NcSelector = crate::bindings::ffi::ncselector;

/// Item structure for [`NcSelector`]
pub type NcSelectorItem = crate::bindings::ffi::ncselector_item;

/// Options structure for [`NcSelector`]
pub type NcSelectorOptions = crate::bindings::ffi::ncselector_options;

/// # `NcSelector` constructors & destructors
impl NcSelector {
    pub fn new<'a>(plane: &mut NcPlane, options: NcSelectorOptions) -> NcResult<&'a mut Self> {
        error_ref_mut![
            unsafe { ncselector_create(plane, &options) },
            "Creating NcSelector"
        ]
    }

    pub fn offer_input(&mut self, input: NcInput) -> bool {
        unsafe { ncselector_offer_input(self, &input) }
    }

    /// Destroy the ncselector. If 'item' is not NULL, the last selected option will
    /// be strdup()ed and assigned to '*item' (and must be free()d by the caller).
    pub fn destroy(&mut self) -> NcResult<()> {
        unsafe { ncselector_destroy(self, null_mut()) };
        Ok(())
    }

    /// Dynamically add items. It is usually sufficient to supply a static
    /// list of items via ncselector_options->items.
    pub fn additem(&mut self, item: NcSelectorItem) -> NcResult<i32> {
        error![
            unsafe { ncselector_additem(self, &item) },
            "Calling selector.additem", -1
        ]
    }

    /// Dynamically delete item
    // TODO API int ncselector_delitem(struct ncselector* n, const char* item);
    pub fn delitem(&mut self, item: &str) -> NcResult<i32> {
        error![
            unsafe { ncselector_delitem(self, cstring![item]) },
            "Calling selector.delitem", -1
        ]
    }

    /// Return reference to the selected option, or NULL if there are no items.
    pub fn selected(&mut self) -> NcResult<String> {
        error_str![
            unsafe { ncselector_selected(self) },
            "Calling selector.selected"
        ]
    }

    /// Return a reference to the ncselector's underlying ncplane.
    pub fn plane<'a>(&mut self) -> NcResult<&'a mut NcPlane> {
        error_ref_mut![unsafe { ncselector_plane(self) }, "Calling selector.plane"]
    }

    /// Move down in the list. A reference to the newly-selected item is
    /// returned, or NULL if there are no items in the list.
    pub fn nextitem(&mut self) -> NcResult<String> {
        let cstr: *const i8 = unsafe { ncselector_nextitem(self) };
        error_str![cstr, "Calling selector.nextitem"]
    }

    /// Move up in the list. A reference to the newly-selected item is
    /// returned, or NULL if there are no items in the list.
    pub fn previtem(&mut self) -> NcResult<String> {
        let cstr: *const i8 = unsafe { ncselector_previtem(self) };
        error_str![cstr, "Calling selector.previtem"]
    }
}

impl NcSelectorItem {
    pub fn new(option: &str, desc: &str) -> Self {
        Self {
            option: cstring_mut![option],
            desc: cstring_mut![desc],
            opcolumns: 0,
            desccolumns: 0,
        }
    }

    /// New empty NcMenuItem for [`NcMenu`][crate::widgets::NcMenu].
    pub fn new_empty() -> Self {
        Self {
            option: null_mut(),
            desc: null_mut(),
            opcolumns: 0,
            desccolumns: 0,
        }
    }
}

/// # `NcMenuOptions` constructors
impl NcSelectorOptions {
    /// New NcMenuOptions for [`crate::widgets::NcMenu`].
    ///
    /// `sections` must contain at least 1 [`NcMenuSection`][crate::widgets::NcMenuSection].
    pub fn new(
        title: &str,
        secondary: &str,
        footer: &str,
        selector_item: &mut [NcSelectorItem],
    ) -> Self {
        //assert![!selector_item.is_empty()];
        Self {
            title: cstring_mut![title], // title may be null, inhibiting riser, saving two rows.
            secondary: cstring_mut![secondary], // secondary may be null
            footer: cstring_mut![footer], // footer may be null

            items: selector_item.as_mut_ptr(), // initial items and descriptions
            defidx: 1,
            maxdisplay: 6, // maximum number of options to display at once, 0 to use all available space
            // exhaustive styling options
            opchannels: NcChannels::from_rgb8(0xe0, 0x80, 0x40, 0, 0, 0),
            descchannels: NcChannels::from_rgb8(0x80, 0xe0, 0x40, 0, 0, 0),
            titlechannels: NcChannels::from_rgb8(0xff, 0xff, 0x80, 0, 0, 0x20),
            footchannels: NcChannels::from_rgb8(0xe0, 0, 0x40, 0x20, 0, 0),
            boxchannels: NcChannels::from_rgb8(0x20, 0xe0, 0x40, 0x20, 0x20, 0x20),
            flags: 0x0,
        }
    }
}
