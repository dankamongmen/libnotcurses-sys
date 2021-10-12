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

use libnotcurses_sys::{
    //// Selector:
    widgets::{NcSelector, NcSelectorItem, NcSelectorOptions},
    // Core
    Nc,
    NcAlign,
    NcAlignApi,
    NcEvType,
    NcEvTypeApi,
    // Input
    NcInput,
    NcKey,
    // Plane
    NcPlane,
    NcPlaneOptions,
    NcResult,
};

fn main() -> NcResult<()> {
    // Init context
    let nc: &mut Nc = Nc::new()?;

    // Enable mouse
    nc.mouse_enable()?;

    // Declare items of selector
    let mut selector_items: [NcSelectorItem; 11] = [
        NcSelectorItem::new("Afrikaans", "Ek kan glas eet, dit maak my nie seer nie."),
        NcSelectorItem::new("AngloSax", "ᛁᚳ᛫ᛗᚨᚷ᛫ᚷᛚᚨᛋ᛫ᛖᚩᛏᚪᚾ᛫ᚩᚾᛞ᛫ᚻᛁᛏ᛫ᚾᛖ᛫ᚻᛖᚪᚱᛗᛁᚪᚧ᛫ᛗᛖ᛬"),
        NcSelectorItem::new(
            "Japanese",
            "私はガラスを食べられます。それは私を傷つけません。",
        ),
        NcSelectorItem::new("Kabuverdianu", "M’tá podê kumê vidru, ká stá máguame."),
        NcSelectorItem::new("Khmer", "ខ្ញុំអាចញុំកញ្ចក់បាន ដោយគ្មានបញ្ហារ"),
        NcSelectorItem::new("Lao", "ຂອ້ຍກິນແກ້ວໄດ້ໂດຍທີ່ມັນບໍ່ໄດ້ເຮັດໃຫ້ຂອ້ຍເຈັບ."),
        NcSelectorItem::new("Russian", "Я могу есть стекло, оно мне не вредит."),
        NcSelectorItem::new("Sanskrit", "kācaṃ śaknomyattum; nopahinasti mām."),
        NcSelectorItem::new("Braille", "⠊⠀⠉⠁⠝⠀⠑⠁⠞⠀⠛⠇⠁⠎⠎⠀⠁⠝⠙⠀⠊⠞⠀⠙⠕⠑⠎⠝⠞⠀⠓⠥⠗⠞⠀⠍⠑"),
        NcSelectorItem::new("Tibetan", "ཤེལ་སྒོ་ཟ་ནས་ང་ན་གི་མ་རེད།"),
        NcSelectorItem::new_empty(),
    ];

    // Pack those items in the selector custom option struct
    // -- with title, description and footer
    let opts: NcSelectorOptions = NcSelectorOptions::new(
        "this is truly, absolutely an awfully long example of a selector title",
        "pick one (you will die regardless)",
        "press q to exit (there is no exit)",
        &mut selector_items,
    );

    // Create first plane (full screen)
    let stdplane: &mut NcPlane = nc.stdplane();

    // Set font color (green)
    stdplane.set_fg_rgb(0x40f040);

    // Set title
    stdplane.set_scrolling(true);
    stdplane.putstr_aligned(0, NcAlign::RIGHT, "selector widget demo")?;

    // Create selection plane
    // y: NcOffset, x: NcOffset, rows: NcDim, cols: NcDim
    let planeopts: NcPlaneOptions = NcPlaneOptions::new_aligned(1, NcAlign::LEFT, 15, 80);
    let selplane: &mut NcPlane = NcPlane::with_options_bound(stdplane, planeopts)?;

    // Create selector
    let selector: &mut NcSelector = NcSelector::new(selplane, opts)?;

    // Create description plane
    let planeopts2: NcPlaneOptions = NcPlaneOptions::new_aligned(15, NcAlign::LEFT, 30, 80);
    let descplane: &mut NcPlane = NcPlane::with_options_bound(stdplane, planeopts2)?;
    descplane.set_scrolling(true);
    descplane.puttext(
        0,
        NcAlign::LEFT,
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
    nc.stop()?;

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
        let keypress: char = nc.getc_blocking(Some(&mut ni))?;

        if !selector.offer_input(ni) {
            // Do not consider release key: only press
            if ni.evtype == NcEvType::RELEASE {
                continue;
            }

            // Act in function of key pressed
            match keypress {
                // Q => quit
                'q' | 'Q' | NcKey::ENTER => {
                    return Ok(selector.selected()?);
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
                // Default: ignore
                _ => (),
            }
        }

        // Render updated selector
        nc.render()?;
    }
}
