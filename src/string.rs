//! `NcString`

// use crate::c_api::libc::{free, strdup};
use cty::c_char;
use std::ffi::CString;
// use std::ffi::{c_void, CString};

/// This type manages a string that t
///
// WIP
// methods that accept `*mut c_char`? take care of deallocating it
//
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
        Self {
            cstring: CString::new(string).expect("CString::new"),
        }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.cstring.as_ptr()
    }

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
