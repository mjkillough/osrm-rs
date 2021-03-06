use std::error;
use std::ffi::{self, CStr};
use std::fmt::{self, Debug, Display};
use std::result::Result as StdResult;

use crate::impl_drop;

pub struct OsrmcError {
    handle: osrmc_sys::osrmc_error_t,
}

impl_drop!(OsrmcError, osrmc_sys::osrmc_error_destruct);

impl Display for OsrmcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        unsafe {
            let ptr = osrmc_sys::osrmc_error_message(self.handle);
            write!(f, "{}", CStr::from_ptr(ptr).to_string_lossy())
        }
    }
}

impl Debug for OsrmcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        Display::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum Error {
    Osrmc(OsrmcError),
    FfiNul(ffi::NulError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        match self {
            Error::Osrmc(inner) => Display::fmt(inner, f),
            Error::FfiNul(inner) => Display::fmt(inner, f),
        }
    }
}

impl error::Error for Error {}

impl From<osrmc_sys::osrmc_error_t> for Error {
    fn from(handle: osrmc_sys::osrmc_error_t) -> Error {
        Error::Osrmc(OsrmcError { handle })
    }
}

impl From<ffi::NulError> for Error {
    fn from(other: ffi::NulError) -> Error {
        Error::FfiNul(other)
    }
}

pub type Result<T> = StdResult<T, Error>;
