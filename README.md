[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://dankamongmen.github.io/notcurses/rustdoc/libnotcurses_sys/)
[![MSRV: 1.49.0](https://flat.badgen.net/badge/MSRV/1.49.0/purple)](https://blog.rust-lang.org/2020/11/19/Rust-1.49.html)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

It is built with several layers of zero-overhead abstractions
over the C functions and pointers accessed through FFI.

It adds greater safety and type correctness over the underlying C library API,
while trying to remain very close to it.

It offers the choice of using it [**more like Rust**](#like-rust)
and/or [**more like C**](#like-C).

```
notcurses           : C library
libnotcurses-sys  ← : C⇄Rust bridge library *(you are here)*
notcurses-rs        : Rust library
```

## like Rust

Where you use the more safely wrapped types, with its methods and constructors,
and error handling with the `NcResult` enum:

```rust
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = Nc::new_cli()?;
    let plane = nc.stdplane();
    plane.putstr("hello world")?;
    nc.render()?;
    nc.stop()?;
    Ok(())
}
```

The `Drop` trait is not implemented for any wrapping type in this library.

This means you still have to manually call the `stop()` method for `Nc`
and `NcDirect` objects, and the `destroy()` method for the rest of types that
allocate, (like `NcPlane`, `NcMenu`…) at the end of their scope, since the

But they do implement methods and use `NcResult` as the return type,
for handling errors in the way we are used to in Rust.

For the types that don't allocate, most are based on primitives like `i32`,
`u32`, `u64`… without a name in the C library. In Rust they are type aliased
(e.g.: `NcChannel`, `NcChannelPair`, `NcRgb`, `NcColor`…), to
leverage type checking, and they implement methods through traits
(e.g. `NcChannelMethods` must be in scope to use the `NcChannel` methods.

### even more like Rust

The *WIP* sister crate
[`notcurses`](https://github.com/dankamongmen/notcurses-rs) will eventually
offer a *closer to Rust*, higher-level, safer, and simpler API, and make it
easier to interoperate with the rest of the Rust ecosystem.

## like C

You can access the imported, or reimplemented C API functions directly,
and use it in a very similar way as the C library is used.

It requires the use of unsafe, since most functions are wrapped directly
by `bindgen` marked as such.

Error handling is done this way by checking the returned `NcIntResult`,
or in case of receiving a pointer, by comparing it to `null_mut()`.

### Example

```rust
use core::ptr::{null, null_mut};
use std::process::exit;

use libnotcurses_sys::c_api::*;

fn main() {
    let options = ffi::notcurses_options {
        termtype: null(),
        loglevel: 0,
        margin_t: 0,
        margin_r: 0,
        margin_b: 0,
        margin_l: 0,
        flags: NCOPTION_NO_ALTERNATE_SCREEN
            | NCOPTION_PRESERVE_CURSOR
            | NCOPTION_SUPPRESS_BANNERS
    };
    unsafe {
        let nc = notcurses_init(&options, null_mut());
        if nc == null_mut() {
            exit(1);
        }
        let plane = notcurses_stdplane(nc);
        let cols = ncplane_putstr(&mut *plane, "hello world");
        if cols < NCRESULT_OK {
            notcurses_stop(nc);
            exit(cols.abs());
        }
        if notcurses_render(nc) < NCRESULT_OK {
            exit(2);
        }
        if notcurses_stop(nc) < NCRESULT_OK {
            exit(3);
        }
    }
}
```

### Official C API docs

- [API reference (man pages)](https://notcurses.com/)
- [Wiki Page](https://nick-black.com/dankwiki/index.php/Notcurses)
- [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
- [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
- [HACKING.md](https://github.com/dankamongmen/notcurses/blob/master/doc/HACKING.md)
- [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
- [FOSDEM 2021 presentation](https://fosdem.org/2021/schedule/event/notcurses/)
