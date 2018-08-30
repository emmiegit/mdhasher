/*
 * error.rs
 *
 * mdhasher - Rename media files to the hash of their contents.
 * Copyright (c) 2018 Ammon Smith
 *
 * mdhasher is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

use self::Error::*;
use std::{fmt, num, io};

pub use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    StaticMsg(&'static str),
    Msg(String),
    Io(io::Error),
    IntParse(num::ParseIntError),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            StaticMsg(s) => s,
            Msg(ref s) => s,
            Io(ref e) => e.description(),
            IntParse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            StaticMsg(_) | Msg(_) => None,
            Io(ref e) => Some(e),
            IntParse(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", StdError::description(self))
    }
}

// Auto-conversion trait implementations
impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Msg(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::IntParse(error)
    }
}
