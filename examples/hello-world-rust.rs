use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::new_cli()?;
    let plane = nc.stdplane();
    plane.putstr("hello world")?;
    nc.render()?;
    nc.stop()?;
    Ok(())
}
