//! `erase_region` example.
//!
//! Shows the result of calling `NcPlane.erase_region` with different parameters.
//!
//! press 'q' to quit at any moment.

use libnotcurses_sys::*;

/// contains the state of the program
struct State<'nc, 'p> {
    nc: &'nc mut Nc,
    showcase: &'p mut NcPlane,
    info: &'p mut NcPlane,
    cursor: &'p mut NcPlane,
}
impl<'nc, 'p> State<'nc, 'p> {
    /// exits the program cleanly
    fn exit(&mut self, exit_code: i32) -> NcResult<()> {
        self.showcase.destroy()?;
        self.info.destroy()?;
        self.cursor.destroy()?;
        unsafe { self.nc.stop()? };
        std::process::exit(exit_code);
    }
}
impl<'nc, 'p> Drop for State<'nc, 'p> {
    fn drop(&mut self) {
        let _ = self.exit(0);
    }
}
static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat";

#[rustfmt::skip]
fn main() -> NcResult<()> {
    let mut _nc = unsafe { Nc::new()? };
    let stp = unsafe {_nc.stdplane() };
    let mut _showcase = NcPlane::new_child_sized(stp, 1, 10, 10, 20)?;
    let mut _info = NcPlane::new_child_sized(stp, 12, 1, 3, 54)?;
    let mut _cursor = NcPlane::new_child_sized(stp, 12, 1, 1, 1)?;
    let mut state = State {
        nc: _nc,
        showcase: _showcase,
        info: _info,
        cursor: _cursor,
    };
    state.showcase.set_base("▒", 0, NcChannels::from_rgb(0x662222, 0x557755))?;
    state.showcase.set_channels(NcChannels::from_rgb(0x222222, 0x449944));
    state.info.set_base(" ", 0, NcChannels::from_rgb(0x222222, 0xAAAAAA))?;
    state.cursor.set_base("█", 0, NcChannels::from_rgb(0x662222, 0x557755))?;
    state.showcase.set_scrolling(true);
    state.info.set_scrolling(true);

    // the first slide
    state.showcase.putstr(&LOREM_IPSUM.repeat(2))?;
    state.info.putstrln(" Showcase on a 10x20 plane the results of calling")?;
    state.info.putstrln("`NcPlane.erase_region()` with different arguments.")?;
    print_info_row(
        &mut state,
        2,
        Some(NcChannels::from_rgb(0x113355, 0x8899CC)), // blue
        NcAlign::Center,
        "(press 'q' to quit, or any other key to continue)"
    )?;
    render_and_wait_input(&mut state)?;

    // THE ACTUAL SLIDESHOW
    erase_region_slideshow(&mut state)?;

    // the last slide
    state.showcase.erase();
    state.info.erase();
    print_info_row(&mut state, 1, None, NcAlign::Center, "that's all, folks! ")?;
    state
        .showcase
        .set_base(" ", 0, NcChannels::from_rgb(0x224411, 0x992222))?;
    render_and_wait_input(&mut state)?;

    // bye
    state.exit(0)?;
    Ok(())
}

/// A *slideshow* of erase_region() examples.
#[rustfmt::skip]
fn erase_region_slideshow(state: &mut State) -> NcResult<()> {
    let cursor = (5, 10);

    erase_region(state, cursor, None, None, 0, 0, "everything (if cursor is in a legal position)")?;
    erase_region(state, cursor, Some(0), Some(0), 0, 0, "everything in all cases")?;
    erase_region(state, cursor, None, None, 1, 1, "current cursor position")?;
    erase_region(state, cursor, None, None, -1, -1, "current cursor position")?;

    erase_region(state, cursor, None, None, 1, 0, "the current row")?;
    erase_region(state, cursor, None, None, 2, 0, "the current row and the one after")?;
    erase_region(state, cursor, None, None, i32::MAX, 0, "all rows with or below the cursor")?;
    erase_region(state, cursor, None, None, -1, 0, "the current row")?;
    erase_region(state, cursor, None, None, -2, 0, "the current row and the one before")?;
    erase_region(state, cursor, None, None, i32::MIN, 0, "all rows with or above the cursor")?;

    erase_region(state, cursor, Some(0), None, 3, 0, "rows 1, 2 and 3")?;
    erase_region(state, cursor, Some(4), None, 3, 0, "rows 4, 5 and 6")?;
    erase_region(state, cursor, Some(7), None, 3, 0, "rows 8, 9 and 10")?;

    erase_region(state, cursor, None, None, 0, 1, "the current column")?;
    erase_region(state, cursor, None, None, 0, 3, "the current column and the two after")?;
    erase_region(state, cursor, None, None, 0, i32::MAX, "all columns with or after the cursor")?;
    erase_region(state, cursor, None, None, 0, -1, "the current column")?;
    erase_region(state, cursor, None, None, 0, -3, "the current column and the two before")?;
    erase_region(state, cursor, None, None, 0, i32::MIN, "all columns with or before the cursor")?;

    erase_region(state, cursor, None, Some(0), 0, 3, "columns 1, 2 and 3")?;
    erase_region(state, cursor, None, Some(4), 0, 3, "columns 5, 6 and 7")?;
    erase_region(state, cursor, None, Some(15), 0, 3, "columns 16, 17 and 18")?;

    erase_region(state, cursor, None, None, 5, 5, "a 5×5 area down-right from current position")?;
    erase_region(state, cursor, None, None, -5, -5, "a 5×5 area up-left from current position")?;
    erase_region(state, cursor, None, None, -5, 5, "a 5×5 area up-right from current position")?;
    erase_region(state, cursor, None, None, 5, -5, "a 5×5 area down-left from current position")?;

    erase_region(state, cursor, None, Some(4), 3, 4, "a 3×4 area down-right from (5, 4) to (7, 7)")?;
    erase_region(state, cursor, None, Some(4), -3, -4, "a 3×4 area up-left from (5, 4) to (3, 1)")?;
    erase_region(state, cursor, None, Some(4), -3, 4, "a 3×4 area up-right from (5, 4) to (7, 7)")?;
    erase_region(state, cursor, None, Some(4), 3, -4, "a 3×4 area down-left from (5, 4) to (7, 1)")?;

    Ok(())
}

