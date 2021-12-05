//! `input` example

#[cfg(any(test, doc))]
use crate::*;
#[cfg(not(any(test, doc)))]
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::with_flags(NcOptions::SUPPRESS_BANNERS)? };
    let splane = unsafe { nc.stdplane() };
    splane.set_scrolling(true);

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
                putstrln!(
                    splane,
                    "char: '{0}' \n{1:?} {2:?}\n",
                    ch,
                    input,
                    input.char()
                )?;
            }
            NcReceived::Event(ev) => {
                putstrln!(
                    splane,
                    "event: {0:?}\n  {1:?} {2:?}\n",
                    ev.name(),
                    input,
                    input.char()
                )?;
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

    unsafe { nc.stop()? };
    Ok(())
}
