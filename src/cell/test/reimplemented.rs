//! Test `cell*_*` reimplemented functions

use serial_test::serial;

use crate::{c_api, NcAlpha, NcCell};

#[test]
#[serial]
fn rgb() {
    // rgb

    let mut c1 = NcCell::new();
    assert_eq![0, c_api::nccell_fg_rgb(&c1)];
    assert_eq![0, c_api::nccell_bg_rgb(&c1)];

    c_api::nccell_set_fg_rgb(&mut c1, 0x99112233_u32);
    assert_eq![0x112233, c_api::nccell_fg_rgb(&c1)];
    c_api::nccell_set_bg_rgb(&mut c1, 0x99445566_u32);
    assert_eq![0x445566, c_api::nccell_bg_rgb(&c1)];

    // rgb8

    let mut c2 = NcCell::new();
    let (mut r, mut g, mut b) = (0, 0, 0);

    c_api::nccell_set_fg_rgb8(&mut c2, 0x11, 0x22, 0x33);
    let fchannel = c_api::nccell_fg_rgb8(&c2, &mut r, &mut g, &mut b);
    assert_eq!((0x11, 0x22, 0x33), (r, g, b));
    assert_eq![0x112233, fchannel & !c_api::NC_BGDEFAULT_MASK];

    c_api::nccell_set_bg_rgb8(&mut c2, 0x44, 0x55, 0x66);
    let bchannel = c_api::nccell_bg_rgb8(&c2, &mut r, &mut g, &mut b);
    assert_eq!((0x44, 0x55, 0x66), (r, g, b));
    assert_eq![0x445566, bchannel & !c_api::NC_BGDEFAULT_MASK];
}

#[test]
#[serial]
fn alpha() {
    let mut c1 = NcCell::new();
    assert_eq![c_api::NCALPHA_OPAQUE, c_api::nccell_fg_alpha(&c1)];
    assert_eq![c_api::NCALPHA_OPAQUE, c_api::nccell_bg_alpha(&c1)];

    c_api::nccell_set_fg_alpha(&mut c1, NcAlpha::Transparent);
    assert_eq![c_api::NCALPHA_TRANSPARENT, c_api::nccell_fg_alpha(&c1)];

    c_api::nccell_set_bg_alpha(&mut c1, crate::NcAlpha::Blend);
    assert_eq![c_api::NCALPHA_BLEND, c_api::nccell_bg_alpha(&c1)];
}

#[test]
#[serial]
fn default() {
    let mut c1 = NcCell::new();
    assert_eq![true, c_api::nccell_fg_default_p(&c1)];
    assert_eq![true, c_api::nccell_bg_default_p(&c1)];

    // rgb
    c_api::nccell_set_fg_rgb(&mut c1, 0x112233_u32);
    c_api::nccell_set_bg_rgb(&mut c1, 0x445566_u32);
    assert_eq![false, c_api::nccell_fg_default_p(&c1)];
    assert_eq![false, c_api::nccell_bg_default_p(&c1)];

    // reset
    c_api::nccell_set_fg_default(&mut c1);
    c_api::nccell_set_bg_default(&mut c1);
    assert_eq![true, c_api::nccell_fg_default_p(&c1)];
    assert_eq![true, c_api::nccell_bg_default_p(&c1)];

    // rgb8
    c_api::nccell_set_fg_rgb8(&mut c1, 0x11, 0x22, 0x33);
    c_api::nccell_set_bg_rgb8(&mut c1, 0x44, 0x55, 0x66);
    assert_eq![false, c_api::nccell_fg_default_p(&c1)];
    assert_eq![false, c_api::nccell_bg_default_p(&c1)];

    // reset
    c_api::nccell_set_fg_default(&mut c1);
    c_api::nccell_set_bg_default(&mut c1);

    // palette
    c_api::nccell_set_fg_palindex(&mut c1, 5);
    c_api::nccell_set_bg_palindex(&mut c1, 6);
    assert_eq![false, c_api::nccell_fg_default_p(&c1)];
    assert_eq![false, c_api::nccell_bg_default_p(&c1)];
}

#[test]
#[serial]
fn palette() {
    let mut c1 = NcCell::new();
    assert_eq![false, c_api::nccell_fg_palindex_p(&c1)];
    assert_eq![false, c_api::nccell_bg_palindex_p(&c1)];
    assert_eq![0, c_api::nccell_fg_palindex(&c1)];
    assert_eq![0, c_api::nccell_bg_palindex(&c1)];

    c_api::nccell_set_fg_palindex(&mut c1, 5);
    c_api::nccell_set_bg_palindex(&mut c1, 6);
    assert_eq![true, c_api::nccell_fg_palindex_p(&c1)];
    assert_eq![true, c_api::nccell_bg_palindex_p(&c1)];

    assert_eq![5, c_api::nccell_fg_palindex(&c1)];
    assert_eq![6, c_api::nccell_bg_palindex(&c1)];
}
