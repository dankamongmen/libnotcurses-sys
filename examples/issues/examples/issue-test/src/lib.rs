//! # BUG when embedding test
//!
//! Testing with `cargo test` gets hung, until a key is pressed, and the test fails.
//! The flags used doesn't seem to matter in this regard.
//!
//! Running with strace (via cargo-with) doesn't seem to fail:
//! ```sh
//! cargo with "strace -s1000" -- test
//! ```
//!
//! # The minimal example:
//!
//! ```
//! use libnotcurses_sys::*;
//! fn main() -> NcResult<()> {
//!     // let nc = unsafe { Nc::new()? }; // ← fails
//!     // let nc = unsafe { Nc::new_cli()? }; // ← fails
//!     let nc = unsafe { Nc::with_flags( // ← fails
//!         NcOptions::NO_ALTERNATE_SCREEN
//!         | NcOptions::INHIBIT_SETLOCALE
//!         | NcOptions::DRAIN_INPUT
//!         | NcOptions::NO_CLEAR_BITMAPS
//!         | NcOptions::NO_FONT_CHANGES
//!         // | NcOptions::NO_QUIT_SIGHANDLERS // ← this leaves echo disabled
//!         | NcOptions::NO_WINCH_SIGHANDLER
//!         | NcOptions::PRESERVE_CURSOR
//!         | NcOptions::SUPPRESS_BANNERS
//!         )? };
//!     unsafe { nc.stop()? };
//!     Ok(())
//! }
//! ```
