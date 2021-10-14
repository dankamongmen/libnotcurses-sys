// FIX NcSelector
//
// NOTE: strings must be associated to variables because lifetimes

use libnotcurses_sys::{widgets::*, *};

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new_cli()?;
    // let plane = NcPlane::new_bound(nc.stdplane(), 10, 0, 10, 10)?;
    let plane = NcPlane::new(nc, 10, 0, 10, 10)?;

    // items

    let item1_optn = NcString::new("item1");
    let item1_desc = NcString::new("desc1");
    let item2_optn = NcString::new("item2");
    let item2_desc = NcString::new("desc2");

    let item1 = NcSelectorItem::new(&item1_optn, &item1_desc);
    let item2 = NcSelectorItem::new(&item2_optn, &item2_desc);
    let selector_items = vec![item1, item2, NcSelectorItem::new_empty()];

    // options

    let title_str = NcString::new("title");
    let secondary_str = NcString::new("secondary");
    let footer_str = NcString::new("footer");
    let seloptions = NcSelectorOptions::with_all_options(
        &title_str,
        &secondary_str,
        &footer_str,
        &selector_items,
        0,
        2,
        0,
        0,
        0,
        0,
        0,
    );

    let selector = NcSelector::new(plane, &seloptions)?;

    // nc.refresh()?;
    // nc.render()?;
    plane.render()?;
    plane.rasterize()?;
    sleep![1];

    selector.destroy()?;
    nc.stop()?;
    Ok(())
}
