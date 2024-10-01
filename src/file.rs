//! Wrapper for `libc::FILE`, both as used by notcurses and the libc crate
//!
//! The interface is largely based on the implementation of the
//! [cfile-rs crate](https://github.com/jkarns275/cfile) by Joshua Karns

use core::ptr::{null_mut, NonNull};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

use libc::{
    c_long, c_void, fclose, /* feof, */ fread, fseek, ftell, SEEK_CUR, SEEK_END, SEEK_SET,
};

// A utility function to pull the current value of errno and put it into an
// Error::Errno
fn get_error<T>() -> Result<T, Error> {
    Err(Error::last_os_error())
}

// See [`NcFile`]. Notcurses functions expects this type of `*FILE` (a struct)
#[allow(clippy::upper_case_acronyms)]
type NcFile_nc = crate::c_api::ffi::FILE;

// See [`NcFile`]. The [`libc`](https://docs.rs/libc/) crate expects this type
// of `*FILE` (an opaque enum)
#[allow(clippy::upper_case_acronyms)]
type NcFile_libc = libc::FILE;

/// A wrapper struct around
/// [`libc::FILE`](https://docs.rs/libc/0.2.80/libc/enum.FILE.html)
///
/// The `notcurses`' [`FILE`][crate::c_api::ffi::FILE] type is a transparent
/// struct, while the equivalent `libc`'s [`FILE`][libc::FILE] is an opaque enum.
///
/// Several methods are provided to cast back and forth between both types,
/// in order to allow both rust libc operations and notcurses file operations
/// over the same underlying `*FILE`.
#[derive(Debug)]
pub struct NcFile {
    file_ptr: NonNull<NcFile_libc>,
}

/// # Constants
impl NcFile {
    // TODO: the following static strings aren't made public

    /// Intended to be passed into the CFile::open method.
    /// It will open the file in a way that will allow reading and writing,
    /// including overwriting old data.
    /// It will not create the file if it does not exist.
    pub const RANDOM_ACCESS_MODE: &'static str = "rb+";

    /// Intended to be passed into the CFile::open method.
    /// It will open the file in a way that will allow reading and writing,
    /// including overwriting old data
    pub const UPDATE: &'static str = "rb+";

    /// Intended to be passed into the CFile::open method.
    /// It will only allow reading.
    pub const READ_ONLY: &'static str = "r";

    /// Intended to be passed into the CFile::open method.
    /// It will only allow writing.
    pub const WRITE_ONLY: &'static str = "w";

    /// Intended to be passed into the CFile::open method.
    /// It will only allow data to be appended to the end of the file.
    pub const APPEND_ONLY: &'static str = "a";

    /// Intended to be passed into the CFile::open method.
    /// It will allow data to be appended to the end of the file, and data to be
    /// read from the file. It will create the file if it doesn't exist.
    pub const APPEND_READ: &'static str = "a+";

    /// Intended to be passed into the CFile::open method.
    /// It will open the file in a way that will allow reading and writing,
    /// including overwriting old data. It will create the file if it doesn't exist
    pub const TRUNCATE_RANDOM_ACCESS_MODE: &'static str = "wb+";
}

/// # Constructors
impl NcFile {
    /// `NcFile` constructor from a file produced by notcurses.
    pub unsafe fn from_nc(file: *mut NcFile_nc) -> Self {
        NcFile { file_ptr: unsafe { NonNull::new_unchecked(NcFile::nc2libc(file)) } }
    }

    /// `NcFile` constructor from a file produced by the libc crate.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_libc(file: *mut NcFile_libc) -> Self {
        NcFile { file_ptr: NonNull::new_unchecked(file) }
    }
}

/// # Methods
impl NcFile {
    /// Returns the file pointer in the format expected by the [`libc`] crate.
    #[inline]
    pub fn as_libc_ptr(&self) -> *mut NcFile_libc {
        self.file_ptr.as_ptr()
    }

    /// Returns the file pointer in the format expected by notcurses.
    #[inline]
    pub fn as_nc_ptr(&self) -> *mut NcFile_nc {
        Self::libc2nc(self.file_ptr.as_ptr())
    }

    /// Returns the current position in the file.
    ///
    /// On error `Error::Errno(errno)` is returned.
    pub fn current_pos(&self) -> Result<u64, Error> {
        unsafe {
            let pos = ftell(self.as_libc_ptr());
            if pos != -1 {
                Ok(pos as u64)
            } else {
                get_error()
            }
        }
    }

    /// Reads the file from start to end. Convenience method.
    ///
    #[inline]
    pub fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let _ = self.seek(SeekFrom::Start(0));
        self.read_to_end(buf)
    }

    // private methods:

    /// Converts a file pointer from the struct notcurses uses to the
    /// opaque enum type libc expects.
    #[inline]
    fn nc2libc(file: *mut NcFile_nc) -> *mut NcFile_libc {
        file as *mut _ as *mut NcFile_libc
    }

    /// Converts a file pointer from the libc opaque enum format to the struct
    /// expected by notcurses.
    #[inline]
    fn libc2nc(file: *mut NcFile_libc) -> *mut NcFile_nc {
        file as *mut _ as *mut NcFile_nc
    }

    /// A utility function to expand a vector without increasing its capacity
    /// more than it needs to be expanded.
    fn expand_buffer(buff: &mut Vec<u8>, by: usize) {
        if buff.capacity() < buff.len() + by {
            buff.reserve(by);
        }
        for _ in 0..by {
            buff.push(0u8);
        }
    }
}

