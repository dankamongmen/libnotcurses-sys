use crate::{NcInput, NCKEY_BUTTON1, NCKEY_BUTTON11};

/// Are all the modifiers off (alt, control, shift)?
pub const fn ncinput_nomod_p(input: &NcInput) -> bool {
    !input.alt && !input.ctrl && !input.shift
}

/// Is this [char] a Supplementary Private Use Area-B codepoint?
///
/// Links:
/// - <https://en.wikipedia.org/wiki/Private_Use_Areas>
/// - <https://codepoints.net/supplementary_private_use_area-b>
#[inline]
pub const fn nckey_supppuab_p(w: char) -> bool {
    w as u32 >= 0x100000_u32 && w as u32 <= 0x10fffd_u32
}

/// Is the event a synthesized mouse event?
#[inline]
pub const fn nckey_mouse_p(r: char) -> bool {
    r >= NCKEY_BUTTON1 && r <= NCKEY_BUTTON11
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
