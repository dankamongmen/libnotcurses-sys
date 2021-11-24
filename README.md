[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://dankamongmen.github.io/notcurses/rustdoc/libnotcurses_sys/)
[![MSRV: 1.56.0](https://flat.badgen.net/badge/MSRV/1.56.0/purple)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

It is built with several layers of zero-overhead abstractions
over the C functions and pointers accessed through FFI.

It adds greater safety and type correctness over the underlying C library API,
while trying to remain very close to it.

```
notcurses           : C library
libnotcurses-sys  ← : C⇄Rust bridge library *(you are here)*
notcurses-rs        : Rust library
```

## Versioning & compatibility

Current major version 2 is not following semver.

Major next version 3 will follow semver with the caveat of being considered a
development version similar as if it were a major version 0.

Each release will indicate the compatibility with a specific version of the
notcurses C API library.

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
(e.g. `NcChannelMethods` must be in scope to use the `NcChannel` methods.

## Official C API docs

- [API reference (man pages)](https://notcurses.com/)
- [Wiki Page](https://nick-black.com/dankwiki/index.php/Notcurses)
- [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
- [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
- [HACKING.md](https://github.com/dankamongmen/notcurses/blob/master/doc/HACKING.md)
- [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
- [FOSDEM 2021 presentation](https://fosdem.org/2021/schedule/event/notcurses/)
