//! Test `NcCell` methods and associated functions.

use crate::{Nc, NcCell, NcChannels, NcChannelsApi, NcPlane, NcStyle, NcStyleApi};

use serial_test::serial;

#[test]
#[serial]
fn constructors() -> crate::NcResult<()> {
    let nc = unsafe { Nc::new()? };
    let plane = NcPlane::new(nc, 0, 0, 10, 10)?;

    // new empty cell
    let mut cell = NcCell::new();

    // only ASCII characters
    assert![NcCell::from_char7b('e').is_ok()];
    assert![NcCell::from_char7b('é').is_err()];

    // duplication
    let c1 = NcCell::from_char7b('a')?;
    let c2 = NcCell::from_char7b('b')?;
    assert_ne![c1, c2];
    let mut c1d = c1.duplicate(plane)?;
    assert_eq![c1d, c1];

    // initialization
    c1d.init();
    assert_ne![c1d, c1];
    assert_eq![c1d, NcCell::new()];

    // from `char`
    assert![NcCell::from_char(plane, 'é').is_ok()];
    #[cfg(not(target_os = "macos"))] // FIXME
    assert![NcCell::from_char(plane, '௵').is_ok()];

    // from `&str`
    assert![NcCell::from_str(plane, "௵").is_ok()];
    let c = NcCell::from_str(plane, "←↓→");
    assert![c.is_ok()];
    // assert![c] // TODO:CHECK == '←'

    // `load` returns the number of bytes copied
    assert_eq![1, NcCell::load(plane, &mut cell, "e")?];
    assert_eq![2, NcCell::load(plane, &mut cell, "é")?];
    assert_eq![3, NcCell::load(plane, &mut cell, "௵")?];
    assert_eq![4, NcCell::load(plane, &mut cell, "🚀")?];
    // person face palming emoji
    // + emoji skintone modifier
    // + ZERO WIDTH JOINER
    // + male emoji
    assert_eq![17, NcCell::load(plane, &mut cell, "🤦🏼‍♂️")?];

    // `prime`
    assert_eq![
        4,
        NcCell::prime(
            plane,
            &mut cell,
            "🚀",
            NcStyle::UNDERLINE,
            NcChannels::from_rgb_both(0x112233)
        )?
    ];
    assert_eq![NcStyle::UNDERLINE, cell.styles()];
    assert_eq![NcChannels::from_rgb_both(0x112233), cell.channels(plane)];

    // cleanup
    cell.release(plane);
    plane.destroy()?;
    unsafe { nc.stop()? };
    Ok(())
}
