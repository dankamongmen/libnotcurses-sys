//! Example 'direct-image'
//!
//! Explore image rendering in direct mode
//!
//! NOTE: This example uses the C style with functions.

use core::ptr::{null, null_mut};
use libnotcurses_sys::{*, fns::*};

mod shared;

fn main() {
    unsafe {
        let ncd = ncdirect_init(null(), null_mut(), 0);

        render_image(&mut *ncd, NCBLIT_1x1);
        render_image(&mut *ncd, NCBLIT_2x1);
        render_image(&mut *ncd, NCBLIT_BRAILLE);

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
            blit,
            NCSCALE_NONE,
        ) != 0 {
            panic!("ERROR: ncdirect_render_image().");
        }
    }
}
