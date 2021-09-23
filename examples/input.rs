//! Input example
//!
//! https://github.com/dankamongmen/notcurses/blob/master/USAGE.md#input

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::with_flags(
        NCOPTION_SUPPRESS_BANNERS | NCOPTION_NO_WINCH_SIGHANDLER | NCOPTION_NO_QUIT_SIGHANDLERS,
    )?;

    println!("Input example.\nPress any key to continue:");
    let key = nc.getc_blocking(None)?;
    println!("Pressed: {}\n", key);

    println!("Press more keys to see their input. You can exit with F1.\n");
    let mut input = NcInput::new_empty();
    loop {
        let key = nc.getc_nblock(Some(&mut input))?;
        match key {
            NCKEY_F01 => break,
            NCKEY_ESC..=NCKEY_BUTTON11 => {
                println!("'{0}' ({1:x})\n{2:#?}\n", key, key as u32, input);
            }
            _ => (),
        }
    }

    nc.stop()?;
    Ok(())
}
