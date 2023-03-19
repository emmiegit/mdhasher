## mdhasher
mdhasher, short for "Media Hasher", is a program to take files with certain extensions, and rename them based on a hash of their contents. This is a simple form of deduplication, and also keeps files named consistently. Based on [media-hash.py](https://github.com/emmiegit/scripts/blob/master/archv/media-hash.py), but rewritten to be faster and easier to maintain.

Available under the terms of the [MIT License](LICENSE).

## Building
```
$ cargo build --release
```

The final binary will be at `target/release/mdhasher`.
