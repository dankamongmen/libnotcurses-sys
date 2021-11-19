// found issues relative to visual geometry

use libnotcurses_sys::*;
use std::ptr::null_mut;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };

    let stdp = nc.stdplane();
    stdp.set_scrolling(true);

    // # ISSUE 1
    // When providing NcBlitter::DEFAULT it always picks NcBlitter::HALF
    // even if NcBlitter::QUADRANT is supported

    let vo = NcVisualOptions::new(
        None,
        NcScale::NONE_HIRES,
        0,
        0,
        None,
        None,
        NcBlitter::DEFAULT,
        0,
        0,
    );

    // let vg = nc.visual_geom(None, Some(&vo))?;
    let mut vg = NcVGeom::new();
    unsafe { crate::c_api::ncvisual_geom(nc, null_mut(), &vo, &mut vg) };

    putstrln!(stdp, "{:?}\n", vg)?;

    let (h, w) = stdp.dim_yx();
    let (y, x) = (h * vg.scaley, w * vg.scalex);
    let buffer: Vec<u8> = vec![200; (y * x) as usize * 3];

    let vo = NcVisualOptions::new(
        None,
        NcScale::NOSCALE,
        0,
        0,
        None,
        None,
        NcBlitter::PIXEL,
        0,
        0,
    );
    let visual = NcVisual::from_rgb_packed(buffer.as_slice(), y, x * 3, x, 255)?;

    // # ISSUE 2
    // `NcVGeom` created from `ncvisual_geom` is missing `maxpixely`:

    // let vgeom = nc.visual_geom(Some(visual), Some(&vo))?;
    // putstrln!(stdp, "{:?}\n{:?}", vgeom, vgeom.blitter_name())?;
    let mut vg = NcVGeom::new();
    unsafe { crate::c_api::ncvisual_geom(nc, visual, &vo, &mut vg) };

    putstrln!(stdp, "{:?}", vg)?;

    visual.destroy();
    unsafe { nc.stop()? };
    Ok(())
}
