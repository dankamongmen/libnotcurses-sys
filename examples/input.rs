//! Input example
//!
//! https://github.com/dankamongmen/notcurses/blob/master/USAGE.md#input
//!

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::with_flags(
        NCOPTION_SUPPRESS_BANNERS | NCOPTION_NO_WINCH_SIGHANDLER | NCOPTION_NO_QUIT_SIGHANDLERS,
    )?;

    println!("Exit with F1\n");

    let mut input = NcInput::new_empty();

    loop {
        match nc.getc_nblock(Some(&mut input)) {
            Ok(key) => match key {
                NCKEY_F01 => break,
                NCKEY_ESC..=NCKEY_RELEASE => {
                    println!("'{0}' ({1:x})\n{2:?}", key, key as u32, input);
                }
                _ => (),
            },
            Err(err) => return Err(err),
        }
    }

    println!("\nExiting...");
    nc.stop()?;
    Ok(())
}
