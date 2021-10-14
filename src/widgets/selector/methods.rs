use core::ptr::{null, null_mut};

use crate::{
    c_api, cstring, error, error_ref_mut, error_str,
    widgets::{NcSelector, NcSelectorItem, NcSelectorOptions},
    NcChannels, NcInput, NcPlane, NcResult, NcString,
};

impl NcSelector {
    /// Creates a selector over a `plane`.
    ///
    /// The selector will take care of destroying the plane.
    pub fn new<'a>(plane: &mut NcPlane, options: &NcSelectorOptions) -> NcResult<&'a mut Self> {
        error_ref_mut![
            unsafe { c_api::ncselector_create(plane, options) },
            "ncselector_create"
        ]
    }

    /// Offers an input to the selector.
    ///
    /// If it's relevant, this function returns true, and the input ought not be
    /// processed further. If it's irrelevant to the selector, false is returned.
    ///
    /// Relevant inputs include:
    /// - a mouse click on an item.
    /// - a mouse scrollwheel event.
    /// - a mouse click on the scrolling arrows.
    /// - up, down, pgup, or pgdown on an unrolled menu (navigates among items).
    pub fn offer_input(&mut self, input: NcInput) -> bool {
        unsafe { c_api::ncselector_offer_input(self, &input) }
    }

    /// Destroys the `NcSelector`.
    ///
    /// Note that this also destroys the [`NcPlane`].
    //
    // If 'item' is not NULL, the last selected option will
    // be strdup()ed and assigned to '*item' (and must be free()d by the caller).
    pub fn destroy(&mut self) -> NcResult<()> {
        unsafe { c_api::ncselector_destroy(self, null_mut()) };
        Ok(())
    }

    /// Adds an item.
    //
    // CHECK whether this works for multiple items.
    pub fn additem(&mut self, item: NcSelectorItem) -> NcResult<i32> {
        error![
            unsafe { c_api::ncselector_additem(self, &item) },
            "Calling selector.additem", -1
        ]
    }

    /// Deletes an item.
    ///
    /// *C style function: [ncselector_delitem()][c_api::ncselector_delitem].*
    pub fn delitem(&mut self, item: &str) -> NcResult<i32> {
        error![
            unsafe { c_api::ncselector_delitem(self, cstring![item]) },
            "Calling selector.delitem", -1
        ]
    }

    /// Returns the selected option if there is one.
    ///
    /// *C style function: [ncselector_delitem()][c_api::ncselector_delitem].*
    pub fn selected(&mut self) -> Option<String> {
        // MAYBE turn this into a macro (option_str![])
        let res = unsafe { c_api::ncselector_selected(self) };
        if res.is_null() {
            Some(crate::rstring!(res).to_string())
        } else {
            None
        }
    }

    // too unsafe
    // /// Return a reference to the ncselector's underlying ncplane.
    // pub fn plane<'a>(&mut self) -> NcResult<&'a mut NcPlane> {
    //     error_ref_mut![unsafe { c_api::ncselector_plane(self) }, "Calling selector.plane"]
    // }

    /// Move down in the list. A reference to the newly-selected item is
    /// returned, or NULL if there are no items in the list.
    pub fn nextitem(&mut self) -> NcResult<String> {
        let cstr: *const i8 = unsafe { c_api::ncselector_nextitem(self) };
        error_str![cstr, "Calling selector.nextitem"]
    }

    /// Move up in the list. A reference to the newly-selected item is
    /// returned, or NULL if there are no items in the list.
    pub fn previtem(&mut self) -> NcResult<String> {
        let cstr: *const i8 = unsafe { c_api::ncselector_previtem(self) };
        error_str![cstr, "Calling selector.previtem"]
    }
}

impl NcSelectorItem {
    /// New item
    pub fn new(option: &NcString, desc: &NcString) -> Self {
        Self {
            option: option.as_ptr(),
            desc: desc.as_ptr(),
        }
    }

    /// New empty `NcSelectorItem`.
    pub fn new_empty() -> Self {
        Self {
            option: null(),
            desc: null(),
        }
    }

    // DOES NOT WORK
    // pub fn new_ncstring(option: NcString, desc: NcString) -> Self {
    //     Self {
    //         option: option.as_ptr(),
    //         desc: desc.as_ptr(),
    //     }
    // }

    // DOES NOT WORK
    // pub fn new_str(option: &'static str, desc: &'static str) -> Self {
    //     use std::ffi::CString;
    //     Self {
    //         option: CString::new(option).unwrap().as_ptr(),
    //         desc: CString::new(desc).unwrap().as_ptr(),
    //     }
    // }
}

// /// A null-terminated list of [`NcSelectorItem`].
// ///
// /// This makes sure the list always ends with an empty item.
// impl NcSelectorItems {
//     /// Creates a list of items from a vector of items.
//     // MAYBE: accept a slice
//     pub fn new(items: Vec<NcSelectorItem>) -> Self {
//         let mut res = Self { items };
//         res.add_empty_item();
//         res
//     }
//
//     /// Creates an empty list of items.
//     pub fn new_empty() -> Self {
//         Self {
//             items: vec![NcSelectorItem::new_empty()],
//         }
//     }
//
//     /// Adds a new `NcSelectorItem` to the list.
//     pub fn add(&mut self, item: NcSelectorItem) {
//         self.items.pop();
//         self.items.push(item);
//         self.add_empty_item();
//     }
//
//     // /// Adds a new NcSelectorItem from its constituent components.
//     // pub fn add_components(&mut self, item_option: &str, item_desc: &str) {
//     //     self.items.pop();
//     //     self.items.push(NcSelectorItem::new(item_option, item_desc));
//     //     self.add_empty_item();
//     // }
//
//     // ///
//     // // CHECK: is private ok?
//     // fn as_mut_ptr(&mut self) -> *mut NcSelectorItem {
//     //     self.items.as_mut_ptr()
//     // }
//
//     fn add_empty_item(&mut self) {
//         self.items.push(NcSelectorItem::new_empty())
//     }
// }

/// # `NcMenuOptions` constructors
//
// TODO:IMPROVE:
// - `title` may be null, inhibiting riser, saving two rows.
// - `secondary` may be null
// - `footer` may be null
impl NcSelectorOptions {
    /// New `NcSelectorOptions` with just the list of items.
    pub fn new(items: &[NcSelectorItem]) -> Self {
        Self {
            title: null(),
            secondary: null(),
            footer: null(),
            items: items.as_ptr(),
            defidx: 0,
            maxdisplay: 0,
            opchannels: 0,
            descchannels: 0,
            titlechannels: 0,
            footchannels: 0,
            boxchannels: 0,
            flags: 0,
        }
    }

    /// New `NcSelectorOptions` with all options.
    pub fn with_all_options(
        title: &NcString,
        secondary: &NcString,
        footer: &NcString,
        items: &[NcSelectorItem],
        default: u32,
        max_display: u32,
        opchannels: NcChannels,
        descchannels: NcChannels,
        titlechannels: NcChannels,
        footchannels: NcChannels,
        boxchannels: NcChannels,
    ) -> Self {
        assert![!items.is_empty()];
        Self {
            title: title.as_ptr(),
            secondary: secondary.as_ptr(),
            footer: footer.as_ptr(),

            // initial items and descriptions,
            items: items.as_ptr(),
            // default item
            defidx: default,
            // maximum number of options to display at once,
            // 0 to use all available space
            maxdisplay: max_display,
            // exhaustive styling options
            opchannels,
            descchannels,
            titlechannels,
            footchannels,
            boxchannels,
            flags: 0x0,
        }
    }
}
