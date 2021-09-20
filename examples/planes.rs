use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = Nc::new()?;

    // get the terminal size in character rows & columns
    let (t_rows, t_cols) = nc.term_dim_yx();
    // the standard plane should have the same size
    let mut stdplane = nc.stdplane();
    assert_eq![(t_rows, t_cols), stdplane.dim_yx()];

    // set the style of stdplane's base cell, make it blue
    stdplane.set_base("¬", 0, NcChannels::from_rgb(0x88aa00, 0x222288))?;
    nrs![&mut nc, 1];

    // add a green plane to the stdplane's pile, displaced right
    let plane_green = NcPlane::new_bound(&mut stdplane, 8, 0, 16, 30)?;
    plane_green.set_base("·", 0, NcChannels::from_rgb(0x224411, 0x229922))?;

    // and add a smaller red plane, displaced down
    let plane_red = NcPlane::new_bound(&mut stdplane, 0, 18, 12, 22)?;
    plane_red.set_base("~", 0, NcChannels::from_rgb(0xaadd2b, 0x882222))?;
    nrs![&mut nc, 0, 500];

    // write in the planes
    stdplane.putstr("000 STDPLANE 000")?;
    plane_green.putstr("111 PLANE 111")?;
    plane_red.putstr("222 PLANE 222")?;
    nrs![&mut nc, 0, 500];

    // TODO: write with styles

    // exit(1, &mut nc, vec![plane_green, plane_red])?;

    // move the green plane down-right
    for _ in 0..16 {
        plane_green.move_rel(1, 1)?;
        nrs![&mut nc, 0, 20];
    }
    // and up
    for _ in 0..16 {
        plane_green.move_rel(-1, -1)?;
        nrs![&mut nc, 0, 20];
    }

    // move the red plane up-left
    for _ in 0..16 {
        plane_red.move_rel(-1, -1)?;
        nrs![&mut nc, 0, 20];
    }
    // and left
    for _ in 0..16 {
        plane_red.move_rel(1, 1)?;
        nrs![&mut nc, 0, 20];
    }
    sleep![1];

    // make the planes scrollable and put a long text
    let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat";
    plane_green.set_scrolling(true);
    plane_green.putstr(lorem_ipsum)?;
    plane_red.set_scrolling(true);
    plane_red.putstr(lorem_ipsum)?;
    nrs![&mut nc, 0, 500];

    // TODO: text with style

    // reorder planes in the z-buffer
    plane_green.move_above(plane_red)?;
    nrs![&mut nc, 0, 500];
    plane_green.move_below(plane_red)?;
    nrs![&mut nc, 0, 500];
    plane_green.move_bottom(); // below every plane
    nrs![&mut nc, 0, 500];
    plane_green.move_top(); // above every plane
    nrs![&mut nc, 0, 500];

    // resize the planes, text gets cut
    plane_green.resize_simple(6, 12)?;
    plane_red.resize_simple(4, 8)?;
    nrs![&mut nc, 0, 300];
    plane_green.resize_simple(16, 30)?;
    plane_red.resize_simple(12, 22)?;
    nrs![&mut nc, 0, 300];

    nrs![&mut nc, 3];
    exit(0, &mut nc, vec![plane_green, plane_red])?;
    Ok(())
}

/// quit the example, cleanly & safely
fn exit(ecode: i32, nc: &mut Nc, planes: Vec<&mut NcPlane>) -> NcResult<()>{
    for p in planes {
        p.destroy()?;
    }
    nc.stop()?;
    std::process::exit(ecode);
}
