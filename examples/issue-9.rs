//
// FIX memory leak while giving ownership of strings to C

use libnotcurses_sys::{
    c_api::libc::{free, strdup},
    widgets::*,
    *,
};
use std::{ffi::c_void, ptr::null_mut};

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new_cli()?;
    let plane = NcPlane::new(nc, 0, 0, 10, 10)?;

    // SETUP STRINGS
    //
    // <https://doc.rust-lang.org/std/ffi/struct.CString.html>
    //
    // 1. generate a C-compatible string from a Rust byte slice.
    // 2. the `as_ptr` method gives a `*const c_char` to feed functions that
    //    expect a nul-terminated string, like C’s `strdup`.
    // 3. `strdup` creates a duplicate of the string that gives a `*mut c_char`
    //    (that must be freed later).
    let str_title = std::ffi::CString::new("title").unwrap();
    let str_title_ptr = str_title.as_ptr();
    let str_title_dup = unsafe { strdup(str_title_ptr) };
    //
    // let str_option = std::ffi::CString::new("item_option").expect("CString::new");
    // let str_option_ptr = str_option.as_ptr();
    // let str_option_dup = unsafe { strdup(str_option_ptr)};

    // SETUP SELECTOR
    //
    let item1 = NcSelectorItem {
        option: null_mut(),
        // option: str_option_dup, // <-- ncselector_destroy doesn't free this
        desc: null_mut(),
        opcolumns: 0,
        desccolumns: 0,
    };
    let mut selector_items: [NcSelectorItem; 1] = [item1];
    let seloptions = NcSelectorOptions {
        // title: null_mut(),
        title: str_title_dup, // <-- ncselector_destroy doesn't free this

        secondary: null_mut(),
        footer: null_mut(),
        items: selector_items.as_mut_ptr(),
        defidx: 0,
        maxdisplay: 0,
        opchannels: 0,
        descchannels: 0,
        titlechannels: 0,
        footchannels: 0,
        boxchannels: 0,
        flags: 0,
    };
    let selector = NcSelector::new(plane, seloptions)?;

    // CLEANUP

    // plane.destroy()?; // NOTE: can't call plane destroy, valgrind complains
    unsafe {
        c_api::ncselector_destroy(selector, null_mut());
    };
    // plane.destroy()?; // also here!

    // NOTE: It seems I must always free the strings manually to avoid memleaks,
    // because ncselector_destroy() isn't taking care of it... I don't know why
    unsafe { free(str_title_dup as *mut c_void) };
    // unsafe { free(str_option_dup as *mut c_void) };

    nc.stop()?;
    Ok(())
}
