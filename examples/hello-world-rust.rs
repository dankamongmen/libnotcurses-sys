use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let plane = unsafe { nc.stdplane() };
    plane.putstr("hello world")?;
    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
