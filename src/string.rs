//! `NcString`

use crate::c_api::libc::{free, strdup};
use cty::c_char;
use std::ffi::{c_void, CString};

/// This type manages a string allocated for methods that accept `*mut c_char`,
/// and takes care of deallocating it at the end of the scope.
///
pub struct NcString {
    ptr: *mut c_char,
}
impl NcString {
    ///
    pub fn new(string: &str) -> Self {
        let cstring = CString::new(string).expect("CString::new");
        let ptr = unsafe { strdup(cstring.as_ptr()) };
        Self { ptr }
    }

    ///
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.ptr
    }
}
impl Drop for NcString {
    fn drop(&mut self) {
        unsafe { free(self.ptr as *mut c_void) };
    }
}
