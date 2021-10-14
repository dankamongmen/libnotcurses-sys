// FIX memory leak with passed pointers to const* c_char

use libnotcurses_sys::{
    widgets::*,
    *,
};

// CString implements a `as_ptr` method that will give you a `*const c_char`
// which you can feed directly to extern functions that expect a nul-terminated
// string, like C’s strdup(). Notice that as_ptr returns a read-only pointer;
// if the C code writes to it, that causes undefined behavior.

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new_cli()?;
    let plane = NcPlane::new(nc, 0, 0, 10, 10)?;

    let str_1 = std::ffi::CString::new("temp_string1").unwrap();

    let item1 = NcSelectorItem {
        // option: std::ptr::null(),
        option: str_1.as_ptr(), // BUG

        desc: str_1.as_ptr(),
    };
    // let mut selector_items: [NcSelectorItem; 1] = [item1];
    let mut selector_items = vec![item1];

    let seloptions = NcSelectorOptions {
        title: str_1.as_ptr(),
        secondary: str_1.as_ptr(),
        footer: str_1.as_ptr(),

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
    nc.stop()?;
    Ok(())
}
