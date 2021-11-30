use crate::{
    c_api::{NCKEY_EOF, PRETERUNICODEBASE},
    NcInput, NcKey,
};

/// Is the event a synthesized mouse event?
#[inline]
pub const fn nckey_mouse_p(r: u32) -> bool {
    r >= NcKey::MOTION.0 && r <= NcKey::BUTTON11.0
}

/// Compares two NcInput structs for data equality.
///
/// Returns true if the two are data-equivalent.
pub const fn ncinput_equal_p(n1: NcInput, n2: NcInput) -> bool {
    if n1.id != n2.id {
        return false;
    }
    if n1.y != n2.y || n1.x != n2.x {
        return false;
    }
    if n1.alt != n2.alt || n1.shift != n2.shift || n1.ctrl != n2.ctrl {
        return false;
    }
    if n1.evtype != n2.evtype {
        return false;
    }
    true
}

/// Is this `u32` number a synthesized event?
///
/// Includes the 300 numbers from [`NcKey::PRETERUNICODEBASE`] on up and `ESC`.
pub const fn nckey_synthesized_p(num: u32) -> bool {
    num >= PRETERUNICODEBASE && num <= NCKEY_EOF
}
