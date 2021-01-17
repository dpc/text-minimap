<p align="center">
  <a href="https://travis-ci.org/dpc/text-minimap">
      <img src="https://img.shields.io/travis/dpc/text-minimap/master.svg" alt="Travis CI Build Status">
  </a>

  <a href="https://crates.io/crates/text-minimap">
      <img src="https://img.shields.io/crates/d/text-minimap.svg" alt="text-minimap on crates.io">
  </a>

  <a href="https://docs.rs/text-minimap">
      <img src="https://docs.rs/text-minimap/badge.svg" alt="text-minimap documentation on docs.rs">
  </a>
</p>

# text-minimap - Generate text minimap/preview using Braille Patterns

Library and command-line utility to efficiently generate minimap/preview
of a given text.


![text-minimap example](http://i.imgur.com/JcR6rwS.png)

## Status

* [x] Efficient io and streaming
* [x] Multiple scaling settings
* [ ] Tab (`\t`) width support
* [ ] Soft-wrapping on text-width support
* [ ] C API

## Alternatives

* [wfxr/code-minimap](https://github.com/wfxr/code-minimap) - seems also written in Rust and with more effort put into it, so you probably should prefer it over this project; probably - I had no time to check the details myself;

## Installing

If you don't have rust compiler and cargo installed follow instructions on
https://www.rustup.rs/ .

Afterwards:

```
cargo install text-minimap
```

## Using

Check `text-minimap --help` output.

## Verification Recommendation

To help with the maintaince, the ownership of this crate is potentially shared between multiple developers.
It is recommend to always use [cargo-crev](https://github.com/crev-dev/cargo-crev/tree/master/cargo-crev)
to verify trustworthiness of each of your dependencies, including this one.
