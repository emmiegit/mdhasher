/*
 * config.rs
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

use {Error, Result};
use clap::{App, Arg};
use std::path::PathBuf;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum LoggingMode {
    Terminal,
    Full,
    Quiet,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    logging: LoggingMode,
    all: bool,
    ignore: bool,
    ignore_file: Option<PathBuf>,
    paths: Vec<PathBuf>,
}

impl Arguments {
    pub fn parse() -> Result<Self> {
        let matches = App::new("Media Hasher")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Ammon Smith")
            .about("A program that takes files and renames them based on the hash of their contents.")
            .max_term_width(110)
            .arg(
                Arg::with_name("logging")
                    .short("l")
                    .long("log")
                    .value_name("MODE")
                    .help(
                        "Specify logging mode, 'terminal' (default), overwrites new lines as they are printed, 'full' prints a new line for each log entry, and 'none' or 'quiet' outputs nothing except error messages."
                    )
            )
            .arg(
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("Check all files, even if their timestamp isn't recent.")
            )
            .arg(
                Arg::with_name("noignore")
                    .short("N")
                    .long("noignore")
                    .help("Doesn't respect any .ignore files, renaming all files that match conditions.")
            )
            .arg(
                Arg::with_name("ignore")
                    .short("I")
                    .long("ignore")
                    .value_name("FILE")
                    .help("Specify an additional .ignore file to consider while walking.")
            )
            .arg(
                Arg::with_name("paths")
                    .multiple(true)
                    .required(true)
                    .help("A list of paths to hash.")
            )
            .get_matches();

        let mut config = Arguments::default();

        if let Some(mode) = matches.value_of("logging") {
            config.logging = match mode {
                "terminal" | "interactive" | "default" => LoggingMode::Terminal,
                "full" | "lines" | "pipe" => LoggingMode::Full,
                "none" | "quiet" => LoggingMode::Quiet,
                _ => return Err(Error::Msg(format!("Unknown logging mode: {}", mode))),
            };
        }

        config.all = matches.is_present("all");
        config.ignore = !matches.is_present("noignore");

        if let Some(path) = matches.value_of_os("ignore") {
            config.ignore_file = Some(PathBuf::from(path));
        }

        for path in matches.values_of_os("paths").unwrap() {
            config.paths.push(PathBuf::from(path));
        }

        Ok(config)
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            logging: LoggingMode::Terminal,
            all: false,
            ignore: true,
            ignore_file: None,
            paths: Vec::new(),
        }
    }
}
