//!

use super::c_api;

/// The type of the [`NcInput`][crate::NcInput] event.
///
/// Note:
/// *Unknown* and *Press* are considered equivalent for the purposes of `PartialEq`.
#[repr(u32)]
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
            Unknown => c_api::NCINTYPE_UNKNOWN,
            Press => c_api::NCINTYPE_PRESS,
            Repeat => c_api::NCINTYPE_REPEAT,
            Release => c_api::NCINTYPE_RELEASE,
        }
    }
}

impl From<u32> for NcInputType {
    fn from(value: u32) -> Self {
        use NcInputType::*;
        match value {
            c_api::NCINTYPE_UNKNOWN => Unknown,
            c_api::NCINTYPE_PRESS => Press,
            c_api::NCINTYPE_REPEAT => Repeat,
            c_api::NCINTYPE_RELEASE => Release,
            _ => Unknown,
        }
    }
}
