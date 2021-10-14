//! `NcSelector` widget.
//!                                 ╭──────────────────────────╮
//!                                 │This is the primary header│
//!   ╭──────────────────────this is the secondary header──────╮
//!   │        ↑                                               │
//!   │ option1 Long text #1                                   │
//!   │ option2 Long text #2                                   │
//!   │ option3 Long text #3                                   │
//!   │ option4 Long text #4                                   │
//!   │ option5 Long text #5                                   │
//!   │ option6 Long text #6                                   │
//!   │        ↓                                               │
//!   ╰────────────────────────────────────here's the footer───╯
//!
//! selection widget -- an ncplane with a title header and a body section. the
//! body section supports infinite scrolling up and down.
//!
//! At all times, exactly one item is selected.

mod builder;
mod methods;

pub use builder::NcSelectorBuilder;

/// High-level widget for selecting one item from a set.
pub type NcSelector = crate::bindings::ffi::ncselector;

/// Options structure for [`NcSelector`].
pub type NcSelectorOptions = crate::bindings::ffi::ncselector_options;

/// Item structure for [`NcSelector`].
pub type NcSelectorItem = crate::bindings::ffi::ncselector_item;
