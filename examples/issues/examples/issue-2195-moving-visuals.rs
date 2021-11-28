// moving visuals outside of window boundaries doesn't work well
//
// https://github.com/dankamongmen/notcurses/issues/2195

use libnotcurses_sys::*;

// height and width of the visual, in cells
const H_CELLS: u32 = 10;
const W_CELLS: u32 = 20;
const CELLS_MOVEMENT: u32 = 40;

fn main() -> NcResult<()> {
    let mut nc = unsafe { Nc::new()? };
    let stp = unsafe { nc.stdplane() };
    // stp.set_scrolling(true); // doesn't seem to affect

    // firstly we move a text plane
    let plane1 = NcPlane::new_bound(stp, 0, 0, H_CELLS, W_CELLS)?;
    plane1.set_base("p", 0, NcChannels::from_rgb(0x778899, 0xBBBBBB))?;

    for _ in 0..CELLS_MOVEMENT {
        plane1.move_rel(1, 1)?;
        nc_render_sleep![&mut nc, 0, 50];
    }
    sleep![1];

    // secondly we move a visual plane and compare
    let geo = stp.pixel_geom();
    let width = W_CELLS * geo.cell_x;
    let height = H_CELLS * geo.cell_y;
    let buffer: Vec<u8> = vec![0xBB; (height * width) as usize * 3];
    let visual1 = NcVisual::from_rgb_packed(buffer.as_slice(), height, width * 3, width, 255)?;
    let voptions1 = NcVisualOptions::builder()
        .yx(1, 2)
        .blitter(NcBlitter::PIXEL)
        .build();
    let visual1plane = unsafe { visual1.blit(&mut nc, Some(&voptions1))? };
    visual1plane.reparent(stp)?;

    for _ in 0..CELLS_MOVEMENT {
        visual1plane.move_rel(1, 1)?;
        nc_render_sleep![&mut nc, 0, 50];
    }
    sleep![1];

    visual1plane.destroy()?;
    visual1.destroy();
    unsafe { nc.stop()? };

    Ok(())
}
