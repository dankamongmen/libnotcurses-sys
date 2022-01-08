use core::ptr::{null, null_mut};
use std::process::exit;

use libnotcurses_sys::c_api::*;

fn main() {
    let options = ffi::notcurses_options {
        termtype: null(),
        loglevel: 0,
        margin_t: 0,
        margin_r: 0,
        margin_b: 0,
        margin_l: 0,
        flags: NCOPTION_CLI_MODE,
    };

    unsafe {
        let nc = notcurses_init(&options, null_mut());
        if nc.is_null() {
            exit(1);
        }
        let plane = notcurses_stdplane(nc);
        let cols = ncplane_putstr(&mut *plane, "hello world");

        if cols < NCRESULT_OK {
            notcurses_stop(nc);
            exit(cols.abs());
        }
        if notcurses_render(&mut *nc) < NCRESULT_OK {
            exit(2);
        }
        if notcurses_stop(nc) < NCRESULT_OK {
            exit(3);
        }
    }
}
