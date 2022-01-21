//! `NcPlot[F|U]64` widget.

use crate::c_api::ffi;

/// A histogram, bound to an [`NcPlane`][crate::NcPlane]
/// (uses non-negative `f64`s)
pub type NcPlotF64 = ffi::ncdplot;

/// A histogram, bound to an [`NcPlane`][crate::NcPlane] (uses `u64`s)
pub type NcPlotU64 = ffi::ncuplot;

/// Options struct for
/// [`NcPlotF64`] or [`NcPlotU64`]
pub type NcPlotOptions = ffi::ncplot_options;

impl NcPlotOptions {
    /// Use domain detection only for max
    pub const DETECTMAXONLY: u32 = c_api::NCPLOT_OPTION_DETECTMAXONLY;

    /// Exponential dependent axis
    pub const EXPONENTIALD: u32 = c_api::NCPLOT_OPTION_EXPONENTIALD;

    /// Show labels for dependent axis
    pub const LABELTICKSD: u32 = c_api::NCPLOT_OPTION_LABELTICKSD;

    /// Use domain detection only for max
    pub const NODEGRADE: u32 = c_api::NCPLOT_OPTION_NODEGRADE;

    /// Independent axis is vertical
    pub const VERTICALI: u32 = c_api::NCPLOT_OPTION_VERTICALI;
}

pub(crate) mod c_api {
    use super::ffi;

    /// Use domain detection only for max
    pub const NCPLOT_OPTION_DETECTMAXONLY: u32 = ffi::NCPLOT_OPTION_DETECTMAXONLY;

    /// Exponential dependent axis
    pub const NCPLOT_OPTION_EXPONENTIALD: u32 = ffi::NCPLOT_OPTION_EXPONENTIALD;

    /// Show labels for dependent axis
    pub const NCPLOT_OPTION_LABELTICKSD: u32 = ffi::NCPLOT_OPTION_LABELTICKSD;

    /// Use domain detection only for max
    pub const NCPLOT_OPTION_NODEGRADE: u32 = ffi::NCPLOT_OPTION_NODEGRADE;

    /// Independent axis is vertical
    pub const NCPLOT_OPTION_VERTICALI: u32 = ffi::NCPLOT_OPTION_VERTICALI;
}
