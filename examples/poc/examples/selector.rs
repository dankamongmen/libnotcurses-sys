//! Selector widget example
//! Copied from: https://github.com/dankamongmen/notcurses/blob/master/src/poc/selector.c
//!
//! All types must be declared explicitely
//! At all times, exactly one item is selected.
//!
//!
//!01:                                                      selector widget demo
//!02: ╭───────────────────────────────────────────────────────────────────────╮
//!03: │ this is truly, absolutely an awfully long example of a selector title │
//!04: ╰─────┬──────────────────────────────pick one (you will die regardless)─┤
//!05:       │             ↑                                                   │
//!06:       │    Afrikaans Ek kan glas eet, dit maak my nie seer nie.         │
//!07:       │     AngloSax ᛁᚳ᛫ᛗᚨᚷ᛫ᚷᛚᚨᛋ᛫ᛖᚩᛏᚪᚾ᛫ᚩᚾᛞ᛫ᚻᛁᛏ᛫ᚾᛖ᛫ᚻᛖᚪᚱᛗᛁᚪᚧ᛫ᛗᛖ᛬          │
//!08:       │     Japanese 私はガラスを食べられます。それは私を傷つけません。 │
//!09:       │ Kabuverdianu M’tá podê kumê vidru, ká stá máguame.              │
//!10:       │        Khmer ខ្ញុំអាចញុំកញ្ចក់បាន ដោយគ្មានបញ្ហារ                         │
//!11:       │          Lao ຂອ້ຍກິນແກ້ວໄດ້ໂດຍທີ່ມັນບໍ່ໄດ້ເຮັດໃຫ້ຂອ້ຍເຈັບ.                    │
//!12:       │             ↓                                                   │
//!13:       ╰──────────────────────────────press q to exit (there is no exit)─╯
//!14:

use libnotcurses_sys::{widgets::*, *};

fn main() -> NcResult<()> {
    // Init context
    let nc: &mut Nc = unsafe { Nc::new()? };

    // Enable mouse
    nc.mice_enable(NcMiceEvents::All)?;

    // Get a reference to the standard plane (full screen)
    let stdplane: &mut NcPlane = unsafe { nc.stdplane() };

    // Set font color (green)
    stdplane.set_fg_rgb(0x40f040);

    // Set title
    stdplane.set_scrolling(true);
    stdplane.putstr_aligned(Some(0), NcAlign::Right, "selector widget demo")?;

    // Create selection plane
    // y: i32, x: i32, rows: u32, cols: u32
    let planeopts: NcPlaneOptions = NcPlaneOptions::new_aligned(1, NcAlign::Left, 15, 80);
    let selplane: &mut NcPlane = NcPlane::new_child(stdplane, &planeopts)?;

    // Create selector
    let selector = NcSelector::builder()
        .item("Afrikaans", "Ek kan glas eet, dit maak my nie seer nie.")
        .item("AngloSax", "ᛁᚳ᛫ᛗᚨᚷ᛫ᚷᛚᚨᛋ᛫ᛖᚩᛏᚪᚾ᛫ᚩᚾᛞ᛫ᚻᛁᛏ᛫ᚾᛖ᛫ᚻᛖᚪᚱᛗᛁᚪᚧ᛫ᛗᛖ᛬")
        .item(
            "Japanese",
            "私はガラスを食べられます。それは私を傷つけません。",
        )
        .item("Kabuverdianu", "M’tá podê kumê vidru, ká stá máguame.")
        .item("Khmer", "ខ្ញុំអាចញុំកញ្ចក់បាន ដោយគ្មានបញ្ហារ")
        .item("Lao", "ຂອ້ຍກິນແກ້ວໄດ້ໂດຍທີ່ມັນບໍ່ໄດ້ເຮັດໃຫ້ຂອ້ຍເຈັບ.")
        .item("Russian", "Я могу есть стекло, оно мне не вредит.")
        .item("Sanskrit", "kācaṃ śaknomyattum; nopahinasti mām.")
        .item("Braille", "⠊⠀⠉⠁⠝⠀⠑⠁⠞⠀⠛⠇⠁⠎⠎⠀⠁⠝⠙⠀⠊⠞⠀⠙⠕⠑⠎⠝⠞⠀⠓⠥⠗⠞⠀⠍⠑")
        .item("Tibetan", "ཤེལ་སྒོ་ཟ་ནས་ང་ན་གི་མ་རེད།")
        .title("this is truly, absolutely an awfully long example of a selector title")
        .secondary("pick one (you will die regardless)")
        .footer("press q to exit (there is no exit)")
        .max_display(4)
        .default_item(1)
        .box_channels(NcChannels::from_rgb(0x20e040, 0x202020))
        .item_channels(
            NcChannels::from_rgb(0xe08040, 0),
            NcChannels::from_rgb(0x80e040, 0),
        )
        .secondary_channels(NcChannels::from_rgb(0xe00040, 0x200000))
        .title_channels(NcChannels::from_rgb(0xffff80, 0x000020))
        .finish(selplane)?;

    // Create description plane
    let planeopts2: NcPlaneOptions = NcPlaneOptions::new_aligned(15, NcAlign::Left, 30, 80);
    let descplane: &mut NcPlane = NcPlane::new_child(stdplane, &planeopts2)?;
    descplane.set_scrolling(true);
    descplane.puttext(
        0,
        NcAlign::Left,
        "Example of a selector widget:\n\
        -- Use the default mouse or arrow key to change selected line.\n\
        -- Or the customized J, K, TAB, SHIFT-TAB.\n\
        -- Press ENTER (or Q) when satisfied with selection (or bored).",
    )?;

    // Render loop
    let selected: String = run_selector(nc, selector)?;

    // Destroy ressources
    selector.destroy()?;

    // Restore context, TERM status like cursor
    unsafe { nc.stop()? };

    // Print solution, now the TERM is normal stdio
    println!("You chose language: {}", &selected);

    Ok(())
}

/// Helper to avoid having a render loop in the main function
fn run_selector(nc: &mut Nc, selector: &mut NcSelector) -> NcResult<String> {
    // Allocate input placeholder
    let mut ni: NcInput = NcInput::new_empty();

    // Pre render <= do not wait input for first rendering
    nc.render()?;

    loop {
        // Wait until user acts
        let keypress: NcReceived = nc.get_blocking(Some(&mut ni))?;

        if !selector.offer_input(ni) {
            // Do not consider release key: only press
            if ni.evtype == NcInputType::Release as u32 {
                continue;
            }

            // Act in function of key pressed
            match keypress {
                NcReceived::Char(ch) => {
                    match ch {
                        // Q => quit
                        'q' | 'Q' => {
                            return selector.selected().ok_or_else(|| NcError::new());
                        }
                        // J => down
                        'j' | 'J' => {
                            selector.nextitem()?;
                        }
                        // K => up
                        'k' | 'K' => {
                            selector.previtem()?;
                        }
                        // Tab => up or down depending if shift is pressed
                        '\u{0009}' => match ni.shift {
                            true => {
                                selector.previtem()?;
                            }
                            false => {
                                selector.nextitem()?;
                            }
                        },
                        _ => (),
                    }
                }
                NcReceived::Event(ev) => match ev {
                    NcKey::ENTER => {
                        return selector.selected().ok_or_else(|| NcError::new());
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // Render updated selector
        nc.render()?;
    }
}
