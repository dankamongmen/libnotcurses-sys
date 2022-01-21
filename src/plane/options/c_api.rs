//!

use crate::{c_api::ffi, NcPlaneOptions};

/// Horizontal alignment relative to the parent plane. Use NcAlign for 'x'.
pub const NCPLANE_OPTION_HORALIGNED: u64 = ffi::NCPLANE_OPTION_HORALIGNED as u64;

/// Vertical alignment relative to the parent plane. Use NcAlign for 'y'.
pub const NCPLANE_OPTION_VERALIGNED: u64 = ffi::NCPLANE_OPTION_VERALIGNED as u64;

/// Maximize relative to the parent plane, modulo the provided margins.
///
/// The margins are best-effort; the plane will always be at least 1 column by
/// 1 row. If the margins can be effected, the plane will be sized to all
/// remaining space. 'y' and 'x' are overloaded as the top and left margins
/// when this flag is used. 'rows' and 'cols' must be 0 when this flag is
/// used. This flag is exclusive with both of the alignment flags.
pub const NCPLANE_OPTION_MARGINALIZED: u64 = ffi::NCPLANE_OPTION_MARGINALIZED as u64;

/// Do not scroll alongside its parent.
///
/// If this plane is bound to a scrolling plane, it ought *not* scroll along
/// with the parent (it will still move with the parent, maintaining its
/// relative position, if the parent is moved to a new location).
pub const NCPLANE_OPTION_FIXED: u64 = ffi::NCPLANE_OPTION_FIXED as u64;

/// Enables automatic growth of the plane to accommodate output.
///
/// Creating a plane with this flag is equivalent to immediately calling
/// `ncplane_set_autogrow(p, true)` following plane creation.
pub const NCPLANE_OPTION_AUTOGROW: u64 = ffi::NCPLANE_OPTION_AUTOGROW as u64;

/// Enables vertical scrolling of the plane to accommodate output.
///
/// Creating a plane with this flag is equivalent to immediately calling
/// `ncplane_set_scrolling(p, true)` following plane creation.
pub const NCPLANE_OPTION_VSCROLL: u64 = ffi::NCPLANE_OPTION_VSCROLL as u64;

/// # Constants
impl NcPlaneOptions {
    /// Horizontal alignment relative to the parent plane. Use NcAlign for 'x'.
    pub const HORALIGNED: u64 = NCPLANE_OPTION_HORALIGNED as u64;

    /// Vertical alignment relative to the parent plane. Use NcAlign for 'y'.
    pub const VERALIGNED: u64 = NCPLANE_OPTION_VERALIGNED as u64;

    /// Maximize relative to the parent plane, modulo the provided margins.
    ///
    /// The margins are best-effort; the plane will always be at least 1 column by
    /// 1 row. If the margins can be effected, the plane will be sized to all
    /// remaining space. 'y' and 'x' are overloaded as the top and left margins
    /// when this flag is used. 'rows' and 'cols' must be 0 when this flag is
    /// used. This flag is exclusive with both of the alignment flags.
    pub const MARGINALIZED: u64 = NCPLANE_OPTION_MARGINALIZED as u64;

    /// Do not scroll alongside its parent.
    ///
    /// If this plane is bound to a scrolling plane, it ought *not* scroll along
    /// with the parent (it will still move with the parent, maintaining its
    /// relative position, if the parent is moved to a new location).
    pub const FIXED: u64 = NCPLANE_OPTION_FIXED as u64;

    /// Enables automatic growth of the plane to accommodate output.
    ///
    /// Creating a plane with this flag is equivalent to immediately calling
    /// `NcPlane::set_autogrow(true)` following plane creation.
    pub const AUTOGROW: u64 = NCPLANE_OPTION_AUTOGROW as u64;

    /// Enables vertical scrolling of the plane to accommodate output.
    ///
    /// Creating a plane with this flag is equivalent to immediately calling
    /// `NcPlane::set_scrolling(true)` following plane creation.
    pub const VSCROLL: u64 = NCPLANE_OPTION_VSCROLL as u64;
}
