[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://docs.rs/libnotcurses-sys/)
[![MSRV: 1.56.0](https://flat.badgen.net/badge/MSRV/1.56.0/purple)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)
[![Lines Of Code](https://tokei.rs/b1/github/dankamongmen/libnotcurses-sys?category=code)](https://github.com/dankamongmen/libnotcurses-sys)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

It is built with several layers of zero-overhead abstractions
over the C functions and pointers, accessed through FFI.

It adds greater safety and type correctness over the underlying C library API,
while trying to remain very close to it.

## Example

```rust
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    nc.stdplane().putstr("hello world")?;
    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
```

## Status

Current version `3.1.0-alpha.1` is compatible with notcurses `3.0.1` (unreleased).

The [documentation](https://docs.rs/libnotcurses-sys/3.1.0-alpha.1/) is very comprehensive.

The library is very much functional, although the API is somewhat unstable,
and is evolving rapidly. The versioning follows *semver*, with the caveat that
current major version **3** is treated as if it were a major version **0**.
