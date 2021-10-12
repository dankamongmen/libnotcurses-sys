/// Alignment within a plane or terminal (alias of [`u32`]).
///
/// Left/right-justified, or centered.
pub type NcAlign = u32; // crate::bindings::ffi::ncalign_e;

#[allow(unused_imports)] // for doc comments
use crate::NcPlane;

crate::impl_api![
    NcAlign,
    NcAlignApi,
    /// Left alignment within an [`NcPlane`] or terminal.
    const LEFT: NcAlign = constants::NCALIGN_LEFT;,
    /// Right alignment within an [`NcPlane`] or terminal.
    const RIGHT: NcAlign = constants::NCALIGN_RIGHT;,
    /// Top alignment within an [`NcPlane`] or terminal.
    const TOP: NcAlign = constants::NCALIGN_LEFT;,
    /// Bottom alignment within an [`NcPlane`] or terminal.
    const BOTTOM: NcAlign = constants::NCALIGN_RIGHT;,
    /// Center alignment within an [`NcPlane`] or terminal.
    const CENTER: NcAlign = constants::NCALIGN_CENTER;,
    /// Nothing unaligned should appear.
    const UNALIGNED: NcAlign = constants::NCALIGN_UNALIGNED;
];

pub(crate) mod constants {
    use crate::NcAlign;

    #[allow(unused_imports)] // for doc comments
    use crate::NcPlane;

    /// [`NcAlign`] left alignment within an [`NcPlane`] or terminal.
    pub const NCALIGN_LEFT: NcAlign = crate::bindings::ffi::ncalign_e_NCALIGN_LEFT;

    /// [`NcAlign`] Right alignment within an [`NcPlane`] or terminal.
    pub const NCALIGN_RIGHT: NcAlign = crate::bindings::ffi::ncalign_e_NCALIGN_RIGHT;

    /// [`NcAlign`] Top alignment within an [`NcPlane`] or terminal.
    pub const NCALIGN_TOP: NcAlign = NCALIGN_LEFT;

    /// [`NcAlign`] Bottom alignment within an [`NcPlane`] or terminal.
    pub const NCALIGN_BOTTOM: NcAlign = NCALIGN_RIGHT;

    /// [`NcAlign`] Center alignment within an [`NcPlane`] or terminal.
    pub const NCALIGN_CENTER: NcAlign = crate::bindings::ffi::ncalign_e_NCALIGN_CENTER;

    /// [`NcAlign`] Nothing unaligned should appear.
    pub const NCALIGN_UNALIGNED: NcAlign = crate::bindings::ffi::ncalign_e_NCALIGN_UNALIGNED;
}
