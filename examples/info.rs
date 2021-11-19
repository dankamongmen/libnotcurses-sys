use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let splane = nc.stdplane();
    splane.set_scrolling(true);

    putstrln!(splane, "ENVIRONMENT\n-----------")?;
    putstrln!(splane, "notcurses version: {}", Nc::version())?;
    putstrln!(splane, "terminal name: {}", nc.detected_terminal())?;
    putstrln!(splane, "user name: {}", Nc::accountname())?;
    putstrln!(splane, "host name: {}", Nc::hostname())?;
    putstrln!(splane)?;

    putstrln!(splane, "CAPABILITIES\n------------")?;
    putstrln!(
        splane,
        "Can display UTF-8: {0}
Can display braille characters: {1}
Can display sextant characters: {2}
Can display quadrant characters: {3}
Can display half block characters: {4}
Can open images: {5}
Can open videos: {6}
Supports Pixels: {7}
Supports True Color: {8}
Supports fading: {9}
Supports changing the palette: {10}
Palette size: {11:?}
",
        nc.canutf8(),
        nc.canbraille(),
        nc.cansextant(),
        nc.canquadrant(),
        nc.canhalfblock(),
        nc.canopen_images(),
        nc.canopen_videos(),
        nc.canpixel(),
        nc.cantruecolor(),
        nc.canfade(),
        nc.canchangecolor(),
        nc.palette_size(),
    )?;

    putstrln!(splane, "GEOMETRY\n------------")?;
    let (t_rows, t_cols) = nc.term_dim_yx();
    putstrln!(
        splane,
        "Terminal dimensions: rows={0}, cols={1}",
        t_rows,
        t_cols
    )?;
    let pgeom = nc.stdplane().pixel_geom();
    putstrln!(splane, "{:#?}.", pgeom)?;

    // Ask for an `NcVisualGeometry` without an `NcVisual`.
    //
    // TODO:WIP (use for unit tests)

    // let _vopts = NcVisualOptions::default();
    let _vopts = NcVisualOptions::new(
        None,
        0,
        1,
        1,
        Some((4, 4, 8, 8)),
        None,
        NcBlitter::ASCII,
        0,
        0,
    );

    let vg = nc.visual_geom(None, None)?; // no visual, no visualoptions
                                          // let vg = nc.visual_geom(None, &vopts)?; // no visualoptions (same response)
    putstrln!(splane, "{:#?}.", vg)?;
    putstrln!(
        splane,
        "(blitter `{}` = {:?})",
        vg.blitter,
        vg.blitter_name()
    )?;

    // Ask for an NcVisualGeometry without Notcurses (from an ncvisual)
    // ...

    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
