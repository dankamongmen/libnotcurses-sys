//!

use c_api::{NcRgb_u32, NcRgba_u32};

/// 24 bits broken into 3x RGB components.
///
/// Unlike with [`NcChannel`], operations involving `NcRgb`
/// ignores the last 4th byte (the alpha component).
///
/// ## Diagram
///
/// ```txt
/// -------- RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
/// `type in C: no data type`
///
/// See also: [`NcRgba`] and [`NcChannel`] types.
///
/// [`NcChannel`]: crate::NcChannel
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct NcRgb(pub c_api::NcRgb_u32);
impl NcRgb {
    /// New const RGB color.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self((r as NcRgb_u32) << 16 | (g as NcRgb_u32) << 8 | b as NcRgb_u32)
    }
}

/// 32 bits broken into 3x RGB components + alpha component.
///
/// ## Diagram
///
/// ```txt
/// AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
/// `type in C: no data type`
///
/// See also: [`NcRgb`] and [`NcChannel`] types.
///
/// [`NcRgba`]: crate::NcRgba
/// [`NcChannel`]: crate::NcChannel
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct NcRgba(pub c_api::NcRgba_u32);
impl NcRgba {
    /// New const RGBA color.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(
            (a as NcRgba_u32) << 24
                | (r as NcRgba_u32) << 16
                | (g as NcRgba_u32) << 8
                | b as NcRgba_u32,
        )
    }
}
mod core_impls {
    use super::{
        c_api::{NcRgb_u32, NcRgba_u32},
        NcRgb, NcRgba,
    };
    use core::fmt;

    crate::from_primitive![NcRgb, NcRgb_u32];
    crate::unit_impl_from![NcRgb, NcRgb_u32];
    crate::unit_impl_fmt![bases; NcRgb];

    impl fmt::Display for NcRgb {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:06X}")
        }
    }
    impl fmt::Debug for NcRgb {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "NcRgb({self})")
        }
    }

    impl From<[u8; 3]> for NcRgb {
        fn from(array: [u8; 3]) -> Self {
            // u32::from_be_bytes(array).into()
            Self(
                (array[0] as NcRgb_u32) << 16
                    | (array[1] as NcRgb_u32) << 8
                    | array[2] as NcRgb_u32,
            )
        }
    }
    impl From<&[u8; 3]> for NcRgb {
        fn from(array: &[u8; 3]) -> Self {
            Self(
                (array[0] as NcRgb_u32) << 16
                    | (array[1] as NcRgb_u32) << 8
                    | array[2] as NcRgb_u32,
            )
        }
    }
    impl From<NcRgb> for [u8; 3] {
        #[inline]
        fn from(rgb: NcRgb) -> Self {
            [
                ((rgb.0 & 0xff0000) >> 16) as u8,
                ((rgb.0 & 0x00ff00) >> 8) as u8,
                (rgb.0 & 0x0000ff) as u8,
            ]
        }
    }
    impl From<NcRgb> for (u8, u8, u8) {
        #[inline]
        fn from(rgb: NcRgb) -> Self {
            (
                ((rgb.0 & 0xff0000) >> 16) as u8,
                ((rgb.0 & 0x00ff00) >> 8) as u8,
                (rgb.0 & 0x0000ff) as u8,
            )
        }
    }
    impl From<(u8, u8, u8)> for NcRgb {
        fn from(tuple: (u8, u8, u8)) -> Self {
            Self((tuple.0 as NcRgb_u32) << 16 | (tuple.1 as NcRgb_u32) << 8 | tuple.2 as NcRgb_u32)
        }
    }

    //

    crate::from_primitive![NcRgba, NcRgba_u32];
    crate::unit_impl_from![NcRgba, NcRgba_u32];
    crate::unit_impl_fmt![bases; NcRgba];

    impl fmt::Display for NcRgba {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:08X}")
        }
    }
    impl fmt::Debug for NcRgba {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "NcRgba({self})")
        }
    }

    /// [R, G, B, A]
    impl From<[u8; 4]> for NcRgba {
        fn from(array: [u8; 4]) -> Self {
            u32::from_be_bytes(array).into()
        }
    }
    impl From<&[u8; 4]> for NcRgba {
        fn from(array: &[u8; 4]) -> Self {
            u32::from_be_bytes(*array).into()
        }
    }
    /// [R, G, B, A]
    impl From<NcRgba> for [u8; 4] {
        #[inline]
        fn from(rgba: NcRgba) -> Self {
            rgba.0.to_be_bytes()
        }
    }

    /// (R, G, B, A)
    impl From<(u8, u8, u8, u8)> for NcRgba {
        fn from(tuple: (u8, u8, u8, u8)) -> Self {
            u32::from_be_bytes([tuple.0, tuple.1, tuple.2, tuple.3]).into()
        }
    }
    /// (R, G, B, A)
    impl From<NcRgba> for (u8, u8, u8, u8) {
        #[inline]
        fn from(rgba: NcRgba) -> Self {
            let a = rgba.0.to_be_bytes();
            (a[0], a[1], a[2], a[3])
        }
    }

    #[cfg(test)]
    mod test {
        use super::{NcRgb, NcRgba};

        #[test]
        fn rgbx_from() {
            let rgb = NcRgb(0x112233_u32);
            let rgb_arr = [0x11, 0x22, 0x33];
            let rgb_tup = (0x11, 0x22, 0x33);

            assert_eq!(rgb, NcRgb::from(rgb_arr));
            assert_eq!(rgb, NcRgb::from(rgb_tup));
            assert_eq!(rgb_arr, <[u8; 3]>::from(rgb));
            assert_eq!(rgb_tup, <(u8, u8, u8)>::from(rgb));

            let rgba = NcRgba(0x112233AA_u32);
            let rgba_arr = [0x11, 0x22, 0x33, 0xAA];
            let rgba_tup = (0x11, 0x22, 0x33, 0xAA);

            assert_eq!(rgba, NcRgba::from(rgba_arr));
            assert_eq!(rgba, NcRgba::from(rgba_tup));
            assert_eq!(rgba_arr, <[u8; 4]>::from(rgba));
            assert_eq!(rgba_tup, <(u8, u8, u8, u8)>::from(rgba));
        }
    }
}

