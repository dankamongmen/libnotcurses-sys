// without an alternate screen the cursor is not restored afterwards
use libnotcurses_sys::*;
fn main() -> NcResult<()> {
    let nc = Nc::with_flags(NCOPTION_NO_ALTERNATE_SCREEN)?;
    nc.stop()?;
    Ok(())
}
