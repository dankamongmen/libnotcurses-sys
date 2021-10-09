//! Test `cell*_*` reimplemented functions

use serial_test::serial;

use crate::{fns, NcCell};

#[test]
#[serial]
fn rgb() {
    // rgb

    let mut c1 = NcCell::new();
    assert_eq![0, fns::nccell_fg_rgb(&c1)];
    assert_eq![0, fns::nccell_bg_rgb(&c1)];

    fns::nccell_set_fg_rgb(&mut c1, 0x99112233);
    assert_eq![0x112233, fns::nccell_fg_rgb(&c1)];
    fns::nccell_set_bg_rgb(&mut c1, 0x99445566);
    assert_eq![0x445566, fns::nccell_bg_rgb(&c1)];

    // rgb8

    let mut c2 = NcCell::new();
    let (mut r, mut g, mut b) = (0, 0, 0);

    fns::nccell_set_fg_rgb8(&mut c2, 0x11, 0x22, 0x33);
    let fchannel = fns::nccell_fg_rgb8(&c2, &mut r, &mut g, &mut b);
    assert_eq!((0x11, 0x22, 0x33), (r, g, b));
    assert_eq![0x112233, fchannel & !crate::NCALPHA_BGDEFAULT_MASK];

    fns::nccell_set_bg_rgb8(&mut c2, 0x44, 0x55, 0x66);
    let bchannel = fns::nccell_bg_rgb8(&c2, &mut r, &mut g, &mut b);
    assert_eq!((0x44, 0x55, 0x66), (r, g, b));
    assert_eq![0x445566, bchannel & !crate::NCALPHA_BGDEFAULT_MASK];
}

#[test]
#[serial]
fn alpha() {
    let mut c1 = NcCell::new();
    assert_eq![0, fns::nccell_fg_alpha(&c1)];
    assert_eq![0, fns::nccell_bg_alpha(&c1)];

    fns::nccell_set_fg_alpha(&mut c1, crate::NCALPHA_TRANSPARENT);
    assert_eq![crate::NCALPHA_TRANSPARENT, fns::nccell_fg_alpha(&c1)];

    fns::nccell_set_bg_alpha(&mut c1, crate::NCALPHA_BLEND);
    assert_eq![crate::NCALPHA_BLEND, fns::nccell_bg_alpha(&c1)];
}

#[test]
#[serial]
fn default() {
    let mut c1 = NcCell::new();
    assert_eq![true, fns::nccell_fg_default_p(&c1)];
    assert_eq![true, fns::nccell_bg_default_p(&c1)];

    // rgb
    fns::nccell_set_fg_rgb(&mut c1, 0x112233);
    fns::nccell_set_bg_rgb(&mut c1, 0x445566);
    assert_eq![false, fns::nccell_fg_default_p(&c1)];
    assert_eq![false, fns::nccell_bg_default_p(&c1)];

    // reset
    fns::nccell_set_fg_default(&mut c1);
    fns::nccell_set_bg_default(&mut c1);
    assert_eq![true, fns::nccell_fg_default_p(&c1)];
    assert_eq![true, fns::nccell_bg_default_p(&c1)];

    // rgb8
    fns::nccell_set_fg_rgb8(&mut c1, 0x11, 0x22, 0x33);
    fns::nccell_set_bg_rgb8(&mut c1, 0x44, 0x55, 0x66);
    assert_eq![false, fns::nccell_fg_default_p(&c1)];
    assert_eq![false, fns::nccell_bg_default_p(&c1)];

    // reset
    fns::nccell_set_fg_default(&mut c1);
    fns::nccell_set_bg_default(&mut c1);

    // palette
    fns::nccell_set_fg_palindex(&mut c1, 5);
    fns::nccell_set_bg_palindex(&mut c1, 6);
    assert_eq![false, fns::nccell_fg_default_p(&c1)];
    assert_eq![false, fns::nccell_bg_default_p(&c1)];
}

#[test]
#[serial]
fn palette() {
    let mut c1 = NcCell::new();
    assert_eq![false, fns::nccell_fg_palindex_p(&c1)];
    assert_eq![false, fns::nccell_bg_palindex_p(&c1)];
    assert_eq![0, fns::nccell_fg_palindex(&c1)];
    assert_eq![0, fns::nccell_bg_palindex(&c1)];

    fns::nccell_set_fg_palindex(&mut c1, 5);
    fns::nccell_set_bg_palindex(&mut c1, 6);
    assert_eq![true, fns::nccell_fg_palindex_p(&c1)];
    assert_eq![true, fns::nccell_bg_palindex_p(&c1)];

    assert_eq![5, fns::nccell_fg_palindex(&c1)];
    assert_eq![6, fns::nccell_bg_palindex(&c1)];
}
