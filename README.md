[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://docs.rs/libnotcurses-sys/)
[![MSRV: 1.56.0](https://flat.badgen.net/badge/MSRV/1.56.0/purple)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

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

## Versioning Notes

Current version **3** follows *semver* as if it were major version *0*.

This means a rapid pace of development of the API, while any breaking changes
happening wont be reflected by a major version bump.

The library is very much functional, although the API is somewhat unstable.