/// shows a *slide* with the result of calling `erase_region`.
fn erase_region(
    state: &mut State,
    cursor: (u32, u32),
    ystart: Option<u32>,
    xstart: Option<u32>,
    ylen: i32,
    xlen: i32,
    what_is_erased: &str,
) -> NcResult<()> {
    // resets the showcase plane
    state.showcase.erase();
    state.showcase.putstr(&LOREM_IPSUM.repeat(2))?;
    state.showcase.cursor_move_yx(cursor.0, cursor.1)?;
    show_cursor(state)?;

    // prints the information
    state.info.erase();
    print_info_row(
        state,
        0,
        Some(NcChannels::from_rgb(0x555555, 0x999999)),
        NcAlign::Right,
        &format!["cursor_yx({}, {})", cursor.0, cursor.1],
    )?;

    let ylen_str = match ylen {
        i32::MAX => "i32::MAX".into(),
        i32::MIN => "i32::MIN".into(),
        _ => ylen.to_string(),
    };
    let xlen_str = match xlen {
        i32::MAX => "i32::MAX".into(),
        i32::MIN => "i32::MIN".into(),
        _ => xlen.to_string(),
    };
    print_info_row(
        state,
        1,
        None,
        NcAlign::Center,
        &format![
            "erase_region({:?}, {:?}, {}, {})",
            ystart, xstart, ylen_str, xlen_str
        ],
    )?;
    print_info_row(
        state,
        2,
        Some(NcChannels::from_rgb(0x555577, 0xAAAABB)),
        NcAlign::Center,
        &format!["erases {}", what_is_erased],
    )?;

    // does the deed
    state.showcase.erase_region(ystart, xstart, ylen, xlen)?;
    render_and_wait_input(state)?;
    Ok(())
}

/// renders the planes, and waits for input. 'q' quits.
fn render_and_wait_input(state: &mut State) -> NcResult<()> {
    state.nc.render()?;
    let res = state.nc.get_blocking(None)?;

    if res == NcReceived::Char('q') {
        state.exit(0)?;
    }
    Ok(())
}

/// positions the cursor plane over the showcase plane
fn show_cursor(state: &mut State) -> NcResult<()> {
    let (sy, sx) = state.showcase.yx();
    let (cy, cx) = state.showcase.cursor_yx();
    state.cursor.move_yx(sy + cy as i32, sx + cx as i32)?;
    Ok(())
}

/// prints a row in the info panel, with optional stain
fn print_info_row(
    state: &mut State,
    row: u32,
    stain: Option<NcChannels>,
    align: NcAlign,
    string: &str,
) -> NcResult<()> {
    let info_len = state.info.dim_x();

    // stain
    if let Some(channels) = stain {
        state.info.cursor_move_yx(row, 0)?;
        state.info.putstr(&" ".repeat(info_len as usize))?;
        state.info.cursor_move_yx(row, 0)?;
        state.info.stain(
            Some(row),
            Some(0),
            Some(1),
            None,
            channels,
            channels,
            channels,
            channels,
        )?;
    }

    state.info.putstr_aligned_stained(Some(row), align, string)?;
    Ok(())
}
