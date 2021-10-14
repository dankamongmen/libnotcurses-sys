// A briefer way of creating an `NcSelector` by using the builder pattern.
//
use libnotcurses_sys::{widgets::*, *};

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new_cli()?;

    let plane = NcPlane::new_bound(nc.stdplane(), 4, 4, 10, 10)?;
    plane.set_base("·", NcStyle::NOSTYLE, NcChannels::from_rgb(0x666633, 0x444411))?;

    let selector = NcSelector::builder()
        .title("title")
        .secondary("secondary")
        .footer("footer")
        .item("1", "bla")
        .item("2", "ble")
        .item("3", "bli")
        .item("4", "blo")
        .item("5", "blu")
        .max_display(4)
        .item_channels(
            NcChannels::from_rgb(0x880000, 0x110000),
            NcChannels::from_rgb(0x008800, 0x002200),
        )
        .box_channels(NcChannels::combine(NcChannel::from_rgb(0x4488BB), NcChannel::with_default()))
        .title_channels(NcChannels::from_rgb(0xeaeaea, 0x334455))
        .secondary_channels(NcChannels::from_rgb(0x4477CC, 0x113322))
        .finish(plane)?;

    // Note: after this point the plane can't be mutated anymore,
    // because it would incurr in multiple mutable borrows.
    // plane.set_base(" ", 0, 0)?;

    nc.render()?;
    sleep![1];

    selector.destroy()?;
    nc.stop()?;
    Ok(())
}
