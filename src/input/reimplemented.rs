//!

use crate::{NcInput, NcKeyMod};

/// Is this `NcInput` free of modifiers (alt, control, shift)?
///
/// *Method: NcInput.[nomod_p()][NcInput#method.nomod_p].*
pub const fn ncinput_nomod_p(input: &NcInput) -> bool {
    input.modifiers == NcKeyMod::None as u32
}

/// Returns true if the `Shift` modifier is present.
///
/// *Method: NcInput.[shift_p()][NcInput#method.shift_p].*
pub const fn ncinput_shift_p(input: &NcInput) -> bool {
    (input.modifiers & NcKeyMod::Shift as u32) != 0
}

/// Returns true if the `Ctrl` modifier is present.
///
/// *Method: NcInput.[ctrl_p()][NcInput#method.ctrl_p].*
pub const fn ncinput_ctrl_p(input: &NcInput) -> bool {
    (input.modifiers & NcKeyMod::Ctrl as u32) != 0
}

/// Returns true if the `Alt` modifier is present.
///
/// *Method: NcInput.[alt_p()][NcInput#method.alt_p].*
pub const fn ncinput_alt_p(input: &NcInput) -> bool {
    (input.modifiers & NcKeyMod::Alt as u32) != 0
}

/// Returns true if the `Meta` modifier is present.
///
/// *Method: NcInput.[meta_p()][NcInput#method.meta_p].*
pub const fn ncinput_meta_p(input: &NcInput) -> bool {
    (input.modifiers & NcKeyMod::Meta as u32) != 0
}

/// Returns true if the two `NcInput` are data-equivalent.
///
/// *Method: NcInput.[equal_p()][NcInput#method.equal_p].*
//
// NOTE: this is probably not needed, since it already implements `PartialEq`.
pub const fn ncinput_equal_p(n1: &NcInput, n2: &NcInput) -> bool {
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
