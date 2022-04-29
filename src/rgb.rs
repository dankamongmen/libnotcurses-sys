//!

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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NcRgb(pub c_api::NcRgb_u32);

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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NcRgba(pub c_api::NcRgba_u32);

mod std_impls {
    use super::{
        c_api::{NcRgb_u32, NcRgba_u32},
        NcRgb, NcRgba,
    };

    crate::from_primitive![NcRgb, NcRgb_u32];
    crate::unit_impl_from![NcRgb, NcRgb_u32];
    crate::unit_impl_fmt![bases+display; NcRgb];

    impl From<[u8; 3]> for NcRgb {
        fn from(array: [u8; 3]) -> Self {
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

    crate::from_primitive![NcRgba, NcRgba_u32];
    crate::unit_impl_from![NcRgba, NcRgba_u32];
    crate::unit_impl_fmt![bases+display; NcRgba];
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
