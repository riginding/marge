use crate::marge_error;
use console::Style;
use std::fmt;

#[derive(Debug)]
pub enum MargeError {
    Git2Error,
    IOError,
    ReqwestError(::reqwest::Error),
    PathNoParentError,
    ParseError,
}

impl fmt::Display for MargeError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        use MargeError::*;

        match self {
            Git2Error => Ok(marge_error!(
                "It seems like you are not inside a git project."
            )),
            IOError => Ok(marge_error!("I've experienced some errors with IO.")),
            ReqwestError(_) => Ok(marge_error!("I have noticed a network error.")),
            PathNoParentError => Ok(marge_error!("I could not find the supplied path")),
            ParseError => Ok(marge_error!("I've had problem parsing the Config")),
        }
    }
}

impl std::error::Error for MargeError {
    fn cause(&self) -> Option<&std::error::Error> {
        use MargeError::*;

        match self {
            Git2Error => None,
            IOError => None,
            ReqwestError(_) => None,
            PathNoParentError => None,
            ParseError => None,
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
    fn from(e: ::reqwest::Error) -> MargeError {
        MargeError::ReqwestError(e)
    }
}
