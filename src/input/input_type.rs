//!

use super::constants;

/// The type of the input event.
///
/// Note:
/// *Unknown* and *Press* are considered equivalent for the purposes of `PartialEq`.
#[derive(Clone, Copy, Debug)]
pub enum NcInputType {
    ///
    Unknown,

    ///
    Press,

    ///
    Repeat,

    ///
    Release,
}

impl Default for NcInputType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<NcInputType> for u32 {
    fn from(it: NcInputType) -> Self {
        use NcInputType::*;
        match it {
            Unknown => constants::NCINTYPE_UNKNOWN,
            Press => constants::NCINTYPE_PRESS,
            Repeat => constants::NCINTYPE_REPEAT,
            Release => constants::NCINTYPE_RELEASE,
        }
    }
}

impl From<u32> for NcInputType {
    fn from(value: u32) -> Self {
        use NcInputType::*;
        match value {
            constants::NCINTYPE_UNKNOWN => Unknown,
            constants::NCINTYPE_PRESS => Press,
            constants::NCINTYPE_REPEAT => Repeat,
            constants::NCINTYPE_RELEASE => Release,
            _ => Unknown,
        }
    }
}
