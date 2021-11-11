//! Input example
//!
//! https://github.com/dankamongmen/notcurses/blob/master/USAGE.md#input

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::with_flags(NcOptions::SUPPRESS_BANNERS | NcOptions::NO_WINCH_SIGHANDLER)?;
    nc.stdplane().set_scrolling(true);
    let splane = nc.stdplane();

    putstrln!(splane, "Input example.\nPress any key to continue:")?;
    nc.render()?;
    let rec = nc.get_blocking(None)?;
    putstrln!(splane, "Received: {:?}\n", rec)?;

    putstrln!(
        splane,
        "Press more keys to see their input. You can exit with F1.\n"
    )?;

    let mut input = NcInput::new_empty();
    loop {
        let rec = nc.get_nblock(Some(&mut input))?;
        match rec {
            NcReceived::Char(ch) => {
                putstrln!(splane, "char: '{0}' \n{1:?}\n", ch, input)?;
            }
            NcReceived::Event(ev) => {
                putstrln!(splane, "event: {0:?} \n{1:?}\n", ev.name(), input)?;

                match ev {
                    NcKey::F01 => break,
                    _ => (),
                }
            }
            NcReceived::Other(o) => {
                putstrln!(
                    splane,
                    "other (this shouldn't happen): {0:?} \n{1:?}\n",
                    o,
                    input
                )?;
            }
            _ => (),
        }
    }

    nc.stop()?;
    Ok(())
}
