use std::fmt;

#[derive(Debug)]
pub enum MargeError {
    Git2Error,
    IOError,
    ReqwestError(::reqwest::Error),
    PathNoParentError,
}

impl fmt::Display for MargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MargeError::*;

        match self {
            Git2Error => write!(f, "marge: It seems like you are not inside a git project."),
            IOError => write!(f, "marge: I've experienced some errors with IO."),
            ReqwestError(e) => write!(f, "marge: I have noticed a network error. {:?}", e),
            PathNoParentError => write!(f, "marge: A supplied path did not exist."),
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
