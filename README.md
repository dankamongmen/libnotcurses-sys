[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://docs.rs/libnotcurses-sys/)
[![MSRV: 1.56.0](https://flat.badgen.net/badge/MSRV/1.56.0/purple)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)
[![Lines Of Code](https://tokei.rs/b1/github/dankamongmen/libnotcurses-sys?category=code)](https://github.com/dankamongmen/libnotcurses-sys)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

It is built with several layers of zero-overhead abstractions
over the C functions and pointers, accessed through FFI.


hello world example
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

## Versioning

Current version `3.1.2` is compatible with notcurses `3.0.3`.

Current major version **`3`** is considered a development version. Just as if it
it were a semver major version `0`.
