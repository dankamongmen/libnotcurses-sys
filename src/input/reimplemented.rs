//!

use crate::{NcInput, NcInputType, NcKeyMod};

/// Is this `NcInput` free of modifiers (alt, control, shift)?
///
/// *Method: NcInput.[nomod_p()][NcInput#method.nomod_p].*
pub fn ncinput_nomod_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).none_p()
}

/// Returns true if the `Shift` modifier is present.
///
/// *Method: NcInput.[shift_p()][NcInput#method.shift_p].*
pub fn ncinput_shift_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).shift_p()
}

/// Returns true if the `Alt` modifier is present.
///
/// *Method: NcInput.[alt_p()][NcInput#method.alt_p].*
pub fn ncinput_alt_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).alt_p()
}

/// Returns true if the `Ctrl` modifier is present.
///
/// *Method: NcInput.[ctrl_p()][NcInput#method.ctrl_p].*
pub fn ncinput_ctrl_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).ctrl_p()
}

/// Returns true if the `Meta` modifier is present.
///
/// *Method: NcInput.[meta_p()][NcInput#method.meta_p].*
pub fn ncinput_meta_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).meta_p()
}

/// Returns true if the `Super` modifier is present.
///
/// *Method: NcInput.[super_p()][NcInput#method.super_p].*
pub fn ncinput_super_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).super_p()
}

/// Returns true if the `Hyper` modifier is present.
///
/// *Method: NcInput.[hyper_p()][NcInput#method.hyper_p].*
pub fn ncinput_hyper_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).hyper_p()
}

/// Returns true if the `CapsLock` modifier is present.
///
/// *Method: NcInput.[capslock_p()][NcInput#method.capslock_p].*
pub fn ncinput_capslock_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).capslock_p()
}

/// Returns true if the `NumLock` modifier is present.
///
/// *Method: NcInput.[numlock_p()][NcInput#method.numlock_p].*
pub fn ncinput_numlock_p(input: &NcInput) -> bool {
    NcKeyMod::from(input.modifiers).numlock_p()
}

/// Returns true if the two `NcInput` are data-equivalent.
///
/// *Method: NcInput.[equal_p()][NcInput#method.equal_p].*
pub fn ncinput_equal_p(n1: &NcInput, n2: &NcInput) -> bool {
    if n1.id != n2.id {
        return false;
    }
    if n1.y != n2.y || n1.x != n2.x {
        return false;
    }
    if (n1.modifiers & !(NcKeyMod::CapsLock | NcKeyMod::NumLock))
        != (n2.modifiers & !(NcKeyMod::CapsLock | NcKeyMod::NumLock))
    {
        return false;
    }
    if n1.evtype != n2.evtype
        && ((n1.evtype != NcInputType::Unknown as u32 && n1.evtype != NcInputType::Press as u32)
            || (n2.evtype != NcInputType::Unknown as u32 && n2.evtype != NcInputType::Press as u32))
    {
        return false;
    }
    if n1.ypx != n2.ypx || n1.xpx != n2.xpx {
        return false;
    }
    true
}
