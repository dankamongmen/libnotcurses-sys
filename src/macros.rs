//! Macros
//!
//
// NOTE: Use full paths everywhere. Don't assume anything will be in scope.

#[allow(unused_imports)]
// enjoy briefer doc comments
use crate::{
    c_api::{NCRESULT_ERR, NCRESULT_OK},
    Nc, NcDirect, NcError, NcIntResultApi, NcPlane, NcResult,
};

// Sleep, Render & Flush Macros ------------------------------------------------

/// Sleeps for `$s` seconds + `$ms` milliseconds
/// + `$us` microseconds + `$ns` nanoseconds
#[macro_export]
macro_rules! sleep {
    ($s:expr) => {
        std::thread::sleep(std::time::Duration::from_secs($s));
    };

    ($s:expr, $ms:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($s * 1000 + $ms));
    };
    ($s:expr, $ms:expr, $us:expr) => {
        std::thread::sleep(std::time::Duration::from_micros(
            $s * 1_000_000 + $ms * 1_000 + $us,
        ));
    };
    ($s:expr, $ms:expr, $us:expr, $ns:expr) => {
        std::thread::sleep(std::time::Duration::from_nanos(
            $s * 1_000_000_000 + $ms * 1_000_000 + $us * 1_000 + $ns,
        ));
    };
}

/// notcurses render sleep:
/// [`Nc::render`][Nc#method.render]\(`$nc`\)? + [`sleep!`]`[$sleep_args]`.
///
/// Renders the `$nc` [`Nc`] object's standard plane pile and then,
/// if there's no error, calls the sleep macro with the rest of the arguments.
///
/// Returns [NcResult].
#[macro_export]
macro_rules! nrs {
    ($nc:expr, $( $sleep_args:expr),+ ) => {
        crate::Nc::render($nc)?;
        sleep![$( $sleep_args ),+];
    };
    ($nc:expr, $( $sleep_args:expr),+ ,) => {
        rsleep![$nc, $( $sleep_args ),* ]
    };
}

/// plane render sleep:
/// [`NcPlane::render`][NcPlane#method.render]\(`$p`\)? +
/// [`NcPlane::rasterize`][NcPlane#method.rasterize]\(`$p`\)? +
/// [`sleep!`]`[$sleep_args]`.
///
/// Renders and rasterizes the `$p` [NcPlane] pile and then, if there are
/// no errors, calls the sleep macro with the rest of the arguments.
///
/// Returns [NcResult].
#[macro_export]
macro_rules! prs {
    ($p:expr, $( $sleep_args:expr),+ ) => {
        crate::NcPlane::render($p)?;
        crate::NcPlane::rasterize($p)?;
        sleep![$( $sleep_args ),+];
    };
    ($nc:expr, $( $sleep_args:expr),+ ,) => {
        rsleep![$nc, $( $sleep_args ),* ]
    };
}

/// [`NcDirect::flush`][NcDirect#method.flush]\(`$ncd`\)? + [`sleep!`]`[$sleep_args]`.
///
/// Flushes the `$ncd` [NcDirect] object and, if there's no error,
/// calls the sleep macro with the rest of the arguments.
///
/// Returns [NcResult].
#[macro_export]
#[deprecated]
#[doc(hidden)]
macro_rules! fsleep {
    ($ncd:expr, $( $sleep_args:expr),+ ) => {
        // Rust style, with methods & NcResult
        crate::NcDirect::flush($ncd)?;
        sleep![$( $sleep_args ),+];
    };
    ($ncd:expr, $( $sleep_args:expr),+ ,) => {
        rsleep![$ncd, $( $sleep_args ),* ]
    };
}

#[deprecated]
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! prsleep {
    ($p:expr, $( $sleep_args:expr),+ ) => {
        prs![$p, $( $sleep_args ),+];
    };
}

#[deprecated]
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! psleep {
    ($p:expr, $( $sleep_args:expr),+ ) => {
        prs![$p, $( $sleep_args ),+];
    };
}

#[deprecated]
#[doc(hidden)]
#[allow(unused_macros)]
macro_rules! rsleep {
    ($nc:expr, $( $sleep_args:expr),+ ) => {
        nrs![$nc, $( $sleep_args ),+];
    };
}

// String & Print Macros -------------------------------------------------------

