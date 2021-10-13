//
// FIX memory leak while giving ownership of strings to C

use libnotcurses_sys::{
    c_api::libc::{free, strdup},
    widgets::*,
    *,
};
use std::{ffi::{CString, c_void}, ptr::null_mut};

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new_cli()?;
    let plane = NcPlane::new(nc, 0, 0, 10, 10)?;

    let mut option_str = NcString::new("option"); // <- the only one that leaks
    let mut desc_str = NcString::new("desc");
    let mut title_str = NcString::new("title");
    let mut secondary_str = NcString::new("secondary");
    let mut footer_str = NcString::new("footer");

    let item1 = NcSelectorItem {
        option: null_mut(),
        // option: option_str.as_mut_ptr(), // BUGFIX: error here

        desc: desc_str.as_mut_ptr(),
        opcolumns: 0,
        desccolumns: 0,
    };
    let mut selector_items: [NcSelectorItem; 1] = [item1];
    let seloptions = NcSelectorOptions {
        title: title_str.as_mut_ptr(),
        secondary: secondary_str.as_mut_ptr(),
        footer: footer_str.as_mut_ptr(),
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

    selector.destroy()?;
    // unsafe {
    //     c_api::ncselector_destroy(selector, null_mut());
    // };

    nc.stop()?;
    Ok(())
}
