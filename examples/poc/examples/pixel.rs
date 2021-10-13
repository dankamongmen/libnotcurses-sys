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
    NcResult,
    NcError,
    // Plane
    NcPlane,
    NcPlaneOptions,
    // Visual
    NcVisual,
    NcVisualOptions,
    NcScale,
    NcScaleApi,
    NcBlitter,
    NcBlitterApi,
    // Input
    NcInput,
    NcKey,
};


fn main() -> NcResult<()> {
    // Parse argument in
    let filepath: String = std::env::args().nth(1).expect("Error: Must pass image file path as first argument. Usage: pixel image.png");

    // Init notcurses context
    let nc: &mut Nc = Nc::new()?;

    // Clause: Pixel must be supported
    if 0 == nc.check_pixel_support() {
        nc.stop()?;
        return Err(NcError::with_msg(1, "Error: This program requires pixel graphics support"));
    }

    // Obtain reference to visual (something like an image)
    let visual: &mut NcVisual = NcVisual::from_file(&filepath)?;

    // Create full screen plane
    // -- Notcurses report an error if image is draw on the stdplane
    let stdplane: &mut NcPlane = nc.stdplane();
    let (row, col) = stdplane.dim_yx();
    let planeopts: NcPlaneOptions = NcPlaneOptions::new(0, 0, row, col);
    let pixelplane: &mut NcPlane = NcPlane::with_options_bound(stdplane, planeopts)?;

    // Craft some visual options (here full screen)
    let opts: NcVisualOptions = NcVisualOptions::with_plane(
        pixelplane,
        NcScale::SCALE,
        0, 0,  // x,y offset relative to plane
        0, 0,  // begx, begy offset of the rendered section
        0, 0,  // sizex, sizey: (0,0) => full plane
        NcBlitter::PIXEL,  // Glyph set to use
        0,  // bitmask over NCVISUAL_OPTION_*
        0,  // transparent color
    );
        
    // Render the visual in the virtual space
    visual.render(nc, &opts)?;

    // Render the virtual space in the real terminal space
    nc.render()?;

    // Alocate an input struct
    let mut ni: NcInput = NcInput::new_empty();

    loop {
        // Wait until keypress (instead of sleeping at each loop)
        let keypress: char = nc.getc_blocking(Some(&mut ni))?;

        // Discriminate key pressed to take action
        match keypress {
            // Q => quit
            'q' | 'Q' | NcKey::ENTER => {
                break;
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
