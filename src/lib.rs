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
#![doc = concat!["```\n", include_str!("../examples/hello-world-rust.rs"), "\n```" ]]
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
//! (e.g. `NcChannelApi` must be in scope to use the `NcChannel` methods.
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
#![doc = concat!["```\n", include_str!("../examples/hello-world-c.rs"), "\n```" ]]
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

mod align;
mod alpha;
mod bindings;
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
mod key;
mod log_level;
mod macros;
mod metric;
mod notcurses;
mod palette;
mod pixel;
mod plane;
mod resizecb;
mod scale;
mod stats;
mod string;
mod style;
mod time;
mod visual;

pub mod widgets;

// wrapper types and traits
//
// `*Api` traits allows using methods and associated constants over type aliased
// primitives, like in the case of `NcAlign`, for example.
//
// Note that the names of the implemented traits can't coincide for type aliases
// with the same underlying primitive, like in the case of `NcAlign` & `NcScale`
// in which case are both aliases over `u32`.
pub use align::{NcAlign, NcAlignApi};
pub use alpha::{NcAlpha, NcAlphaApi};
pub use blitter::{NcBlitter, NcBlitterApi};
pub use capabilities::NcCapabilities;
pub use cell::NcCell;
pub use channel::{NcChannel, NcChannelApi, NcChannels, NcChannelsApi, NcComponent, NcRgb};
pub use dimension::{NcDim, NcOffset};
pub use direct::{NcDirect, NcDirectFlags, NcDirectFlagsApi};
pub use error::{NcError, NcIntResult, NcIntResultApi, NcResult};
pub use fade::{NcFadeCb, NcFadeCtx};
pub use fd::{NcFdPlane, NcFdPlaneOptions, NcSubproc, NcSubprocOptions};
pub use file::NcFile;
pub use input::{NcEvType, NcEvTypeApi, NcInput, NcMiceEvents, NcMiceEventsApi, NcReceived};
pub use key::NcKey;
pub use log_level::{NcLogLevel, NcLogLevelApi};
pub use macros::*;
pub use notcurses::{Nc, NcOptions};
pub use palette::{NcPalette, NcPaletteIndex};
pub use pixel::{NcPixel, NcPixelGeometry, NcPixelImpl, NcPixelImplApi};
pub use plane::{NcPlane, NcPlaneOptions};
pub use r#box::{NcBoxMask, NcBoxMaskApi};
pub use resizecb::{NcResizeCb, NcResizeCbApi, NcResizeCbUnsafe};
pub use scale::{NcScale, NcScaleApi};
pub use stats::NcStats;
pub use string::NcString;
pub use style::{NcStyle, NcStyleApi};
pub use time::NcTime;
pub use visual::{NcRgba, NcVGeom, NcVisual, NcVisualGeometry, NcVisualOptions};

pub mod c_api {
    //! The C API including global constants, functions and structs.
    //!
    //! Includes also both automatically imported functions by bindgen, and
    //! manually wrapped and reimplemented global functions.

    // public re-export of external crates:
    pub use libc;

    pub mod ffi {
        //! Rust FFI bindings, automatically generated with bindgen.
        //!
        //! Almost all of the notcurses API functions are reexported to the public
        //! API, while structs, enums and constants are type aliased or wrapped up.
        //!
        pub use crate::bindings::ffi::*;
    }

    // public re-export of imported functions & structs:
    #[doc(inline)]
    pub use crate::bindings::*;

    // public re-export of reimplemented functions:
    pub use crate::capabilities::reimplemented::*;
    pub use crate::cell::reimplemented::*;
    pub use crate::channel::reimplemented::*;
    pub use crate::direct::reimplemented::*;
    pub use crate::input::reimplemented::*;
    pub use crate::key::reimplemented::*;
    pub use crate::metric::reimplemented::*;
    pub use crate::notcurses::reimplemented::*;
    pub use crate::palette::reimplemented::*;
    pub use crate::pixel::reimplemented::*;
    pub use crate::plane::reimplemented::*;
    pub use crate::resizecb::reimplemented::*;

    // public re-export of reimplemented constants:
    pub use crate::align::constants::*;
    pub use crate::alpha::constants::*;
    pub use crate::blitter::constants::*;
    pub use crate::channel::constants::*;
    pub use crate::direct::constants::*;
    pub use crate::error::constants::*;
    pub use crate::input::constants::*;
    pub use crate::key::constants::*;
    pub use crate::log_level::constants::*;
    pub use crate::metric::constants::*;
    pub use crate::notcurses::constants::*;
    pub use crate::palette::constants::*;
    pub use crate::pixel::constants::*;
    pub use crate::plane::constants::*;
    pub use crate::r#box::constants::*;
    pub use crate::scale::constants::*;
    pub use crate::style::constants::*;
    pub use crate::visual::constants::*;
    pub use crate::widgets::menu::constants::*;
    pub use crate::widgets::plot::constants::*;
    pub use crate::widgets::progbar::constants::*;
    pub use crate::widgets::reader::constants::*;
    pub use crate::widgets::reel::constants::*;
    pub use crate::widgets::tabbed::constants::*;

    // private re-export of helper functions for testing:
    mod helpers {
        #![allow(unused_imports)]
        pub use crate::notcurses::helpers::*;
        pub use crate::plane::helpers::*;
    }
    #[allow(unused_imports)]
    pub(crate) use helpers::*;
}
