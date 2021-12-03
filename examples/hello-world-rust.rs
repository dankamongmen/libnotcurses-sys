use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    nc.stdplane().putstr("hello world")?;
    nc.render()?;
    unsafe { nc.stop()? }
}
