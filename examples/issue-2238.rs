// This doesn't exit properly when run inside screen or tmux
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::with_flags(NcOptions::NO_ALTERNATE_SCREEN)?;
    nc.stop()?;
    Ok(())
}
