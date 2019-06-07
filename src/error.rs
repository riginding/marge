extern crate serde;
use serde::{Serialize};

#[derive(Debug,Serialize)]
pub enum MargeError {
  Git2Error(::git2::Error),
  IOError(::std::io::Error),
  ReqwestError(::reqwest::Error)
}

impl From<::git2::Error> for MargeError {
    fn from(e: ::git2::Error) -> MargeError {
        MargeError::Git2Error(e)
    }
}

impl From<::std::io::Error> for MargeError {
    fn from(e: ::std::io::Error) -> MargeError {
        MargeError::IOError(e)
    }
}

impl From<::reqwest::Error> for MargeError {
    fn from(e: ::reqwest::Error) -> MargeError {
        MargeError::ReqwestError(e)
    }
}

