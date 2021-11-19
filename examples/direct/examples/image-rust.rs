//! Example 'direct-image'
//!
//! Explore image rendering in direct mode
//!
//! NOTE: This example uses the Rust style with methods.

use libnotcurses_sys::*;

mod shared;

fn main() -> NcResult<()> {
    let mut ncd = unsafe { NcDirect::new()? };

    render_image(&mut ncd, NcBlitter::_1x1)?;
    render_image(&mut ncd, NcBlitter::_2x1)?;
    render_image(&mut ncd, NcBlitter::BRAILLE)?;

    unsafe { ncd.stop()? };
    Ok(())
}

fn render_image(ncd: &mut NcDirect, blit: NcBlitter) -> NcResult<()> {
    let image_path = shared::project_root_path_string("examples/res/image-16x16.png");

    if let Err(nc_error) = ncd.render_image(&image_path, NcAlign::CENTER, blit, NcScale::NOSCALE) {
        return Err(NcError::with_msg(
            nc_error.int,
            "ERROR: ncdirect_render_image(). Make sure you \
            are running this example from the examples folder",
        ));
    }
    Ok(())
}
