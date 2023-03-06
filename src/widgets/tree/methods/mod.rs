//! `NcTree*` methods and associated functions.

mod options;
mod tree;

pub use options::*;
pub use tree::*;

#[cfg(not(feature = "std"))]
use alloc::ffi::CString;

#[cfg(feature = "std")]
use std::ffi::CString;

use core::{ffi::c_void, ptr::null_mut};

use super::NcTreeItem;

/// # `NcTreeItem` constructor
impl NcTreeItem {
    /// Creates an [`NcTreeItem`].
    pub fn new(curry: &str, subs: Option<&mut [NcTreeItem]>, subcount: usize) -> Self {
        if let Some(subs) = subs {
            Self {
                curry: CString::new(curry).unwrap().into_raw() as *mut _ as *mut c_void,
                subs: subs.as_mut_ptr(),
                subcount: subcount as u32,
            }
        } else {
            Self {
                curry: CString::new(curry).unwrap().into_raw() as *mut _ as *mut c_void,
                subs: null_mut(),
                subcount: subcount as u32,
            }
        }
    }
}
