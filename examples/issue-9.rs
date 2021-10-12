use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc: &mut Nc = Nc::new()?;

    let _i = widgets::NcSelectorItem::new("XXX", "YYY");

    nc.stop()?;

    Ok(())
}
