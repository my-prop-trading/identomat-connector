use std::fmt;
use std::error::Error as StdError;

use flurl::FlUrlError;
use http::status::InvalidStatusCode;
pub struct IdentomatError {
    pub message: String,
}

impl fmt::Display for IdentomatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl fmt::Debug for IdentomatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl StdError for IdentomatError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl From<FlUrlError> for IdentomatError {
    fn from(error: FlUrlError) -> Self {
        IdentomatError {
            message: error.to_string(),  // You can customize the message
        }
    }
}

impl From<InvalidStatusCode> for IdentomatError {
    fn from(error: InvalidStatusCode) -> Self {
        IdentomatError {
            message: error.to_string(),
        }
    }
}