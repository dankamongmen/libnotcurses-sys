//! based on the proof of concept at ../../src/poc/menu.c

// FIXME: has bugs, doesn't work well
// probably related to the arrays or the strings...

use libnotcurses_sys::{widgets::*, *};

fn main() -> NcResult<()> {
    let mut nc = unsafe { Nc::new()? };
    nc.mice_enable(NcMiceEvents::All)?;

    let mut demo_items = [
        NcMenuItem::new("Restart", NcInput::with_ctrl('r')),
        NcMenuItem::new("Disabled", NcInput::with_ctrl('🙂')),
    ];
    let mut file_items = [
        NcMenuItem::new("New", NcInput::with_ctrl('n')),
        NcMenuItem::new("Open", NcInput::with_ctrl('o')),
        NcMenuItem::new("Close", NcInput::with_ctrl('c')),
        NcMenuItem::new_empty(),
        NcMenuItem::new("Quit", NcInput::with_ctrl('q')),
    ];

    let mut help_items = [NcMenuItem::new("About", NcInput::with_ctrl('a'))];

    let mut sections = [
        NcMenuSection::new("Demo", &mut demo_items, NcInput::with_alt('a')),
        NcMenuSection::new("File", &mut file_items, NcInput::with_alt('f')),
        NcMenuSection::new_separator(),
        // DEBUG: remove alt modifier for now.
        NcMenuSection::new("Help", &mut help_items, NcInput::new('h')),
    ];

    let mut mopts = NcMenuOptions::new(&mut sections);
    mopts.header_channels_mut().set_fg_rgb(0x00ff00);
    mopts.header_channels_mut().set_bg_rgb(0x440000);
    mopts.section_channels_mut().set_fg_rgb(0xb0d700);
    mopts.section_channels_mut().set_bg_rgb(0x002000);

    let stdplane = unsafe { nc.stdplane() };
    let (dim_y, _dim_x) = stdplane.dim_yx();

    let menu_top = NcMenu::new(stdplane, &mopts)?;
    //menu_top.item_set_status("Schwarzgerät", "Disabled", false)?;
    //menu_top.item_set_status("Schwarzgerät", "Restart", false)?;

    stdplane.set_base("x", 0, NcChannels::from_rgb(0x88aa00, 0x000088))?;

    nc.render()?;

    stdplane.set_fg_rgb(0x00dddd);
    stdplane.putstr_aligned(
        Some(dim_y - 1),
        NcAlign::Right,
        " -=+ menu poc. press q to exit +=-",
    )?;

    run_menu(&mut nc, menu_top)?;

    stdplane.erase(); // is this needed?

    // BUG FIXME: this doesn't show over the menu (at row 0)
    stdplane.putstr_aligned(
        Some(0),
        NcAlign::Right,
        " -=+ menu poc. press q to exit +=-",
    )?;
    stdplane.putstr_aligned(
        Some(1),
        NcAlign::Center,
        " -=+ menu poc. press q to exit +=-",
    )?;
    stdplane.putstr_aligned(Some(2), NcAlign::Left, " -=+ menu poc. press q to exit +=-")?;

    mopts.flags |= NcMenuOptions::BOTTOM;
    let menu_bottom = NcMenu::new(stdplane, &mopts)?;

    run_menu(&mut nc, menu_bottom)?;

    Ok(())
}

fn run_menu(nc: &mut Nc, menu: &mut NcMenu) -> NcResult<()> {
    // yellow rectangle
    let planeopts = NcPlaneOptions::new_aligned(10, NcAlign::Center, 10, 40);
    let stdplane = unsafe { nc.stdplane() };
    let selplane = NcPlane::new_child(stdplane, &planeopts)?;
    selplane.set_fg_rgb(0);
    selplane.set_bg_rgb(0xdddddd);
    let mut channels = 0;
    channels.set_fg_rgb(0x000088);
    channels.set_bg_rgb(0x88aa00);
    selplane.set_base(" ", 0, channels)?;
    // Otherwise get crash with puttext
    selplane.set_scrolling(true);

    let mut ni = NcInput::new_empty();
    let mut rec: NcReceived;
    nc.render()?;

    loop {
        stdplane.erase();
        selplane.erase();

        rec = nc.get_blocking(Some(&mut ni))?;

        // DEBUG: FIXME
        stdplane.putstr_yx(Some(2), Some(0), &format!["{:?}", ni])?;
        nc.render()?;

        // BUG FIXME: always returns false:
        if !menu.offer_input(ni) {
            match rec {
                NcReceived::Char(ch) => match ch {
                    'q' => {
                        menu.destroy();
                        selplane.destroy()?;
                        unsafe { nc.stop()? };
                        return Ok(());
                    }
                    _ => (),
                },
                NcReceived::Event(ev) => match ev {
                    NcKey::ENTER => {
                        if let Some(selection) = menu.selected(Some(&mut ni)) {
                            match selection.as_ref() {
                                "Quit" => {
                                    menu.destroy();
                                    selplane.destroy()?;
                                    unsafe { nc.stop()? };
                                    return Ok(());
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        let mut selni = NcInput::new_empty();
        if let Some(selitem) = menu.selected(Some(&mut selni)) {
            let mut msg = String::new();
            msg.push_str("Item selected:\n\nDescription: ");
            msg.push_str(&selitem);
            msg.push_str("\nInput Id: ");
            msg.push_str(&selni.id.to_string());
            selplane.puttext(1, NcAlign::Left, &msg)?;
        } else {
            selplane.puttext(1, NcAlign::Center, "No menu item currently selected")?;
        }
        nc.render()?;
    }
}
