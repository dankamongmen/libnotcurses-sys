//! Pixel POC
//! Inspired by: https://github.com/dankamongmen/notcurses/blob/master/src/poc/pixel.c
//!
//! This displays an image scaled to the terminal size
//! Press 'q' or 'ctrl-c' to quit
//!
//! All types must be declared explicitely

use libnotcurses_sys::{
    // Core
    Nc,
    NcBlitter,
    NcBlitterApi,
    NcError,
    // Input
    NcInput,
    NcKey,
    // Plane
    NcPlane,
    NcPlaneOptions,
    NcReceived,
    NcResult,
    NcScale,
    NcScaleApi,
    // Visual
    NcVisual,
    NcVisualOptions,
};

fn main() -> NcResult<()> {
    // Parse argument in
    let filepath: String = std::env::args()
        .nth(1)
        .expect("Error: Must pass image file path as first argument. Usage: pixel image.png");

    // Init notcurses context
    let nc: &mut Nc = Nc::new()?;

    // Clause: Pixel must be supported
    if 0 == nc.check_pixel_support() {
        nc.stop()?;
        return Err(NcError::with_msg(
            1,
            "Error: This program requires pixel graphics support",
        ));
    }

    // Obtain reference to visual (something like an image)
    let visual: &mut NcVisual = NcVisual::from_file(&filepath)?;

    // Create full screen plane
    // -- Notcurses report an error if image is draw on the stdplane
    let stdplane: &mut NcPlane = nc.stdplane();
    let (row, col) = stdplane.dim_yx();
    let planeopts: NcPlaneOptions = NcPlaneOptions::new(0, 0, row, col);
    let pixelplane: &mut NcPlane = NcPlane::with_options_bound(stdplane, &planeopts)?;

    // Craft some visual options (here full screen)
    let opts: NcVisualOptions = NcVisualOptions::new(
        Some(pixelplane),
        NcScale::SCALE,
        0,
        0, // x,y offset relative to plane
        None, // begx, begy, lenx, leny offset of the rendered section
        None, // pixel cell offset
        NcBlitter::PIXEL, // Glyph set to use
        0,                // bitmask over NCVISUAL_OPTION_*
        0,                // transparent color
    );

    // Render the visual in the virtual space
    visual.blit(nc, Some(&opts))?;

    // Render the virtual space in the real terminal space
    nc.render()?;

    // Alocate an input struct
    let mut ni: NcInput = NcInput::new_empty();

    loop {
        // Wait until keypress (instead of sleeping at each loop)
        let keypress: NcReceived = nc.get_blocking(Some(&mut ni))?;

        // Discriminate key pressed to take action
        match keypress {
            NcReceived::Char(ch) => {
                match ch {
                    // Q => quit
                    'q' | 'Q' => {
                        break;
                    }
                    _ => (),
                }
            }
            NcReceived::Event(ev) => match ev {
                NcKey::ENTER => break,
                _ => (),
            },
            _ => (),
        }

        // Render again
        nc.render()?;
    }

    // Destroy visual
    visual.destroy();

    // Restore the terminal context
    nc.stop()?;

    // Say goodbye
    println!("Goodbye from notcurses pixel rendered poc (rust binding)");

    // Return success => 0
    Ok(())
}
