use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let stdplane = unsafe { nc.stdplane() };
    stdplane.putstr("hello world")?;
    nc.render()?;
    unsafe { nc.stop()? }; // always stop notcurses before exiting
    Ok(())
}
