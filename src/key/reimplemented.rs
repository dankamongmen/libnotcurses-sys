//!

use crate::{
    c_api::{NCKEY_EOF, PRETERUNICODEBASE},
    NcKey,
};

/// Is the event a synthesized mouse event?
#[inline]
pub const fn nckey_mouse_p(r: u32) -> bool {
    r >= NcKey::Motion.0 && r <= NcKey::Button11.0
}

/// Is this `u32` number a synthesized event?
///
/// Includes the 300 numbers from [`PRETERUNICODEBASE`] on up and `ESC`.
pub const fn nckey_synthesized_p(num: u32) -> bool {
    num >= PRETERUNICODEBASE && num <= NCKEY_EOF
}