/// Converts an `&str` into `*const c_char`.
#[macro_export]
#[doc(hidden)]
macro_rules! cstring {
    ($s:expr) => {
        std::ffi::CString::new($s).unwrap().as_ptr()
    };
}

/// Converts an `&str` into `*mut c_char`.
#[macro_export]
#[doc(hidden)]
macro_rules! cstring_mut {
    ($s:expr) => {
        std::ffi::CString::new($s).unwrap().into_raw()
    };
}

/// Converts a `*const c_char` into an `&str`.
#[macro_export]
#[doc(hidden)]
macro_rules! rstring {
    ($s:expr) => {
        unsafe { std::ffi::CStr::from_ptr($s).to_str().unwrap() }
        // possible alternative:
        // unsafe { std::ffi::CStr::from_ptr($s).to_string_lossy() }
    };
}

/// Converts a `*const c_char` into a `String`, freeing the original alloc.
#[macro_export]
#[doc(hidden)]
macro_rules! rstring_free {
    ($s:expr) => {{
        #[allow(unused_unsafe)]
        let nc_string = unsafe { $s };
        let string = crate::rstring![nc_string].to_string();
        unsafe { c_api::libc::free(nc_string as *mut core::ffi::c_void) };
        string
    }};
}

/// Wrapper around [`libc::printf`][c_api::libc::printf].
#[macro_export]
#[doc(hidden)]
macro_rules! printf {
    ($s:expr) => {
        unsafe { c_api::libc::printf(cstring![$s]) }
    };
    ($s:expr $(, $opt:expr)*) => {
        unsafe { c_api::libc::printf(cstring![$s], $($opt),*) }
    };
}

/// Wrapper around [`NcPlane.putstr`][NcPlane#method.putstr],
/// rendering and rasterizing the plane afterwards.
///
/// Returns an `NcResult` with the number of columns advanced,
/// without including newlines.
///
/// # Example
/// ```
/// # use libnotcurses_sys::*;
/// # fn main() -> NcResult<()> {
/// let nc = Nc::new_cli()?;
/// let splane = nc.stdplane();
/// splane.set_scrolling(true);
/// putstr!(splane, "hello ")?;
/// putstr!(splane, " world\n")?;
/// putstr!(splane, "formatted text: {:?}\n", (0, 1.0, "two") )?;
/// # nc.stop()?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! putstr {
    ($plane:ident, $text:literal) => {
        {
            let res = $plane.putstr($text)?;
            $plane.render()?;
            $plane.rasterize()?;
            Ok(res)
        }
    };
    ($plane:ident, $text:literal, $($args:tt)*) => {
        {
            let res = $plane.putstr(&format![$text, $($args)*])?;
            $plane.render()?;
            $plane.rasterize()?;
            Ok(res)
        }
    };
}

/// Wrapper around [`NcPlane.putstrln`][NcPlane#method.putstrln].
/// rendering and rasterizing the plane afterwards.
///
/// Returns an `NcResult` with the number of columns advanced,
/// without including newlines.
///
/// # Example
/// ```
/// # use libnotcurses_sys::*;
/// # fn main() -> NcResult<()> {
/// let nc = Nc::new_cli()?;
/// let splane = nc.stdplane();
/// splane.set_scrolling(true);
/// putstrln!(splane, "hello world")?;
/// putstrln!(splane, "formatted text: {:?}", (0, 1.0, "two") )?;
/// # nc.stop()?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! putstrln {
    ($plane:ident) => {
        {
            let res = $plane.putln()?;
            $plane.render()?;
            $plane.rasterize()?;
            Ok(res)
        }
    };
    ($plane:ident, $text:literal) => {
        {
            let res = $plane.putstrln($text)?;
            $plane.render()?;
            $plane.rasterize()?;
            Ok(res)
        }
    };
    ($plane:ident, $text:literal, $($args:tt)*) => {
        {
            let res = $plane.putstrln(&format![$text, $($args)*])?;
            $plane.render()?;
            $plane.rasterize()?;
            Ok(res)
        }
    };
}

// Error Wrappers Macros -------------------------------------------------------

