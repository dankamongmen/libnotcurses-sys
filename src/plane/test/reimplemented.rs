//! Test `ncplane_*` reimplemented functions.

use crate::c_api::{self, ncplane_new_test, notcurses_init_test, notcurses_stop};
use serial_test::serial;

#[test]
#[serial]
fn ncplane_notcurses() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        let nc2 = c_api::ncplane_notcurses(plane);
        assert_eq![nc as *mut _, nc2];

        let nc3 = c_api::ncplane_notcurses_const(plane);
        assert_eq![nc as *const _, nc3];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_cursor() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        let (mut y, mut x) = (0, 0);
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
        assert_eq![x, 0];
        assert_eq![y, 0];

        let res = c_api::ncplane_cursor_move_yx(plane, 10, 15);
        assert_eq![res, 0];
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
        assert_eq![x, 15];
        assert_eq![y, 10];

        c_api::ncplane_home(plane);
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
        assert_eq![x, 0];
        assert_eq![y, 0];

        let _res = c_api::ncplane_cursor_move_yx(plane, 10, 15);
        c_api::ncplane_erase(plane); // has to move the cursor to 0,0
        c_api::ncplane_cursor_yx(plane, &mut y, &mut x);
        assert_eq![x, 0];
        assert_eq![y, 0];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_channels() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        let channels = c_api::ncplane_channels(plane);
        assert_eq![channels, 0];

        c_api::ncplane_set_channels(plane, 0x1122334455667788);
        assert_eq![0x1122334455667788, c_api::ncplane_channels(plane)];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_fchannel() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        c_api::ncplane_set_channels(plane, 0x1122334455667788);
        let channels = c_api::ncplane_channels(plane);
        assert_eq![0x11223344, c_api::ncchannels_fchannel(channels)];

        let channels = c_api::ncplane_set_fchannel(plane, 0x10203040);
        assert_eq![0x10203040, c_api::ncchannels_fchannel(channels)];
        assert_eq![0x1020304055667788, channels];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_bchannel() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        c_api::ncplane_set_channels(plane, 0x1122334455667788);
        let channels = c_api::ncplane_channels(plane);
        assert_eq![0x55667788, c_api::ncchannels_bchannel(channels)];

        // BUG? ncplane_set_bchannel and ncplane_set_fchannel don't get
        // applied unless they are assigned to a variable. Weird.

        let channels = c_api::ncplane_set_bchannel(plane, 0x50607080);
        assert_eq![0x50607080, c_api::ncchannels_bchannel(channels)];
        assert_eq![0x1122334450607080, channels];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_rgb() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        c_api::ncplane_set_fg_rgb(plane, 0x112233);
        assert_eq![0x112233, c_api::ncplane_fg_rgb(plane)];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_default() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);
        assert_eq![true, c_api::ncplane_bg_default_p(plane)];
        assert_eq![true, c_api::ncplane_fg_default_p(plane)];

        c_api::ncplane_set_bg_rgb8(plane, 11, 22, 33);
        c_api::ncplane_set_fg_rgb8(plane, 44, 55, 66);
        assert_eq![false, c_api::ncplane_bg_default_p(plane)];
        assert_eq![false, c_api::ncplane_fg_default_p(plane)];

        c_api::ncplane_set_bg_default(plane);
        c_api::ncplane_set_fg_default(plane);
        assert_eq![true, c_api::ncplane_bg_default_p(plane)];
        assert_eq![true, c_api::ncplane_fg_default_p(plane)];

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_dimensions() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 10, 20);

        let (mut y, mut x) = (0, 0);
        c_api::ncplane_dim_yx(plane, &mut y, &mut x);
        assert_eq!((10, 20), (y, x));

        assert_eq!(10, c_api::ncplane_dim_y(plane));
        assert_eq!(20, c_api::ncplane_dim_x(plane));

        notcurses_stop(nc);
    }
}

#[test]
#[serial]
fn ncplane_resize() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        let res = c_api::ncplane_resize_simple(plane, 40, 40);
        assert_eq![c_api::NCRESULT_OK, res];

        let (mut y, mut x) = (0, 0);
        c_api::ncplane_dim_yx(plane, &mut y, &mut x);
        assert_eq!((40, 40), (y, x));

        // TODO: test further plane subset keeping unchanged features
        let res = c_api::ncplane_resize(plane, 0, 0, 0, 0, 0, 0, 60, 70);
        assert_eq![c_api::NCRESULT_OK, res];

        assert_eq!(60, c_api::ncplane_dim_y(plane));
        assert_eq!(70, c_api::ncplane_dim_x(plane));

        notcurses_stop(nc);
    }
}

// TODO: resizecb

#[test]
#[serial]
// TODO: CHECK: zeroes out every cell of the plane, dumps the egcpool,
// The base cell is preserved.
fn ncplane_erase() {
    unsafe {
        let nc = notcurses_init_test();
        let plane = ncplane_new_test(nc, 0, 0, 20, 20);

        c_api::ncplane_set_bg_rgb(plane, 0x112233);
        c_api::ncplane_set_fg_rgb(plane, 0x445566);
        assert_eq![false, c_api::ncplane_bg_default_p(plane)];
        assert_eq![false, c_api::ncplane_fg_default_p(plane)];

        // FIXME? DEBUG
        c_api::ncplane_erase(plane);
        // assert_eq![true, c_api::ncplane_bg_default_p(plane)];
        // assert_eq![true, c_api::ncplane_fg_default_p(plane)];
        //print!(" C: {:#0x} ", c_api::ncplane_channels(plane));

        notcurses_stop(nc);
    }
}

// #[test]
// #[serial]
// fn ncplane_at_cursor() {
//     unsafe {
//         let nc = notcurses_init_test();
//         let plane = ncplane_new_test(nc, 0, 0, 20, 20);
//
//         notcurses_stop(nc);
//     }
// }
//
// #[test]
// #[serial]
// fn ncplane_at_cursor_cell() {
//     unsafe {
//         let nc = notcurses_init_test();
//         let plane = ncplane_new_test(nc, 0, 0, 20, 20);
//
//         notcurses_stop(nc);
//     }
// }
