//! `NcMultiSelector` widget.

use crate::c_api::ffi;

/// high-level widget for selecting items from a set
pub type NcMultiSelector = ffi::ncmultiselector;

/// an item for [`NcMultiSelector`]
pub type NcMultiSelectorItem = ffi::ncmselector_item;

/// Options structure for [`NcMultiSelector`]
pub type NcMultiSelectorOptions = ffi::ncmultiselector_options;
