[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://dankamongmen.github.io/notcurses/rustdoc/libnotcurses_sys/)
[![MSRV: 1.56.0](https://flat.badgen.net/badge/MSRV/1.56.0/purple)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

It is built with several layers of zero-overhead abstractions
over the C functions and pointers, accessed through FFI.

It adds greater safety and type correctness over the underlying C library API,
while trying to remain very close to it.

## Versioning

Current libnotcurses-sys **`3.0.0`** is compatible with notcurses API **`3.0.0`**.

Both project's version number are independent from each other. Historically,
version *1* and *2* of this library didn't follow semver, but was tied to the
API version, never enjoying a major version *0* for exploratory development.

This is why version **3** is following semver as if it were major version *0*.

This means a rapid pace of development of the API, while any breaking changes
happening wont be reflected by a major version bump.

## Example

Where you use the more safely wrapped types, with its methods and constructors,
and error handling with the `NcResult` enum:

```rust
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = unsafe { Nc::new_cli()? };
    let plane = nc.stdplane();
    plane.putstr("hello world")?;
    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
```

The `Drop` trait is not implemented for any wrapping type in this library.

This means you still have to manually call the `stop()` method for `Nc`
and `NcDirect` objects, and the `destroy()` method for the rest of types that
allocate, (like `NcPlane`, `NcMenu`…) at the end of their scope.

But they do implement methods and use `NcResult` as the return type,
for handling errors in the way we are used to in Rust.

For the types that don't allocate, most are based on primitives like `i32`,
`u32`, `u64`… without a name in the C library. In Rust they are type aliased
(e.g.: `NcChannel`, `NcChannelPair`, `NcRgb`, `NcColor`…), to
leverage type checking, and they implement methods through traits
(e.g. `NcChannelApi` must be in scope to use the `NcChannel` methods.

## Official C API docs

- [API reference (man pages)](https://notcurses.com/)
- [Wiki Page](https://nick-black.com/dankwiki/index.php/Notcurses)
- [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
- [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
- [HACKING.md](https://github.com/dankamongmen/notcurses/blob/master/doc/HACKING.md)
- [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
- [FOSDEM 2021 presentation](https://fosdem.org/2021/schedule/event/notcurses/)
