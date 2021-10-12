/// Styling attribute flags (alias of [`u16`]).
///
/// ## Diagram
///
/// ```txt
/// 11111111 11111111
/// ```
///
/// `type in C:  uint16_t`
///
pub type NcStyle = u16;

/// Enables the [`NcStyle`] associated methods and constants.
pub trait NcStyleApi {
    const MASK: u16 = constants::NCSTYLE_MASK as u16;
    const ITALIC: u16 = constants::NCSTYLE_ITALIC as u16;
    const UNDERLINE: u16 = constants::NCSTYLE_UNDERLINE as u16;
    const UNDERCURL: u16 = constants::NCSTYLE_UNDERCURL as u16;
    const STRUCK: u16 = constants::NCSTYLE_STRUCK as u16;
    const BOLD: u16 = constants::NCSTYLE_BOLD as u16;
    const NOSTYLE: u16 = constants::NCSTYLE_NONE as u16;

    fn add(&mut self, other_style: NcStyle);
    fn has(&self, other: NcStyle) -> bool;
    fn to_vec(&self) -> Vec<NcStyle>;
}

impl NcStyleApi for NcStyle {
    /// Returns a `Vec` with all the styles contained in the current style.
    fn to_vec(&self) -> Vec<NcStyle> {
        let mut v = vec![];
        let styles = [
            NcStyle::ITALIC,
            NcStyle::UNDERLINE,
            NcStyle::UNDERCURL,
            NcStyle::STRUCK,
            NcStyle::BOLD,
            NcStyle::NOSTYLE,
        ];
        for s in &styles {
            if self.has(*s) {
                v.push(*s)
            }
        }
        v
    }

    /// Returns true if the current style has included the `other_style`.
    fn has(&self, other_style: NcStyle) -> bool {
        (self & other_style) == other_style
    }

    /// Adds the `other_style` to the current style.
    fn add(&mut self, other_style: NcStyle) {
        *self |= other_style
    }
}

pub(crate) mod constants {
    ///
    pub const NCSTYLE_MASK: u16 = crate::bindings::ffi::NCSTYLE_MASK as u16;

    ///
    pub const NCSTYLE_ITALIC: u16 = crate::bindings::ffi::NCSTYLE_ITALIC as u16;

    ///
    pub const NCSTYLE_UNDERLINE: u16 = crate::bindings::ffi::NCSTYLE_UNDERLINE as u16;

    ///
    pub const NCSTYLE_UNDERCURL: u16 = crate::bindings::ffi::NCSTYLE_UNDERCURL as u16;

    ///
    pub const NCSTYLE_STRUCK: u16 = crate::bindings::ffi::NCSTYLE_STRUCK as u16;

    ///
    pub const NCSTYLE_BOLD: u16 = crate::bindings::ffi::NCSTYLE_BOLD as u16;

    ///
    pub const NCSTYLE_NONE: u16 = crate::bindings::ffi::NCSTYLE_NONE as u16;
}
