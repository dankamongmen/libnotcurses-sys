use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = Nc::new()?;

    let width = 32;
    let height = 32;
    let buffer: Vec<u8> = vec![0xBB; (height * width) as usize * 3];

    let visual = NcVisual::from_rgb_packed(buffer.as_slice(), height, width * 3, width, 255)?;
    let vopt = NcVisualOptions::without_plane(1, 2, 0, 0, height, width, NcBlitter::PIXEL, 0, 0);

    // THIS WORKS (USING ncvisual_render):
    // let _plane = visual.render(&mut nc, &vopt)?;
    // let _plane = unsafe { c_api::ncvisual_render(nc, visual, &vopt) };

    // FIXME: THIS DOESN'T WORK (USING ncvisual_blit):
    let _plane = visual.blit(&mut nc, Some(&vopt))?;
    // let plane = visual.blit(&mut nc, None)?;
    // let _plane = unsafe { c_api::ncvisual_blit(nc, visual, &vopt) };
    // let _plane = unsafe { c_api::ncvisual_blit(nc, visual, std::ptr::null()) };

    nc_render_sleep![&mut nc, 2];

    // plane.destroy()?;
    visual.destroy();
    nc.stop()?;

    Ok(())
}
