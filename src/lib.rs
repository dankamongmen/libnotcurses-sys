//! `libnotcurses-sys` is a low-level Rust wrapper for the
//! [notcurses C library](https://www.github.com/dankamongmen/notcurses/)
//!
//! It is built with several layers of zero-overhead abstractions
//! over the C functions and pointers accessed through FFI.
//!
//! It adds greater safety and type correctness over the underlying C library
//! API, while trying to remain very close to it.
//!
//! It offers the choice of using it [**more like Rust**](#like-rust)
//! and/or [**more like C**](#like-C).
//!
//! ## Like Rust
//!
//! Where you use the more safely wrapped types, with its methods and
//! constructors, and error handling with the `NcResult` enum:
//!
//! ### Example
//!
//! ```ignore
//! use libnotcurses_sys::*;
//!
//! # #[cfg(not(miri))]
//! fn main() -> NcResult<()> {
//!     let mut nc = Nc::with_flags(NCOPTION_NO_ALTERNATE_SCREEN)?;
//!     let plane = nc.stdplane();
//!     plane.putstr("hello world")?;
//!     nc.render()?;
//!     nc.stop()?;
//!     Ok(())
//! }
//! # #[cfg(miri)]
//! # fn main() {}
//! ```
//!
//! The `Drop` trait is not implemented for any wrapping type in this library.
//!
//! This means you still have to manually call the `stop()` method for `Nc`
//! and `NcDirect` objects, and the `destroy()` method for the rest of types that
//! allocate, (like `NcPlane`, `NcMenu`…) at the end of their scope, since the
//!
//! But they do implement methods and use `NcResult` as the return type,
//! for handling errors in the way we are used to in Rust.
//!
//! For the types that don't allocate, most are based on primitives like `i32`,
//! `u32`, `u64`… without a name in the C library. In Rust they are type aliased
//! (e.g.: `NcChannel`, `NcChannelPair`, `NcRgb`, `NcColor`…), to
//! leverage type checking, and they implement methods through traits
//! (e.g. `NcChannelMethods` must be in scope to use the `NcChannel` methods.
//!
//! ### even more like Rust
//!
//! The *WIP* sister crate
//! [`notcurses`](https://github.com/dankamongmen/notcurses-rs) will eventually
//! offer a *closer to Rust*, higher-level, safer, and simpler API, and make it
//! easier to interoperate with the rest of the Rust ecosystem.
//!
//! ## Like C
//!
//! You can access the imported, or reimplemented C API functions directly,
//! and use it in a very similar way as the C library is used.
//!
//! It requires the use of unsafe, since most functions are wrapped directly
//! by `bindgen` marked as such.
//!
//! Error handling is done this way by checking the returned `NcIntResult`,
//! or in case of receiving a pointer, by comparing it to `null_mut()`.
//!
//! ### Example
//!
//! ```ignore
//! use core::ptr::{null, null_mut};
//! use std::process::exit;
//!
//! use libnotcurses_sys::{*, fns::};
//!
//! # #[cfg(not(miri))]
//! fn main() {
//!     let options = ffi::notcurses_options {
//!         termtype: null(),
//!         renderfp: null_mut(),
//!         loglevel: 0,
//!         margin_t: 0,
//!         margin_r: 0,
//!         margin_b: 0,
//!         margin_l: 0,
//!         flags: NCOPTION_NO_ALTERNATE_SCREEN,
//!     };
//!     unsafe {
//!         let nc = notcurses_init(&options, null_mut());
//!         if nc.is_null() {
//!             exit(1);
//!         }
//!         let plane = notcurses_stdplane(nc);
//!         let cols = ncplane_putstr(&mut *plane, "hello world");
//!         if cols < NCRESULT_OK {
//!             notcurses_stop(nc);
//!             exit(cols.abs());
//!         }
//!         if notcurses_stop(nc) < NCRESULT_OK {
//!             exit(2);
//!         }
//!     }
//! }
//! # #[cfg(miri)]
//! # fn main() {}
//! ```
//!
//! ### The `notcurses` C API docs
//!
//! - [API reference (man pages)](https://notcurses.com/)
//! - [Wiki Page](https://nick-black.com/dankwiki/index.php/Notcurses)
//! - [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
//! - [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
//! - [HACKING.md](https://github.com/dankamongmen/notcurses/blob/master/doc/HACKING.md)
//! - [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
//! - [FOSDEM 2021 presentation](https://fosdem.org/2021/schedule/event/notcurses/)
//!
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![allow(clippy::too_many_arguments, clippy::needless_doctest_main)]

mod blitter;
mod r#box;
mod capabilities;
mod cell;
mod channel;
mod dimension;
mod direct;
mod error;
mod fade;
mod fd;
mod file;
mod input;
mod macros;
mod metric;
mod notcurses;
mod palette;
mod pixel;
mod plane;
mod resizecb;
mod stats;
mod time;
mod visual;

pub mod widgets;

// export methods and constants at the root
pub use blitter::*;
pub use capabilities::*;
pub use cell::*;
pub use channel::*;
pub use dimension::*;
pub use direct::*;
pub use error::*;
pub use fade::*;
pub use fd::*;
pub use file::*;
pub use input::*;
pub use macros::*;
pub use metric::*;
pub use notcurses::*;
pub use palette::*;
pub use pixel::*;
pub use plane::*;
pub use r#box::*;
pub use resizecb::*;
pub use stats::*;
pub use time::*;
pub use visual::*;

mod bindings;
pub mod ffi {
    //! Rust FFI bindings, automatically generated with bindgen.
    //!
    //! Almost all of the notcurses API functions are reexported to the public
    //! API, while structs, enums and constants are type aliased or wrapped up.
    //!
    pub use crate::bindings::ffi::*;
}

pub mod fns {
    //! The global notcurses functions, either imported or reimplemented.

    // public re-export of imported functions:
    #[doc(inline)]
    pub use crate::bindings::*;

    // public re-export of reimplemented functions:
    pub use crate::capabilities::reimplemented::*;
    pub use crate::cell::reimplemented::*;
    pub use crate::channel::reimplemented::*;
    pub use crate::direct::reimplemented::*;
    pub use crate::input::reimplemented::*;
    pub use crate::metric::reimplemented::*;
    pub use crate::notcurses::reimplemented::*;
    pub use crate::palette::reimplemented::*;
    pub use crate::pixel::reimplemented::*;
    pub use crate::plane::reimplemented::*;
    pub use crate::resizecb::reimplemented::*;

    // private re-export of helper functions for testing:
    mod helpers {
        #![allow(unused_imports)]
        pub use crate::notcurses::helpers::*;
        pub use crate::plane::helpers::*;
    }
    #[allow(unused_imports)]
    pub(crate) use helpers::*;
}

// public re-export of external crates:
pub use libc;
