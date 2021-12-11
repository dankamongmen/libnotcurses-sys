//! Error handling with `Error`, `NcResult` & `NcIntResult` for error handling

use std::{self, error, fmt};

/// The value used to return errors by the underlying C API (alias of `i32`).
///
/// A value < 0 means error, (usually -1).
///
/// # Defined constants:
pub type NcIntResult = i32;

crate::impl_api![
    NcIntResult,
    NcIntResultApi,
    /// OK value, for the functions that return [`NcIntResult`].
    const OK: i32 = 0;,
    /// ERROR value, for the functions that return an [`NcIntResult`].
    const ERR: i32 = -1;,
    /// MAX value, for the functions that return [`NcIntResult`].
    const MAX: i32 = i32::MAX;
];

pub(crate) mod constants {
    #[allow(unused_imports)] // for doc comments
    use crate::NcIntResult;

    /// OK value, for the functions that return [`NcIntResult`].
    pub const NCRESULT_OK: i32 = 0;

    /// ERROR value, for the functions that return an [`NcIntResult`].
    pub const NCRESULT_ERR: i32 = -1;

    /// MAX value, for the functions that return [`NcIntResult`].
    pub const NCRESULT_MAX: i32 = i32::MAX;
}

/// The error type for the Rust methods API.
#[derive(Debug, Clone, Default)]
pub struct NcError {
    /// [NcIntResult].
    pub int: i32,
    pub msg: String,
}

impl fmt::Display for NcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "NcError {}: {}", self.int, self.msg)
    }
}

impl error::Error for NcError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl NcError {
    /// New NcError with default
    /// [`NcIntResult::ERR`][NcIntResult#associatedconstant.ERR]
    /// error number, and no message.
    pub fn new() -> Self {
        Self { int: NcIntResult::ERR, ..Default::default() }
    }

    /// New NcError with custom error number, and without message.
    pub fn new_err(int: NcIntResult) -> Self {
        Self { int, ..Default::default() }
    }

    /// New NcError with default
    /// [`NcIntResult::ERR`][NcIntResult#associatedconstant.ERR]
    /// error number and a custom message.
    pub fn new_msg(msg: &str) -> Self {
        Self { int: NcIntResult::ERR, msg: msg.to_string() }
    }

    /// New NcError with both a custom error number and a custom message.
    pub fn with_msg(int: NcIntResult, msg: &str) -> Self {
        Self { int, msg: msg.to_string() }
    }
}

/// The result type for the Rust methods API.
pub type NcResult<T> = Result<T, NcError>;
