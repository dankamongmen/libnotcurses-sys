//! `NcString`
// WIP

// use crate::c_api::libc::{free, strdup};

use core::ffi::c_char;

#[cfg(not(feature = "std"))]
use alloc::ffi::CString;

#[cfg(feature = "std")]
use std::ffi::CString;

/// A wrapped [`CString`] accepted by widgets.
///
// MAYBE: also for exporting to `*mut c_char`?
#[derive(Debug)]
pub struct NcString {
    cstring: CString,
    // ptr: *mut c_char,
    // deallocate: bool,
}
impl NcString {
    ///
    pub fn new(string: &str) -> Self {
        // let cstring = CString::new(string).expect("CString::new");
        // let ptr = unsafe { strdup(cstring.as_ptr()) };
        // Self { ptr, deallocate: true }
        Self { cstring: CString::new(string).expect("CString::new") }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.cstring.as_ptr()
    }

    // MAYBE: fn as_raw(self) ? (transfers ownership)

    // /// Choose whether to dellocate the string on drop or not.
    // pub fn deallocate(&mut self, deallocate: bool) {
    //     self.deallocate = deallocate;
    // }

    //
    // pub fn as_mut_ptr(&mut self) -> *mut c_char {
    //     self.cstring.as_mut_ptr()
    // }
}

// impl Drop for NcString {
//     fn drop(&mut self) {
//         if self.deallocate {
//             unsafe { free(self.ptr as *mut c_void) };
//         }
//     }
// }