impl Read for NcFile {
    /// Reads exactly the number of bytes required to fill buf.
    ///
    /// If the end of the file is reached before buf is filled,
    /// `Err(EndOfFile(bytes_read))` will be returned.  The data that was read
    /// before that will still have been placed into buf.
    ///
    /// Upon some other error, `Err(Errno(errno))` will be returned.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        unsafe {
            let result = fread(
                buf.as_ptr() as *mut c_void,
                1,
                buf.len(),
                self.as_libc_ptr(),
            );
            if result != buf.len() {
                match get_error::<u8>() {
                    Err(err) => {
                        if err.kind() == ErrorKind::UnexpectedEof {
                            Ok(result)
                        } else {
                            Err(err)
                        }
                    }
                    Ok(_) => panic!("This is impossible"),
                }
            } else {
                Ok(result)
            }
        }
    }

    /// Reads the entire file starting from the current_position
    /// expanding buf as needed.
    ///
    /// On a successful read, this function will return `Ok(bytes_read)`.
    ///
    /// If an error occurs during reading, some varient of error will be returned.
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let pos = self.current_pos();
        let _ = self.seek(SeekFrom::End(0));
        let end = self.current_pos();
        match pos {
            Ok(cur_pos) => match end {
                Ok(end_pos) => {
                    if end_pos == cur_pos {
                        return Ok(0);
                    }
                    let to_read = (end_pos - cur_pos) as usize;
                    if buf.len() < to_read {
                        let to_reserve = to_read - buf.len();
                        Self::expand_buffer(buf, to_reserve);
                    }
                    let _ = self.seek(SeekFrom::Start(cur_pos));
                    match self.read_exact(buf) {
                        Ok(()) => Ok(to_read),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Reads the entire file from the beginning and stores it in a string.
    ///
    /// On a successful read, this function will return `Ok(bytes_read)`.
    ///
    /// If an error occurs during reading, some varient of error will be returned.
    fn read_to_string(&mut self, strbuf: &mut String) -> Result<usize, Error> {
        let mut buffer = vec![0u8];

        let bytes_read = self.read_all(&mut buffer)?;

        let result = std::str::from_utf8(&buffer);

        if let Ok(strslice) = result {
            *strbuf = strslice.to_string();
            Ok(bytes_read)
        } else {
            get_error()
        }
    }

    /// Reads exactly the number of bytes required to fill buf.
    ///
    /// If the end of the file is reached before buf is filled,
    /// `Err(EndOfFile(bytes_read))` will be returned. The data that was read
    /// before that will still have been placed into buf.
    ///
    /// Upon some other error, `Err(Errno(errno))` will be returned.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        unsafe {
            let result = fread(
                buf.as_ptr() as *mut c_void,
                1,
                buf.len(),
                self.as_libc_ptr(),
            );
            if result == buf.len() {
                Ok(())
            } else {
                // Check if we hit the end of the file
                // FIXME
                // if feof(self.as_libc_ptr()) != 0 {
                //     get_error()
                // } else {
                //     get_error()
                // }
                get_error()
            }
        }
    }
}

impl Seek for NcFile {
    /// Changes the current position in the file using the [`SeekFrom`] enum.
    ///
    /// To set relative to the beginning of the file (i.e. index is 0 + offset):
    /// ```ignore
    /// SeekFrom::Start(offset)
    /// ```
    /// To set relative to the end of the file (i.e. index is file_lenth - 1 - offset):
    /// ```ignore
    /// SeekFrom::End(offset)
    /// ```
    /// To set relative to the current position:
    /// ```ignore
    /// SeekFrom::End(offset)
    /// ```
    ///
    /// On error `Error::Errno(errno)` is returned.
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        unsafe {
            let result = match pos {
                SeekFrom::Start(from) => fseek(self.as_libc_ptr(), from as c_long, SEEK_SET),
                SeekFrom::End(from) => fseek(self.as_libc_ptr(), from as c_long, SEEK_END),
                SeekFrom::Current(delta) => fseek(self.as_libc_ptr(), delta as c_long, SEEK_CUR),
            };
            if result == 0 {
                self.current_pos()
            } else {
                get_error()
            }
        }
    }
}

impl Drop for NcFile {
    /// Ensures the file stream is closed before abandoning the data.
    fn drop(&mut self) {
        let _ = unsafe {
            if !(self.as_libc_ptr()).is_null() {
                let res = fclose(self.as_libc_ptr());
                if res == 0 {
                    self.file_ptr = NonNull::new_unchecked(null_mut::<NcFile_libc>());
                    Ok(())
                } else {
                    get_error()
                }
            } else {
                Ok(())
            }
        };
    }
}
