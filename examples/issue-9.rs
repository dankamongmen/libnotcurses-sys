// https://doc.rust-lang.org/std/ffi/struct.CString.html
//

use core::ptr::null_mut;
use libnotcurses_sys::{*, widgets::*};

// FIX memleak while using strings

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new()?;
    let plane = NcPlane::new(nc, 0,0,10,10)?;

    // WIP
    let option_str = std::ffi::CString::new("X").expect("option_str");

    let item1 = NcSelectorItem {

        // this is OK:
        option: null_mut(),
        // this causes memory leak
        // option: unsafe { c_api::libc::strdup(option_str.as_ptr()) },

        desc: null_mut(),
        opcolumns:0,
        desccolumns:0,
    };

    let mut selector_items: [NcSelectorItem; 1] = [
        item1,
    ];



    let seloptions = NcSelectorOptions {
        title: null_mut(),
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

    plane.destroy()?;

    selector.destroy()?;

    nc.stop()?;
    Ok(())
}
