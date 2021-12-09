//! Example 'direct-image'
//!
//! Explore image rendering in direct mode
//!
//! NOTE: This example uses the C style with functions.

use core::ptr::{null, null_mut};
use libnotcurses_sys::{c_api::*, *};

mod shared;

fn main() {
    unsafe {
        let ncd = ncdirect_init(null(), null_mut(), 0);

        render_image(&mut *ncd, NcBlitter::Ascii);
        render_image(&mut *ncd, NcBlitter::Half);
        render_image(&mut *ncd, NcBlitter::Braille);

        ncdirect_stop(ncd);
    }
}

fn render_image(ncd: &mut NcDirect, blit: NcBlitter) {
    let image_path = shared::project_root_path_string("examples/res/image-16x16.png");

    unsafe {
        if ncdirect_render_image(
            ncd,
            cstring![image_path],
            NCALIGN_CENTER,
            blit.into(),
            NCSCALE_NONE,
        ) != 0
        {
            panic!("ERROR: ncdirect_render_image().");
        }
    }
}
