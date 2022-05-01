//!

use crate::NcKey;

/// A received character or event.
///
/// # Default
/// *[`NcReceived::NoInput`]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NcReceived {
    /// No input was received
    ///
    /// A `0x00` (NUL) was received, meaning no input.
    NoInput,

    /// A synthesized event was received.
    Key(NcKey),

    /// A valid [`char`] was received.
    Char(char),
}

mod std_impls {
    use crate::{NcInput, NcKey, NcReceived};
    use std::fmt;

    impl Default for NcReceived {
        fn default() -> Self {
            Self::NoInput
        }
    }

    impl fmt::Display for NcReceived {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcReceived::*;
            let string = match self {
                Key(k) => format!["{k}"],
                Char(c) => format!["{c:?}"],
                NoInput => "No".to_string(),
            };
            write!(f, "{}", string)
        }
    }
    impl fmt::Debug for NcReceived {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcReceived::*;
            let string = match self {
                Key(k) => format!["Key({k})"],
                Char(c) => format!["Char({c:?})"],
                NoInput => "NoInput".to_string(),
            };
            write!(f, "Received::{}", string)
        }
    }

    impl From<NcInput> for NcReceived {
        fn from(i: NcInput) -> Self {
            Self::from(i.id)
        }
    }
    impl From<&NcInput> for NcReceived {
        fn from(i: &NcInput) -> Self {
            Self::from(i.id)
        }
    }
    impl From<&mut NcInput> for NcReceived {
        fn from(i: &mut NcInput) -> Self {
            Self::from(i.id)
        }
    }

    impl PartialEq for NcInput {
        fn eq(&self, other: &Self) -> bool {
            self.equal_p(other)
        }
    }

    impl From<NcReceived> for u32 {
        fn from(r: NcReceived) -> Self {
            use NcReceived::*;
            match r {
                Char(c) => c.into(),
                Key(k) => k.into(),
                NoInput => 0,
            }
        }
    }

    impl From<u32> for NcReceived {
        fn from(num: u32) -> Self {
            use NcReceived::*;
            if num == 0 {
                NoInput
            } else if NcKey::is(num) {
                Key(NcKey::new(num).unwrap())
            } else if let Some(c) = core::char::from_u32(num) {
                Char(c)
            } else {
                unreachable!("NcReceived::from({}) not a char", num);
            }
        }
    }
}
