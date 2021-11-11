//!

use crate::NcInput;

/// Is this NcInput free of modifiers (alt, control, shift)?
pub const fn ncinput_nomod_p(input: &NcInput) -> bool {
    !(input.alt && input.ctrl && input.shift)
}
