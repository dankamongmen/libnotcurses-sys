use crate::{c_api, Nc, NcPlane, NcPlaneOptions};

/// Helper function for a new NcPlane on C style tests.
#[allow(dead_code)]
pub(crate) unsafe fn ncplane_new_test<'a>(
    nc: &mut Nc,
    y: i32,
    x: i32,
    rows: u32,
    cols: u32,
) -> &'a mut NcPlane {
    &mut *c_api::ncpile_create(nc, &NcPlaneOptions::new(y, x, rows, cols))
}

/// Helper function for a new bound NcPlane on C style tests.
#[allow(dead_code)]
pub(crate) unsafe fn ncplane_new_bound_test<'a>(
    plane: &mut NcPlane,
    y: i32,
    x: i32,
    rows: u32,
    cols: u32,
) -> &'a mut NcPlane {
    &mut *c_api::ncplane_create(plane, &NcPlaneOptions::new(y, x, rows, cols))
}
