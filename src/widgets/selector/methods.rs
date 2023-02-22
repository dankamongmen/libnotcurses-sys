use core::{
    ffi::c_char,
    ptr::{null, null_mut},
};

use crate::{
    c_api, cstring, error, error_ref_mut, error_str,
    widgets::{NcSelector, NcSelectorBuilder, NcSelectorItem, NcSelectorOptions},
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

    /// Starts the builder.
    pub fn builder() -> NcSelectorBuilder {
        NcSelectorBuilder::new()
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
    pub fn offer_input(&mut self, input: impl Into<NcInput>) -> bool {
        unsafe { c_api::ncselector_offer_input(self, &input.into()) }
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
        let cs = cstring![item];
        error![
            unsafe { c_api::ncselector_delitem(self, cs.as_ptr()) },
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
            None
        } else {
            Some(crate::rstring!(res).to_string())
        }
    }

    // NOTE: too unsafe
    // /// Return a reference to the ncselector's underlying ncplane.
    // pub fn plane<'a>(&mut self) -> NcResult<&'a mut NcPlane> {
    //     error_ref_mut![unsafe { c_api::ncselector_plane(self) }, "Calling selector.plane"]
    // }

    /// Move down in the list. A reference to the newly-selected item is
    /// returned, or NULL if there are no items in the list.
    pub fn nextitem(&mut self) -> NcResult<String> {
        let cstr: *const c_char = unsafe { c_api::ncselector_nextitem(self) };
        error_str![cstr, "Calling selector.nextitem"]
    }

    /// Move up in the list. A reference to the newly-selected item is
    /// returned, or NULL if there are no items in the list.
    pub fn previtem(&mut self) -> NcResult<String> {
        let cstr: *const c_char = unsafe { c_api::ncselector_previtem(self) };
        error_str![cstr, "Calling selector.previtem"]
    }
}

impl NcSelectorItem {
    /// New item
    pub fn new(option: &NcString, desc: &NcString) -> Self {
        Self { option: option.as_ptr(), desc: desc.as_ptr() }
    }

    /// New empty `NcSelectorItem`.
    pub fn new_empty() -> Self {
        Self { option: null(), desc: null() }
    }
}

/// # `NcMenuOptions` constructors
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
        title: Option<&NcString>,
        secondary: Option<&NcString>,
        footer: Option<&NcString>,
        items: &[NcSelectorItem],
        default: u32,
        max_display: u32,
        opchannels: impl Into<NcChannels>,
        descchannels: impl Into<NcChannels>,
        titlechannels: impl Into<NcChannels>,
        footchannels: impl Into<NcChannels>,
        boxchannels: impl Into<NcChannels>,
    ) -> Self {
        assert![!items.is_empty()]; // DEBUG

        let title_ptr = if let Some(s) = title { s.as_ptr() } else { null() };
        let secondary_ptr = if let Some(s) = secondary { s.as_ptr() } else { null() };
        let footer_ptr = if let Some(s) = footer { s.as_ptr() } else { null() };

        Self {
            title: title_ptr,
            secondary: secondary_ptr,
            footer: footer_ptr,
            // initial items and descriptions,
            items: items.as_ptr(),
            // default item
            defidx: default,
            // maximum number of options to display at once,
            // 0 to use all available space
            maxdisplay: max_display,
            // exhaustive styling options
            opchannels: opchannels.into().into(),
            descchannels: descchannels.into().into(),
            titlechannels: titlechannels.into().into(),
            footchannels: footchannels.into().into(),
            boxchannels: boxchannels.into().into(),
            flags: 0x0,
        }
    }
}
