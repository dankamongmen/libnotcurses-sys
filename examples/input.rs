//! Input example
//!
//! https://github.com/dankamongmen/notcurses/blob/master/USAGE.md#input

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::with_flags(
        NcOptions::SUPPRESS_BANNERS | NcOptions::NO_WINCH_SIGHANDLER | NcOptions::NO_QUIT_SIGHANDLERS,
    )?;
    nc.stdplane().set_scrolling(true);
    let splane = nc.stdplane();

    putstrln!(splane, "Input example.\nPress any key to continue:")?;
    nc.render()?;
    let key = nc.get_blocking(None)?;
    putstrln!(splane, "Pressed: {}\n", key)?;

    putstrln!(
        splane,
        "Press more keys to see their input. You can exit with F1.\n"
    )?;
    let mut input = NcInput::new_empty();
    loop {
        let key = nc.get_nblock(Some(&mut input))?;
        match key {
            NcKey::F01 => break,
            NcKey::ESC..=NcKey::BUTTON11 => {
                putstrln!(
                    splane,
                    "pressed: '{0}' ({1:x})\n{2:?}\n",
                    key,
                    key as u32,
                    input
                )?;
            }
            _ => (),
        }
    }

    nc.stop()?;
    Ok(())
}
