use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let ncd = NcDirect::new()?;

    let (t_rows, t_cols) = ncd.dim_yx();
    println!("Terminal rows={0}, cols={1}", t_rows, t_cols);

    println!(
        "Can display UTF-8: {0}
Can open images: {1}
Supports Pixels: {2:?}
Palette size: {3:?}
",
        ncd.canutf8(),
        ncd.canopen_images(),
        ncd.check_pixel_support(),
        ncd.palette_size(),
    );

    ncd.stop()?;
    Ok(())
}
