// FIX memory leak with passed pointers to const* c_char

use libnotcurses_sys::{widgets::*, *};
use std::ffi::CString;

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new_cli()?;
    let plane = NcPlane::new_bound(nc.stdplane(), 0, 0, 10, 10)?;

    let str_1 = CString::new("temp_string1").unwrap();
    let str_2 = CString::new("temp_string2").unwrap();

    let item1 = NcSelectorItem {
        option: str_1.as_ptr(),
        desc: str_2.as_ptr(),
    };
    let selector_items = vec![item1, NcSelectorItem::new_empty()];

    let seloptions = NcSelectorOptions {
        title: str_1.as_ptr(),
        secondary: str_1.as_ptr(),
        footer: str_1.as_ptr(),

        items: selector_items.as_ptr(),
        defidx: 0,
        maxdisplay: 0,
        opchannels: 0,
        descchannels: 0,
        titlechannels: 0,
        footchannels: 0,
        boxchannels: 0,
        flags: 0,
    };
    let selector = NcSelector::new(plane, &seloptions)?;

    nc.refresh()?;
    nc.render()?;
    sleep![1];

    selector.destroy()?;
    nc.stop()?;
    Ok(())
}
