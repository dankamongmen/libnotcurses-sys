//! Error handling with `NcError`, `NcResult` & `NcResult_i32`

/// The result type for the Rust methods API.
pub type NcResult<T> = Result<T, NcError>;

/// The error type for the Rust methods API.
#[derive(Debug, Clone, Default)]
pub struct NcError {
    pub int: c_api::NcResult_i32,
    pub msg: String,
}

/// # Methods
impl NcError {
    /// New NcError with default [`NCRESULT_ERR`][c_api::NCRESULT_ERR]
    /// error number, and no message.
    pub fn new() -> Self {
        Self { int: c_api::NCRESULT_ERR, ..Default::default() }
    }

    /// New NcError with custom error number, and without message.
    pub fn new_err(int: c_api::NcResult_i32) -> Self {
        Self { int, ..Default::default() }
    }

    /// New NcError with default [`NCRESULT_ERR`][c_api::NCRESULT_ERR]
    /// error number and a custom message.
    pub fn new_msg(msg: &str) -> Self {
        Self { int: c_api::NCRESULT_ERR, msg: msg.to_string() }
    }

    /// New NcError with both a custom error number and a custom message.
    pub fn with_msg(int: c_api::NcResult_i32, msg: &str) -> Self {
        Self { int, msg: msg.to_string() }
    }
}

mod std_impls {
    use super::NcError;
    use std::{self, error, fmt};

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
}

pub(crate) mod c_api {
    /// The value used to return errors by the underlying C API.
    ///
    /// A value < 0 means error, (usually -1).
    ///
    /// It's recommended to use [`NcResult`][crate::NcResult] instead.
    ///
    /// # Defined constants:
    /// - [`NCRESULT_OK`]
    /// - [`NCRESULT_ERR`]
    /// - [`NCRESULT_MAX`]
    pub type NcResult_i32 = i32;

    /// [`NcResult_i32`] OK value.
    pub const NCRESULT_OK: i32 = 0;

    /// [`NcResult_i32`] ERROR value.
    pub const NCRESULT_ERR: i32 = -1;

    /// [`NcResult_i32`] MAX value.
    pub const NCRESULT_MAX: i32 = i32::MAX;
}
