//! https://github.com/dankamongmen/notcurses/issues/2169
//!
//! blocking input doesn't work

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::without_altscreen()?;

    /*
    // blocking works:

    // TEST_1: manual OK
    println!("press a key:");
    let mut input1 = NcInput::new_empty();
    let key = unsafe {
        notcurses_get(nc, std::ptr::null(), &mut input1)
    };
    println!("{} → {}", key, unsafe {core::char::from_u32_unchecked(key)} );

    // TEST_2: wrapped OK
    println!("press a key:");
    let mut input2 = NcInput::new_empty();
    let key = notcurses_getc_blocking(nc, Some(&mut input2));
    println!("{}", key);
    */

    // non-blocking doesn't work

    // TEST_3: manual BUG
    let mut input3 = NcInput::new_empty();
    let ts = ffi::timespec { tv_sec: 0, tv_nsec: 0 };
    loop {
        println!("before nblock");
        let key = unsafe {
            notcurses_get(nc, &ts, &mut input3)
        };
        println!("after nblock. key = `{}` ({})", key, key as i32);

        match key {
            032..=127 => break,
            _ => (),
        }
        sleep![0, 100];
    }

    // // TEST_4: wrapped BUG
    // let mut input4 = NcInput::new_empty();
    // loop {
    //     println!("before nblock");
    //     let key = notcurses_getc_nblock(nc, &mut input4); // <PROBLEM
    //     println!("after nblock. key = `{}`", key);
    //
    //     match key {
    //         'q' => break,
    //         _ => (),
    //     }
    //     sleep![0, 100];
    // }

    nc.stop()?;
    Ok(())
}