/// Returns an `Ok($ok)`,
/// or an `Err(`[`NcError`]`)` if `$res` < [`NCRESULT_OK`].
///
/// In other words:
/// Returns Ok(`$ok`) if `$res` >= [NCRESULT_OK], otherwise returns
/// Err([NcError]::[new][NcError#method.new](`$res`, `$msg`)).
///
/// `$ok` & `$msg` are optional. By default they will be the unit
/// type `()`, and an empty `&str` `""`, respectively.
#[macro_export]
#[doc(hidden)]
macro_rules! error {
    ($res:expr, $msg:expr, $ok:expr) => {{
        let res = $res;
        if res >= crate::c_api::NCRESULT_OK {
            return Ok($ok);
        } else {
            return Err(crate::NcError::with_msg(res, $msg));
        }
    }};
    ($res:expr, $msg:expr) => {
        error![$res, $msg, ()]
    };
    ($res:expr) => {
        error![$res, "", ()]
    };
}

/// Returns an `Ok(&T)` from a `*const T` pointer,
/// or an `Err(`[`NcError`]`)` if the pointer is null.
///
/// In other words:
/// Returns Ok(&*`$ptr`) if `!$ptr.is_null()`, otherwise returns
/// Err([NcError]]::[new][NcError#method.new]([NCRESULT_ERR], `$msg`)).
///
/// `$msg` is optional. By default it will be an empty `&str` `""`.
#[macro_export]
#[doc(hidden)]
macro_rules! error_ref {
    ($ptr:expr, $msg:expr, $ok:expr) => {{
        let ptr = $ptr; // avoid calling a function multiple times
        if ptr.is_null() {
            return Err(crate::NcError::with_msg(crate::c_api::NCRESULT_ERR, $msg));
        } else {
            #[allow(unused_unsafe)]
            return Ok(unsafe { $ok });
        }
    }};
    ($ptr:expr, $msg:expr) => {{
        let ptr = $ptr;
        error_ref![$ptr, $msg, unsafe { &*ptr }];
    }};
    ($ptr:expr) => {{
        let ptr = $ptr;
        error_ref![$ptr, "", unsafe { &*ptr }];
    }};
}

/// Returns an `Ok(&mut T)` from a `*mut T` pointer,
/// or an `Err(`[`NcError`]`)` if the pointer is null.
///
/// In other words:
/// Returns Ok(&mut *`$ptr`) if `!$ptr._is_null()`, otherwise returns
/// Err([NcError]]::[new][NcError#method.new]([NCRESULT_ERR], `$msg`)).
///
/// `$msg` is optional. By default it will be an empty `&str` `""`.
#[macro_export]
#[doc(hidden)]
macro_rules! error_ref_mut {
    ($ptr:expr, $msg:expr, $ok:expr) => {{
        let ptr = $ptr; // avoid calling a function multiple times
        if ptr.is_null() {
            return Err(crate::NcError::with_msg(crate::c_api::NCRESULT_ERR, $msg));
        } else {
            #[allow(unused_unsafe)]
            return Ok(unsafe { $ok });
        }
    }};
    ($ptr:expr, $msg:expr) => {{
        let ptr = $ptr;
        error_ref_mut![ptr, $msg, unsafe { &mut *ptr }];
    }};
    ($ptr:expr) => {{
        let ptr = $ptr;
        error_ref_mut![ptr, "", unsafe { &mut *ptr }];
    }};
}

/// Returns an `Ok(String)` from a `*const` pointer to a C string,
/// or an `Err(`[`NcError`]`)` if the pointer is null.
///
/// In other words:
/// Returns Ok((&*`$str`).to_string()) if `!$str.is_null()`, otherwise returns
/// Err([NcError]]::[new][NcError#method.new]([NCRESULT_ERR], `$msg`)).
///
/// `$msg` is optional. By default it will be an empty `&str` `""`.
#[macro_export]
#[doc(hidden)]
macro_rules! error_str {
    ($str:expr, $msg:expr) => {
        if !$str.is_null() {
            #[allow(unused_unsafe)]
            return Ok(unsafe { crate::rstring!($str).to_string() });
        } else {
            return Err(crate::NcError::with_msg(crate::c_api::NCRESULT_ERR, $msg));
        }
    };
    ($str:expr) => {
        error_str![$str, ""];
    };
}

// Implementation Helper Macros ------------------------------------------------

/// Implements methods and constants for an existing type.
//
// Allows to have full doc-comments both in the trait definition
// and in the concrete implementation.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_api {
    ($type:ident, $trait:ident, $($i:item),*) => {
        #[doc = concat!("Enables the [`", stringify!($type), "`] associated methods and constants.")]
        pub trait $trait {
            $($i)*
        }

        impl $trait for $type {
            $($i)*
        }
    };
}
