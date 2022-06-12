//!

/// The type of the [`NcInput`][crate::NcInput] event.
///
/// Note: *Unknown* and *Press* are considered equivalent.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

mod std_impls {
    use super::{c_api::*, NcInputType};
    use std::fmt;

    impl Default for NcInputType {
        fn default() -> Self {
            Self::Unknown
        }
    }

    impl fmt::Display for NcInputType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcInputType::*;
            write!(
                f,
                "{}",
                match self {
                    Unknown => "Unknown",
                    Press => "Press",
                    Repeat => "Repeat",
                    Release => "Release",
                }
            )
        }
    }

    impl fmt::Debug for NcInputType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "NcInputType::{}", self)
        }
    }

    impl From<NcInputType> for u32 {
        fn from(it: NcInputType) -> Self {
            use NcInputType::*;
            match it {
                Unknown => NCTYPE_UNKNOWN,
                Press => NCTYPE_PRESS,
                Repeat => NCTYPE_REPEAT,
                Release => NCTYPE_RELEASE,
            }
        }
    }

    impl From<u32> for NcInputType {
        fn from(value: u32) -> Self {
            use NcInputType::*;
            match value {
                NCTYPE_UNKNOWN => Unknown,
                NCTYPE_PRESS => Press,
                NCTYPE_REPEAT => Repeat,
                NCTYPE_RELEASE => Release,
                _ => Unknown,
            }
        }
    }
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// The type of the [`NcInput`][crate::NcInput] event.
    ///
    /// It's recommended to use  [`NcInputType`][crate::NcInputType] instead.
    ///
    /// # Associated `c_api` constants:
    /// - [`NCTYPE_UNKNOWN`]
    /// - [`NCTYPE_PRESS`]
    /// - [`NCTYPE_REPEAT`]
    /// - [`NCTYPE_RELEASE`]
    pub type NcInputType_u32 = u32;

    /// [`NcInputType_u32`] *Unknown* input type event.
    pub const NCTYPE_UNKNOWN: u32 = ffi::ncintype_e_NCTYPE_UNKNOWN;

    /// [`NcInputType_u32`] *Press* input type event.
    pub const NCTYPE_PRESS: u32 = ffi::ncintype_e_NCTYPE_PRESS;

    /// [`NcInputType_u32`] *Repeat* input type event.
    pub const NCTYPE_REPEAT: u32 = ffi::ncintype_e_NCTYPE_REPEAT;

    /// [`NcInputType_u32`] *Release* input type event.
    pub const NCTYPE_RELEASE: u32 = ffi::ncintype_e_NCTYPE_RELEASE;
}
