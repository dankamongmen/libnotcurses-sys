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
#[derive(Clone, Copy, Debug, PartialEq, Default)]
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
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct NcRgba(pub c_api::NcRgba_u32);

mod std_impls {
    use super::{
        c_api::{NcRgb_u32, NcRgba_u32},
        NcRgb, NcRgba,
    };

    crate::from_primitive![NcRgb, NcRgb_u32];
    crate::unit_impl_from![NcRgb, NcRgb_u32];
    crate::unit_impl_fmt![bases+display; NcRgb];

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
