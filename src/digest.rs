/*
 * digest.rs
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

use Result;
use shaman::digest::Digest as DigestTrait;
use shaman::sha1::Sha1;
use shaman::sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fmt::{self, Debug};
use std::io::Read;

enum DigestImp {
    Sha1(Sha1),
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
}

impl Debug for DigestImp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DigestImp::*;

        let name = match self {
            Sha1(_) => "Sha1",
            Sha224(_) => "Sha224",
            Sha256(_) => "Sha256",
            Sha384(_) => "Sha384",
            Sha512(_) => "Sha512",
        };

        write!(f, "DigestImp {{ {} }}", name)
    }
}

#[derive(Debug)]
pub struct Digest {
    digest: DigestImp,
    buffer: Vec<u8>,
};

impl Digest {
    pub fn new(name: &str) -> Option<Self> {
        let digest = match name {
            "1" | "sha1" | "SHA1" => DigestImp::Sha1(Sha1::new()),
            "224" | "sha224" | "SHA224" => DigestImp::Sha224(Sha224::new()),
            "256" | "sha256" | "SHA256" => DigestImp::Sha256(Sha256::new()),
            "384" | "sha384" | "SHA384" => DigestImp::Sha384(Sha384::new()),
            "512" | "sha512" | "SHA512" => DigestImp::Sha512(Sha512::new()),
            _ => return None,
        };

        Some(Digest {
            digest,
            buffer: Vec::new(),
        })
    }

    pub fn hash<R: Read>(&mut self, source: R) -> Result<String> {
        source.read_to_end(&mut self.buffer)?;
        self.digest.input(&self.buffer);

        let hash = self.digest.result_str();
        self.digest.reset();
        self.buffer.clear();

        Ok(hash)
    }

    fn get(&self) -> &DigestTrait {
        use self::DigestImp::*;

        match &self.0 {
            Sha1(digest) => digest,
            Sha224(digest) => digest,
            Sha256(digest) => digest,
            Sha384(digest) => digest,
            Sha512(digest) => digest,
        }
    }

    fn get_mut(&mut self) -> &mut DigestTrait {
        use self::DigestImp::*;

        match &mut self.0 {
            Sha1(digest) => digest,
            Sha224(digest) => digest,
            Sha256(digest) => digest,
            Sha384(digest) => digest,
            Sha512(digest) => digest,
        }
    }
}
