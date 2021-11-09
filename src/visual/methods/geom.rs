//!

use crate::{NcBlitter, NcBlitterApi, NcVGeom};

/// # NcVGeom Constructors
impl NcVGeom {
    /// Returns a new `NcVGeom` with zeroed fields.
    pub fn new() -> Self {
        Self {
            pixy: 0,
            pixx: 0,
            cdimy: 0,
            cdimx: 0,
            rpixy: 0,
            rpixx: 0,
            rcelly: 0,
            rcellx: 0,
            scaley: 0,
            scalex: 0,
            maxpixely: 0,
            maxpixelx: 0,
            begy: 0,
            begx: 0,
            leny: 0,
            lenx: 0,
            blitter: NcBlitter::DEFAULT,
        }
    }
}
