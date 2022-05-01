//! `input` example

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::with_flags(NcFlag::SuppressBanners)? };

    let splane = unsafe { nc.stdplane() };
    splane.set_scrolling(true);

    nc.mice_enable(NcMiceEvents::All)?;

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
            NcReceived::Key(key) => {
                putstrln!(
                    splane,
                    "key: {0:?}\n  {1:?} {2:?}\n",
                    key.name(),
                    input,
                    input.char()
                )?;
                match key {
                    NcKey::F01 => break,
                    _ => (),
                }
            }
            NcReceived::NoInput => (),
        }
    }

    unsafe { nc.stop()? };
    Ok(())
}
