//! can't move the cursor to where it is already (end-of-plane)

use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Nc::new_cli()?;

    nc.refresh()?;

    println!("Creating a plane of 10×20…");

    let plane = NcPlane::new_bound(nc.stdplane(), 10, 0, 10, 20)?;
    plane.set_base("·", 0, NcChannels::from_rgb(0x224411, 0x229922))?;
    plane.set_scrolling(true);

    println!["cursor begins at: {:?}", plane.cursor_yx()];

    plane.putstr(&"<txt>".repeat(80))?;

    let (cursor_y, cursor_x) = plane.cursor_yx();
    println!["cursor ends at: ({}, {})", cursor_y, cursor_x];

    println!("trying to move to the same position. . .");

    if let Err(_) = plane.cursor_move_yx(cursor_y, cursor_x) {
        println!("ERROR!!");
    }

    nc.render()?;
    plane.destroy()?;
    nc.stop()?;
    Ok(())
}
