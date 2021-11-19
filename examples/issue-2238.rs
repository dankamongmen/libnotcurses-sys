// This doesn't exit properly when run inside screen or tmux
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::with_flags(NcOptions::NO_ALTERNATE_SCREEN)? };
    unsafe { nc.stop()? };
    Ok(())
}
