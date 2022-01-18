//! Macros
//!
//
// NOTE: Use full paths everywhere. Don't assume anything will be in scope.

#[allow(unused_imports)] // for doc comments
use crate::{
    c_api::{NCRESULT_ERR, NCRESULT_OK},
    Nc, NcDirect, NcError, NcPlane, NcResult, NcVisual, NcVisualOptions,
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

/// [`Nc::render`][Nc#method.render]\(`$nc`\)? + [`sleep!`]`[$sleep_args]`.
///
/// Renders the `$nc` [`Nc`]'s standard pile, and then,
/// if there's no error, calls the `sleep` macro with the rest of the arguments.
///
/// Returns [`NcResult`].
#[macro_export]
macro_rules! nc_render_sleep {
    ($nc:expr, $( $sleep_args:expr),+ ) => {
        crate::Nc::render($nc)?;
        sleep![$( $sleep_args ),+];
    };
    ($nc:expr, $( $sleep_args:expr),+ ,) => {
        rsleep![$nc, $( $sleep_args ),* ]
    };
}

/// [`NcPlane::render`][NcPlane#method.render]\(`$p`\)? +
/// [`NcPlane::rasterize`][NcPlane#method.rasterize]\(`$p`\)? +
/// [`sleep!`]`[$sleep_args]`.
///
/// Renders and rasterizes the `$p` [`NcPlane`] pile and then, if there are
/// no errors, calls the sleep macro with the rest of the arguments.
///
/// Returns [`NcResult`].
#[macro_export]
macro_rules! pile_render_sleep {
    ($p:expr, $( $sleep_args:expr),+ ) => {
        crate::NcPlane::render($p)?;
        crate::NcPlane::rasterize($p)?;
        sleep![$( $sleep_args ),+];
    };
    ($nc:expr, $( $sleep_args:expr),+ ,) => {
        rsleep![$nc, $( $sleep_args ),* ]
    };
}

/// [`NcVisual::blit`][NcVisual#method.blit]\(`$v`, `$nc`, `$vo`\)? +
/// [`Nc::render`][Nc#method.render]\(`$nc`\)? + [`sleep!`]`[$sleep_args]`.
///
/// Renders and rasterizes the `$v` [`NcVisual`] with its `$vo`
/// [`NcVisualOptions`], the $nc` [`Nc`]'s standard pile, and then, if there are
/// no errors, calls the `sleep` macro with the rest of the arguments.
///
/// Returns [`NcResult`].
#[macro_export]
macro_rules! visual_render_sleep {
    ($v: expr, $vo: expr, $nc:expr, $( $sleep_args:expr),+ ) => {
        unsafe { crate::NcVisual::blit($v, $nc, Some($vo))? };
        crate::Nc::render($nc)?;
        sleep![$( $sleep_args ),+];
    };
    ($nc:expr, $( $sleep_args:expr),+ ,) => {
        rsleep![$nc, $( $sleep_args ),* ]
    };
}

// String & Print Macros -------------------------------------------------------

/// Converts an `&str` into `*const c_char`.
///
/// See [`Cstring`].
#[macro_export]
#[doc(hidden)]
macro_rules! cstring {
    ($s:expr) => {
        // NOTE: The returned pointer will be valid for as long as self is
        // CHECK does this live long enough in all circumstances?
        std::ffi::CString::new($s).unwrap().as_ptr()
    };
}

/// Converts an `&str` into `*mut c_char`.
///
/// See [`Cstring`].
#[macro_export]
#[doc(hidden)]
macro_rules! cstring_mut {
    ($s:expr) => {
        // we can't use this without taking the responsibility of deallocating:
        std::ffi::CString::new($s).unwrap().into_raw()

        // another option:
        // unsafe { crate::c_api::libc::strdup(crate::cstring![$s]) }
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
        unsafe { crate::c_api::libc::free(nc_string as *mut core::ffi::c_void) };
        string
    }};
}

/// Wrapper around [`libc::printf`][c_api::libc::printf].
#[macro_export]
#[doc(hidden)]
macro_rules! printf {
    ($s:expr) => {
        unsafe { crate::c_api::libc::printf(cstring![$s]) }
    };
    ($s:expr $(, $opt:expr)*) => {
        unsafe { crate::c_api::libc::printf(cstring![$s], $($opt),*) }
    };
}

