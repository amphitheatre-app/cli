// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Inspired by dotenv::errors

use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl Error {
    pub fn not_found(&self) -> bool {
        if let Error::Io(ref io_error) = *self {
            return io_error.kind() == io::ErrorKind::NotFound;
        }
        false
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(fmt, "{}", err),
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error as StdError;

    use super::*;

    #[test]
    fn test_io_error_source() {
        let err = Error::Io(std::io::ErrorKind::PermissionDenied.into());
        let io_err = err.source().unwrap().downcast_ref::<std::io::Error>().unwrap();
        assert_eq!(std::io::ErrorKind::PermissionDenied, io_err.kind());
    }

    #[test]
    fn test_error_not_found_true() {
        let err = Error::Io(std::io::ErrorKind::NotFound.into());
        assert!(err.not_found());
    }

    #[test]
    fn test_error_not_found_false() {
        let err = Error::Io(std::io::ErrorKind::PermissionDenied.into());
        assert!(!err.not_found());
    }

    #[test]
    fn test_io_error_display() {
        let err = Error::Io(std::io::ErrorKind::PermissionDenied.into());
        let io_err: std::io::Error = std::io::ErrorKind::PermissionDenied.into();

        let err_desc = format!("{}", err);
        let io_err_desc = format!("{}", io_err);
        assert_eq!(io_err_desc, err_desc);
    }
}
