use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum MargeError {
  Git2Error,
  IOError,
  ReqwestError,
}

impl fmt::Display for MargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MargeError::*;

        match self {
            Git2Error => write!(f, "Error in git"),
            IOError => write!(f, "Error in io"),
            ReqwestError => write!(f, "Error in reqwest"),
        }
    }

}

impl std::error::Error for MargeError {
    fn cause(&self) -> Option<&std::error::Error> {
        use MargeError::*;

        match self {
            Git2Error => None,
            IOError => None,
            ReqwestError => None,
        }
    }
}

impl From<::git2::Error> for MargeError {
    fn from(_: ::git2::Error) -> MargeError {
        MargeError::Git2Error
    }
}

impl From<::std::io::Error> for MargeError {
    fn from(_: ::std::io::Error) -> MargeError {
        MargeError::IOError
    }
}

impl From<::reqwest::Error> for MargeError {
    fn from(_: ::reqwest::Error) -> MargeError {
        MargeError::ReqwestError
    }
}