/// Wrapper around [`NcPlane.putstr`][NcPlane#method.putstr],
/// rendering and rasterizing the plane afterwards.
///
/// Returns an `NcResult` with the number of columns advanced,
/// without including newlines.
///
/// # Example
/// ```ignore
/// # use libnotcurses_sys::*;
/// # fn main() -> NcResult<()> {
/// let nc = unsafe { Nc::new_cli()? };
/// let splane = unsafe { nc.stdplane() };
/// splane.set_scrolling(true);
/// putstr!(splane, "hello ")?;
/// putstr!(splane, " world\n")?;
/// putstr!(splane, "formatted text: {:?}\n", (0, 1.0, "two") )?;
/// # unsafe { nc.stop()? };
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
/// ```ignore
/// # use libnotcurses_sys::*;
/// # fn main() -> NcResult<()> {
/// let nc = unsafe { Nc::new_cli()? };
/// let splane = nc.stdplane();
/// splane.set_scrolling(true);
/// putstrln!(splane, "hello world")?;
/// putstrln!(splane, "formatted text: {:?}", (0, 1.0, "two") )?;
/// # unsafe { nc.stop()? };
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

/// Implements multiple variants of `From` between a primitive
/// and a unit struct containing that primitive.
#[macro_export]
#[doc(hidden)]
macro_rules! unit_impl_from [
    ($struct:ty, $prim:ty ) => {
        // from prim
        impl From<$prim> for $struct {
            fn from(p: $prim) -> Self { <$struct>::from_primitive(p) }
        }
        impl<'a> From<&'a $prim> for $struct {
            fn from(p: &'a $prim) -> Self { <$struct>::from_primitive(*p) }
        }
        impl<'a> From<&'a mut $prim> for $struct {
            fn from(p: &'a mut $prim) -> Self { <$struct>::from_primitive(*p) }
        }

        // from struct
        impl From<$struct> for $prim {
            fn from(s: $struct) -> Self { s.0 }
        }
        impl<'a> From<&'a $struct> for &'a $prim {
            fn from(s: &'a $struct) -> Self { &s.0 }
        }
        impl<'a> From<&'a mut $struct> for &'a mut $prim {
            fn from(s: &'a mut $struct) -> Self { &mut s.0 }
        }
        impl From<&$struct> for *const $prim {
            fn from(s: &$struct) -> Self { &s.0 as *const $prim}
        }
        impl From<&mut $struct> for *mut $prim {
            fn from(s: &mut $struct) -> Self { &mut s.0 as *mut $prim}
        }
    };
];

/// Implements overloadable operators for a unit struct that contains a primitive.
#[macro_export]
#[doc(hidden)]
macro_rules! unit_impl_ops [
    // implements the bitwise operators.
    (bitwise; $type:ty) => {
        crate::unit_impl_ops![op_refs; BitAnd, bitand, $type];
        crate::unit_impl_ops![op_refs; BitOr, bitor, $type];
        crate::unit_impl_ops![op_refs; BitXor, bitxor, $type];
        crate::unit_impl_ops![op_refs; Shl, shl, $type];
        crate::unit_impl_ops![op_refs; Shr, shr, $type];
        crate::unit_impl_ops![op_refs_a; BitAndAssign, bitand_assign, $type];
        crate::unit_impl_ops![op_refs_a; BitOrAssign, bitor_assign, $type];
        crate::unit_impl_ops![op_refs_a; BitXorAssign, bitxor_assign, $type];
        crate::unit_impl_ops![op_refs_a; ShlAssign, shl_assign, $type];
        crate::unit_impl_ops![op_refs_a; ShrAssign, shr_assign, $type];
        crate::unit_impl_ops![op_refs_u; Not, not, $type];
    };

    // implements the arithmetic operators (except neg).
    (arithmetic; $type:ty) => {
        crate::unit_impl_ops![op_refs; Add, add, $type];
        crate::unit_impl_ops![op_refs; Sub, sub, $type];
        crate::unit_impl_ops![op_refs; Mul, mul, $type];
        crate::unit_impl_ops![op_refs; Div, div, $type];
        crate::unit_impl_ops![op_refs; Rem, rem, $type];
        crate::unit_impl_ops![op_refs; Rem, rem, $type];
        crate::unit_impl_ops![op_refs_a; AndAssign, and_assign, $type];
        crate::unit_impl_ops![op_refs_a; OrAssign, or_assign, $type];
        crate::unit_impl_ops![op_refs_a; XorAssign, xor_assign, $type];
        crate::unit_impl_ops![op_refs_a; ShlAssign, shl_assign, $type];
        crate::unit_impl_ops![op_refs_a; ShrAssign, shr_assign, $type];
    };

    // ----------

    // implements all the variants of a single operator. (non-Assign version)
    (op_refs; $op:tt, $fn:ident, $type:ty) => {
        crate::unit_impl_ops![op; $type, $op, $fn, $type, $type];
        crate::unit_impl_ops![op; $type, $op, $fn, $type, &'b $type];
        crate::unit_impl_ops![op; $type, $op, $fn, $type, &'b mut $type];
        crate::unit_impl_ops![op; $type, $op, $fn, &'a $type, $type];
        crate::unit_impl_ops![op; $type, $op, $fn, &'a $type, &'b $type];
        crate::unit_impl_ops![op; $type, $op, $fn, &'a $type, &'b mut $type];
        crate::unit_impl_ops![op; $type, $op, $fn, &'a mut $type, $type];
        crate::unit_impl_ops![op; $type, $op, $fn, &'a mut $type, &'b $type];
        crate::unit_impl_ops![op; $type, $op, $fn, &'a mut $type, &'b mut $type];
    };

    // implements all the variants of a single operator. (Assign version)
    (op_refs_a; $op:tt, $fn:ident, $type:ty) => {
        crate::unit_impl_ops![op_a; $op, $fn, $type, $type];
        crate::unit_impl_ops![op_a; $op, $fn, $type, &'b $type];
        crate::unit_impl_ops![op_a; $op, $fn, $type, &'b mut $type];
        crate::unit_impl_ops![op_a; $op, $fn, &'a mut $type, $type];
        crate::unit_impl_ops![op_a; $op, $fn, &'a mut $type, &'b $type];
        crate::unit_impl_ops![op_a; $op, $fn, &'a mut $type, &'b mut $type];
    };

    // implements all the variants of a single operator. (Unary version)
    (op_refs_u; $op:tt, $fn:ident, $type:ty) => {
        crate::unit_impl_ops![op_u; $type, $op, $fn, $type];
        crate::unit_impl_ops![op_u; $type, $op, $fn, &'a $type];
        crate::unit_impl_ops![op_u; $type, $op, $fn, &'a mut $type];
    };

    // ----------

    // implements a single operator. (non-Assign version)
    //
    // # Arguments
    //
    // - $type:  the main type for the implementation, must be owned.
    // - $op:    the operator trait
    // - $fn:    the operator function
    // - $for:   the main type for the implementation, can be a reference.
    // - $rhs:   the right hand side of the operation, can be a reference.
    //
    (op; $type:ty, $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            type Output = $type;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                <$type>::from_primitive(self.0.$fn(rhs.0))
            }
        }
    };

    // implements a single operator. (Assign version)
    (op_a; $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            fn $fn(&mut self, rhs: $rhs) {
                self.0.$fn(rhs.0)
            }
        }
    };

    // implements a single operator. (Unary version)
    (op_u; $type:ty, $op:tt, $fn: ident, $for:ty) => {
        impl<'a> core::ops::$op for $for {
            type Output = $type;
            fn $fn(self) -> Self::Output {
                <$type>::from_primitive(self.0.$fn())
            }
        }
    };
];

/// Implements a constructor for unit structs from its inner value type,
/// intended to be called from the `unit_impl_*` macros.
#[macro_export]
#[doc(hidden)]
macro_rules! from_primitive [
    ($inner:ty) => {
        pub(crate) fn from_primitive(value: $inner) -> Self {
            Self(value)
        }
    }
];
