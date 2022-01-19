//! `NcKey`

// functions manually reimplemented: 2
// ------------------------------------------
// (+) done: 2
// (#) test: 0
// ------------------------------------------
// + nckey_mouse_p
// + nckey_synthesized_p

pub(crate) mod reimplemented;

#[allow(clippy::module_inception)]
mod key;
mod keymod;
pub use {key::NcKey, keymod::NcKeyMod};

pub(crate) mod c_api {
    pub use super::key::c_api::*;
    pub use super::keymod::c_api::*;
}
