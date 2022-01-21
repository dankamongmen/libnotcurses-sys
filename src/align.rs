//! `NcAlign`

/// Alignment within an [`NcPlane`][crate::NcPlane] or terminal.
///
/// - `Left`|`Right` justified (horizontally).
/// - `Top`|`Down` justified (vertically).
/// - `Centered` (both horizontally & vertically).
/// - `Unaligned` for an invalid state.
///
/// # Default
/// *[`NcAlign::Left`]/[`Top`][NcAlign::Top]*
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcAlign {
    /// Nothing unaligned should be rendered.
    Unaligned = c_api::NCALIGN_UNALIGNED,
    /// Left (==[`Top`][NcAlign::Top]) alignment.
    Left = c_api::NCALIGN_LEFT,
    /// Center alignment.
    Center = c_api::NCALIGN_CENTER,
    /// Right (==[`Bottom`][NcAlign::Bottom]) alignment.
    Right = c_api::NCALIGN_RIGHT,
}

mod std_impls {
    use super::{c_api, NcAlign};
    use std::fmt;

    impl Default for NcAlign {
        fn default() -> Self {
            Self::Left
        }
    }

    /// # Aliases
    impl NcAlign {
        /// Top (==[`Left`][NcAlign::Left]) alignment.
        pub const Top: NcAlign = NcAlign::Left;
        /// Bottom (==[`Right`][NcAlign::Right]) alignment.
        pub const Bottom: NcAlign = NcAlign::Right;
    }

    impl fmt::Display for NcAlign {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use NcAlign::*;
            write!(
                f,
                "{}",
                match self {
                    Left => "Left",
                    Center => "Center",
                    Right => "Right",
                    Unaligned => "Unaligned",
                }
            )
        }
    }

    impl From<c_api::NcAlign_u32> for NcAlign {
        fn from(align: c_api::NcAlign_u32) -> Self {
            use {c_api::*, NcAlign::*};
            match align {
                NCALIGN_LEFT => Left,
                NCALIGN_CENTER => Center,
                NCALIGN_RIGHT => Right,
                NCALIGN_UNALIGNED => Unaligned,
                _ => Unaligned, // invalid values default to `Unaligned`
            }
        }
    }

    impl From<NcAlign> for c_api::NcAlign_u32 {
        fn from(align: NcAlign) -> Self {
            use {c_api::*, NcAlign::*};
            match align {
                Left => NCALIGN_LEFT,
                Center => NCALIGN_CENTER,
                Right => NCALIGN_RIGHT,
                Unaligned => NCALIGN_UNALIGNED,
            }
        }
    }

    impl From<i32> for NcAlign {
        fn from(align: i32) -> Self {
            use {c_api::*, NcAlign::*};

            const NCALIGN_LEFT_i32: i32 = NCALIGN_LEFT as i32;
            const NCALIGN_CENTER_i32: i32 = NCALIGN_CENTER as i32;
            const NCALIGN_RIGHT_i32: i32 = NCALIGN_RIGHT as i32;
            const NCALIGN_UNALIGNED_i32: i32 = NCALIGN_UNALIGNED as i32;

            match align {
                NCALIGN_LEFT_i32 => Left,
                NCALIGN_CENTER_i32 => Center,
                NCALIGN_RIGHT_i32 => Right,
                NCALIGN_UNALIGNED_i32 => Unaligned,
                _ => Unaligned,
            }
        }
    }

    impl From<NcAlign> for i32 {
        fn from(align: NcAlign) -> Self {
            use {c_api::*, NcAlign::*};
            match align {
                Left => NCALIGN_LEFT as i32,
                Center => NCALIGN_CENTER as i32,
                Right => NCALIGN_RIGHT as i32,
                Unaligned => NCALIGN_UNALIGNED as i32,
            }
        }
    }
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    /// Alignment within an [`NcPlane`][crate::NcPlane] or terminal.
    ///
    /// It's recommended to use [`NcAlign`][crate::NcAlign] instead.
    ///
    /// # Associated `c_api` constants
    ///
    /// - [`NCALIGN_LEFT`]
    /// - [`NCALIGN_RIGHT`]
    /// - [`NCALIGN_TOP`]
    /// - [`NCALIGN_BOTTOM`]
    /// - [`NCALIGN_CENTER`]
    /// - [`NCALIGN_UNALIGNED`]
    pub type NcAlign_u32 = ffi::ncalign_e;

    /// [`NcAlign_u32`] Left alignment.
    pub const NCALIGN_LEFT: NcAlign_u32 = ffi::ncalign_e_NCALIGN_LEFT;

    /// [`NcAlign_u32`] Right alignment.
    pub const NCALIGN_RIGHT: NcAlign_u32 = ffi::ncalign_e_NCALIGN_RIGHT;

    /// [`NcAlign_u32`] Top alignment.
    pub const NCALIGN_TOP: NcAlign_u32 = NCALIGN_LEFT;

    /// [`NcAlign_u32`] Bottom alignment.
    pub const NCALIGN_BOTTOM: NcAlign_u32 = NCALIGN_RIGHT;

    /// [`NcAlign_u32`] Center alignment.
    pub const NCALIGN_CENTER: NcAlign_u32 = ffi::ncalign_e_NCALIGN_CENTER;

    /// [`NcAlign_u32`] Nothing unaligned should be rendered.
    pub const NCALIGN_UNALIGNED: NcAlign_u32 = ffi::ncalign_e_NCALIGN_UNALIGNED;
}
