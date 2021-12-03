use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let stdp = unsafe { nc.stdplane() };
    stdp.putstr("hello world")?;
    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
