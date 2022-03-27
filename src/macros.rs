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
        std::ffi::CString::new($s).unwrap()
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
        let cs = cstring![$s];
        unsafe { crate::c_api::libc::printf(cs.as_ptr()) }
    };
    ($s:expr $(, $opt:expr)*) => {
        let cs = cstring![$s];
        unsafe { crate::c_api::libc::printf(cs.as_ptr(), $($opt),*) }
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
/// ```
/// # use libnotcurses_sys::*;
/// # fn main() -> NcResult<()> {
/// let nc = unsafe { Nc::new_cli()? };
/// let splane = unsafe { nc.stdplane() };
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

/// Implements overloadable operators for a unit struct containing a primitive.
///
/// # Usage
///
/// - (bitwise; outer_type, inner_type)
/// - (arithmetic; outer_type, inner_type)
/// - (neg; outer_type)
///
/// # Note
///
/// - The output type will always be the unit struct.
/// - There are no implemented ops between unit struct and &mut primitiv.
#[macro_export]
#[doc(hidden)]
macro_rules! unit_impl_ops [
    // # external API: implements sets of operations
    // -------------------------------------------------------------------------

    // ## implements the bitwise operators.
    (bitwise; $outer:ty, $inner:ty) => {
        // (struct)
        crate::unit_impl_ops![op_refs_u; Not, not, $outer];
        // (struct OP struct)
        crate::unit_impl_ops![op_refs_ss; BitAnd, bitand, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; BitOr, bitor, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; BitXor, bitxor, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Shl, shl, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Shr, shr, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; BitAndAssign, bitand_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; BitOrAssign, bitor_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; BitXorAssign, bitxor_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; ShlAssign, shl_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; ShrAssign, shr_assign, $outer, $outer];
        // (struct OP primitive)
        crate::unit_impl_ops![op_refs_sp; BitAnd, bitand, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; BitOr, bitor, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; BitXor, bitxor, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Shl, shl, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Shr, shr, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; BitAndAssign, bitand_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; BitOrAssign, bitor_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; BitXorAssign, bitxor_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; ShlAssign, shl_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; ShrAssign, shr_assign, $outer, $inner];
        // (primitive OP struct)
        crate::unit_impl_ops![op_refs_ps; BitAnd, bitand, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; BitOr, bitor, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; BitXor, bitxor, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Shl, shl, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Shr, shr, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; BitAndAssign, bitand_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; BitOrAssign, bitor_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; BitXorAssign, bitxor_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; ShlAssign, shl_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; ShrAssign, shr_assign, $outer, $inner];
    };

    // ## implements the arithmetic operators, except Neg.
    (arithmetic; $outer:ty, $inner:ty) => {
        // (struct OP struct)
        crate::unit_impl_ops![op_refs_ss; Add, add, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Sub, sub, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Mul, mul, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Div, div, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Rem, rem, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss; Rem, rem, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; AndAssign, and_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; OrAssign, or_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; XorAssign, xor_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; ShlAssign, shl_assign, $outer, $outer];
        crate::unit_impl_ops![op_refs_ss_a; ShrAssign, shr_assign, $outer, $outer];
        // (struct OP primitive)
        crate::unit_impl_ops![op_refs_sp; Add, add, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Sub, sub, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Mul, mul, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Div, div, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Rem, rem, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp; Rem, rem, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; AndAssign, and_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; OrAssign, or_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; XorAssign, xor_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; ShlAssign, shl_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_sp_a; ShrAssign, shr_assign, $outer, $inner];
        // (primitive OP struct)
        crate::unit_impl_ops![op_refs_ps; Add, add, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Sub, sub, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Mul, mul, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Div, div, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Rem, rem, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps; Rem, rem, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; AndAssign, and_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; OrAssign, or_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; XorAssign, xor_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; ShlAssign, shl_assign, $outer, $inner];
        crate::unit_impl_ops![op_refs_ps_a; ShrAssign, shr_assign, $outer, $inner];
    };

    // # implements Neg.
    (neg; $type:ty) => {
        crate::unit_impl_ops![op_refs_u; Neg, neg, $type];
    };

    // # internal API: implements multiple variants of an operation
    // -------------------------------------------------------------------------

    // ## implements all the variants of a single `non-assign` operator
    //
    // (struct OP struct)
    (op_refs_ss; $op:tt, $fn:ident, $T1:ty, $T2:ty) => {
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, $T1, $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, $T1, &'b $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, $T1, &'b mut $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, &'a $T1, $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, &'a $T1, &'b $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, &'a $T1, &'b mut $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, &'a mut $T1, $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, &'a mut $T1, &'b $T2];
        crate::unit_impl_ops![op_ss; $T1, $op, $fn, &'a mut $T1, &'b mut $T2];
    };
    // (struct OP primitive)
    (op_refs_sp; $op:tt, $fn:ident, $T1:ty, $T2:ty) => {
        crate::unit_impl_ops![op_sp; $T1, $op, $fn, $T1, $T2];
        crate::unit_impl_ops![op_sp; $T1, $op, $fn, $T1, &'b $T2];
        crate::unit_impl_ops![op_sp; $T1, $op, $fn, &'a $T1, $T2];
        crate::unit_impl_ops![op_sp; $T1, $op, $fn, &'a $T1, &'b $T2];
        crate::unit_impl_ops![op_sp; $T1, $op, $fn, &'a mut $T1, $T2];
        crate::unit_impl_ops![op_sp; $T1, $op, $fn, &'a mut $T1, &'b $T2];
        // Note: no implementation for `&mut primitive`
        // crate::unit_impl_ops![op_sp; $T1, $op, $fn, $T1, &'b mut $T2];
        // crate::unit_impl_ops![op_sp; $T1, $op, $fn, &'a $T1, &'b mut $T2];
        // crate::unit_impl_ops![op_sp; $T1, $op, $fn, &'a mut $T1, &'b mut $T2];
    };
    // (primitive OP struct)
    (op_refs_ps; $op:tt, $fn:ident, $T1:ty, $T2:ty) => {
        crate::unit_impl_ops![op_ps; $T1, $op, $fn, $T2, $T1];
        crate::unit_impl_ops![op_ps; $T1, $op, $fn, $T2, &'b $T1];
        crate::unit_impl_ops![op_ps; $T1, $op, $fn, $T2, &'b mut $T1];
        crate::unit_impl_ops![op_ps; $T1, $op, $fn, &'a $T2, $T1];
        crate::unit_impl_ops![op_ps; $T1, $op, $fn, &'a $T2, &'b $T1];
        crate::unit_impl_ops![op_ps; $T1, $op, $fn, &'a $T2, &'b mut $T1];
        // Note: no implementation for `&mut primitive`
        // crate::unit_impl_ops![op_ps; $T1, $op, $fn, &'a mut $T2, $T1];
        // crate::unit_impl_ops![op_ps; $T1, $op, $fn, &'a mut $T2, &'a $T1];
        // crate::unit_impl_ops![op_ps; $T1, $op, $fn, &'a mut $T2, &'a mut $T1];
    };

    // ## implements all the variants of a single `assign` operator
    //
    // (struct OP struct)
    (op_refs_ss_a; $op:tt, $fn:ident, $T1:ty, $T2:ty) => {
        crate::unit_impl_ops![op_ss_a; $op, $fn, $T1, $T2];
        crate::unit_impl_ops![op_ss_a; $op, $fn, $T1, &'b $T2];
        crate::unit_impl_ops![op_ss_a; $op, $fn, $T1, &'b mut $T2];
        crate::unit_impl_ops![op_ss_a; $op, $fn, &'a mut $T1, $T2];
        crate::unit_impl_ops![op_ss_a; $op, $fn, &'a mut $T1, &'b $T2];
        crate::unit_impl_ops![op_ss_a; $op, $fn, &'a mut $T1, &'b mut $T2];
    };
    // (struct OP primitive)
    (op_refs_sp_a; $op:tt, $fn:ident, $T1:ty, $T2:ty) => {
        crate::unit_impl_ops![op_sp_a; $op, $fn, $T1, $T2];
        crate::unit_impl_ops![op_sp_a; $op, $fn, $T1, &'b $T2];
        crate::unit_impl_ops![op_sp_a; $op, $fn, &'a mut $T1, $T2];
        crate::unit_impl_ops![op_sp_a; $op, $fn, &'a mut $T1, &'b $T2];
        // Note: no implementation for `&mut primitive`
        // crate::unit_impl_ops![op_sp_a; $op, $fn, $T1, &'b mut $T2];
        // crate::unit_impl_ops![op_sp_a; $op, $fn, &'a mut $T1, &'b mut $T2];
    };
    // (struct OP primitive)
    (op_refs_ps_a; $op:tt, $fn:ident, $T1:ty, $T2:ty) => {
        crate::unit_impl_ops![op_ps_a; $op, $fn, $T2, $T1];
        crate::unit_impl_ops![op_ps_a; $op, $fn, $T2, &'b $T1];
        crate::unit_impl_ops![op_ps_a; $op, $fn, $T2, &'b mut $T1];
        // Note: no implementation for `&mut primitive`
        // crate::unit_impl_ops![op_ps_a; $op, $fn, &'a mut $T2, $T1];
        // crate::unit_impl_ops![op_ps_a; $op, $fn, &'a mut $T2, &'b $T1];
        // crate::unit_impl_ops![op_ps_a; $op, $fn, &'a mut $T2, &'b mut $T1];
    };

    // ## implements all the variants of a single `unary` operator.
    (op_refs_u; $op:tt, $fn:ident, $T1:ty) => {
        crate::unit_impl_ops![op_u; $T1, $op, $fn, $T1];
        crate::unit_impl_ops![op_u; $T1, $op, $fn, &'a $T1];
        crate::unit_impl_ops![op_u; $T1, $op, $fn, &'a mut $T1];
    };

    // # internal API: implements a single variant of an operator
    // -------------------------------------------------------------------------

    // ## implements a single `non-assign` operator
    //
    // (struct OP struct)
    //
    // ### Arguments
    //
    // - $type:  the return type for the implementation.
    // - $op:    the operator trait
    // - $fn:    the operator function
    // - $for:   the main type for the implementation, can be a reference.
    // - $rhs:   the right hand side of the operation, can be a reference.
    //
    (op_ss; $type:ty, $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            type Output = $type;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                <$type>::from_primitive(self.0.$fn(rhs.0))
            }
        }
    };
    // (struct OP primitive)
    (op_sp; $type:ty, $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            type Output = $type;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                <$type>::from_primitive(self.0.$fn(rhs))
            }
        }
    };
    // (primitive OP struct)
    (op_ps; $type:ty, $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            type Output = $type;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                <$type>::from_primitive(self.$fn(rhs.0))
            }
        }
    };

    // ## implements a single `assign` operator
    //
    // (struct OP primitive)
    (op_ss_a; $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            fn $fn(&mut self, rhs: $rhs) {
                self.0.$fn(rhs.0)
            }
        }
    };
    // (struct OP primitive)
    (op_sp_a; $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            fn $fn(&mut self, rhs: $rhs) {
                self.0.$fn(rhs)
            }
        }
    };
    // (primitive OP struct)
    (op_ps_a; $op:tt, $fn: ident, $for:ty, $rhs:ty) => {
        impl<'a, 'b> core::ops::$op<$rhs> for $for {
            fn $fn(&mut self, rhs: $rhs) {
                self.$fn(rhs.0)
            }
        }
    };

    // ## implements a single `unary` operator.
    (op_u; $type:ty, $op:tt, $fn: ident, $for:ty) => {
        impl<'a> core::ops::$op for $for {
            type Output = $type;
            fn $fn(self) -> Self::Output {
                <$type>::from_primitive(self.0.$fn())
            }
        }
    };
];

/// Implements formatting traits for a unit struct containing a primitive.
#[macro_export]
#[doc(hidden)]
macro_rules! unit_impl_fmt [
    (all; $type:ty) => {
        crate::unit_impl_fmt![bases+display; $type];
        crate::unit_impl_fmt![scientific; $type];
    };

    (bases+display; $type:ty) => {
        crate::unit_impl_fmt![bases; $type];
        crate::unit_impl_fmt![display; $type];
    };

    (bases; $type:ty) => {
        crate::unit_impl_fmt![single; Binary, $type];
        crate::unit_impl_fmt![single; Octal, $type];
        crate::unit_impl_fmt![single; LowerHex, $type];
        crate::unit_impl_fmt![single; UpperHex, $type];
    };

    (scientific; $type:ty) => {
        crate::unit_impl_fmt![single; UpperExp, $type];
        crate::unit_impl_fmt![single; LowerExp, $type];
    };

    (display; $type:ty) => {
        crate::unit_impl_fmt![single; Display, $type];
    };

    (single; $trait:ident, $type:ty) => {
        impl std::fmt::$trait for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let val = self.0;
                std::fmt::$trait::fmt(&val, f)
            }
        }
    };
];

/// Implements a constructor for unit structs from its inner value type,
/// intended to be called from the `unit_impl_*` macros.
#[macro_export]
#[doc(hidden)]
macro_rules! from_primitive [
    ($outer:ty, $inner:ty) => {
        impl $outer {
            pub(crate) fn from_primitive(value: $inner) -> Self {
                Self(value)
            }
        }
    }
];
