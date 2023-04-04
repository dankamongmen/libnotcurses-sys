//! Example 'direct-cursor'
//!
//! Explore cursor functions in direct mode
//!

use libnotcurses_sys::*;
use rand::{thread_rng, Rng};
use std::{thread::sleep, time::Duration};

fn main() -> NcResult<()> {
    let mut rng = thread_rng();

    let ncd = unsafe { NcDirect::new()? };

    let cols = ncd.dim_x();
    let rows = ncd.dim_y();
    println!("terminal size (rows, cols): {}, {}", rows, cols);

    let mut channels =
        NcChannels::combine(NcChannel::from_rgb(0xAA2244), NcChannel::from_rgb(0x112233));
    ncd.putstr(channels, "The current coordinates are")?;

    for _n in 0..40 {
        ncd.flush()?;
        sleep(Duration::from_millis(30));
        channels.set_fg_rgb([
            rng.gen_range(0x66..=0xEE),
            rng.gen_range(0x66..=0xEE),
            rng.gen_range(0x66..=0xEE),
        ]);
        channels.set_bg_rgb([
            rng.gen_range(0..=0x9),
            rng.gen_range(0..=0x9),
            rng.gen_range(0..=0x9),
        ]);
        ncd.putstr(channels, ".")?;
    }

    let (cy, cx) = ncd.cursor_yx()?;
    ncd.putstr(channels, &format!(" ({},{})\n", cy, cx))?;
    sleep(Duration::from_millis(1000));

    let sentence = vec![
        "And", "now", "I", "will", "clear", "the", "screen", ".", ".", ".",
    ];
    for word in sentence {
        channels.set_fg_rgb(channels.fg_rgb().0.wrapping_sub(0x050505));
        channels.set_bg_rgb(channels.bg_rgb().0.wrapping_add(0x090909));
        ncd.putstr(channels, &format!["{} ", word])?;
        ncd.flush()?;
        sleep(Duration::from_millis(150));
    }
    sleep(Duration::from_millis(300));
    channels.set_fg_rgb(0xFFFFFF);
    channels.set_bg_default();
    ncd.putstr(channels, "\nbye!\n\n")?;
    ncd.flush()?;
    sleep(Duration::from_millis(600));
    ncd.clear()?;
    unsafe { ncd.stop()? };
    Ok(())
}