pub(crate) mod c_api {
    /// 24 bits broken into 3x RGB components.
    ///
    /// It's recommended to use [`NcRgb`] instead.
    ///
    /// Unlike with [`NcChannel_u32`], operations involving `NcRgb_u32`
    /// ignores the last 4th byte (the alpha component).
    ///
    /// ## Diagram
    ///
    /// ```txt
    /// -------- RRRRRRRR GGGGGGGG BBBBBBBB
    /// ```
    /// `type in C: no data type`
    ///
    /// See also: [`NcRgba_u32`] and [`NcChannel_u32`] types.
    ///
    /// [`NcRgb`]: crate::NcRgb
    /// [`NcChannel_u32`]: crate::c_api::NcChannel_u32
    pub type NcRgb_u32 = u32;

    /// 32 bits broken into 3x RGB components plus one alpha component.
    ///
    /// It's recommended to use [`NcRgba`] instead.
    ///
    /// ## Diagram
    ///
    /// ```txt
    /// AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB
    /// ```
    /// `type in C: no data type`
    ///
    /// See also: [`NcRgb_u32`] and [`NcChannel_u32`] types.
    ///
    /// [`NcRgba`]: crate::NcRgba
    /// [`NcChannel_u32`]: crate::c_api::NcChannel_u32
    pub type NcRgba_u32 = u32;

    // MAYBE?
    // // NcBgra
    // //
    // /// 32 bits broken into 3x 8bpp BGR channels + 8ppp alpha.
    // ///
    // /// ## Diagram
    // ///
    // /// ```txt
    // /// AAAAAAAA BBBBBBBB GGGGGGGG RRRRRRRR
    // /// ```
    // ///
    // /// `type in C: no data type`
    // ///
    // /// See also: [`NcRgba`], [`NcRgb`] and [`NcChannel`] types.
    // pub type NcBgra = u32;
}
