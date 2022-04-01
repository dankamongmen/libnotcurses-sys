use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let stdplane = unsafe { nc.stdplane() };
    stdplane.putstr("\nhello world!\n")?;
    nc.render()?;
    unsafe { nc.stop()? }; // always stop before exiting
    Ok(())
}
