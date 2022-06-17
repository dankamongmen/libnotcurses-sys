[![Crate](https://img.shields.io/crates/v/libnotcurses-sys.svg)](https://crates.io/crates/libnotcurses-sys)
[![API](https://docs.rs/libnotcurses-sys/badge.svg)](https://docs.rs/libnotcurses-sys/)
[![MSRV: 1.58.1](https://flat.badgen.net/badge/MSRV/1.58.1/purple)](https://blog.rust-lang.org/2022/01/20/Rust-1.58.1.html)
[![Lines Of Code](https://tokei.rs/b1/github/dankamongmen/libnotcurses-sys?category=code)](https://github.com/dankamongmen/libnotcurses-sys)

`libnotcurses-sys` is a low-level Rust wrapper for the
[notcurses C library](https://www.github.com/dankamongmen/notcurses/)

<!-- It's recommended to use the [notcurses higher level bindings][notcurses-rs].
-
[notcurses-rs]: https://crates.io/crates/notcurses
-->

## Example

```rust
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = unsafe { Nc::new_cli()? };
    let stdplane = unsafe { nc.stdplane() };
    stdplane.putstr("\nhello world!\n")?;
    nc.render()?;
    unsafe { nc.stop()? };
    Ok(())
}
```

## Versioning

Current version `3.6.1` is compatible with notcurses `3.0.8`.

*Current major version `3` is considered a development version*
