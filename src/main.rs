/*
 * main.rs
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

#![deny(missing_debug_implementations)]

extern crate clap;
extern crate fts;
extern crate ignore;
extern crate shaman;

mod args;
mod digest;
mod error;
mod hasher;

use args::Arguments;
use digest::Digest;
use hasher::hash_recursively;
use std::process::exit;

pub use error::{Error, StdError};

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

fn main() {
    let args = match Arguments::parse() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error parsing arguments: {}", err);
            exit(1);
        },
    };

    hash_recursively(&args);
}
