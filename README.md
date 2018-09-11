# `vitallocator`

*A Rust allocator interface to the PS Vita kernel allocator*.

[![TravisCI](https://img.shields.io/travis/vita-rust/vitallocator/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/vita-rust/vitallocator/builds)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/vita-rust/vitallocator)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
[![Crate](https://img.shields.io/crates/v/vitallocator.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/vitallocator)
[![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/vitallocator)
[![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)


## Usage

This crate depends on [`psp2-sys`](https://github.com/vita-rust/psp2-sys),
so you'll need the [`vitasdk`](https://vitasdk.org) set up and the
`$VITASDK` environment variable set.


## Credits

* [**VitaSDK team**](http://vitasdk.org/) for the `arm-vita-eabi` toolchain, `psp2` headers, ...
* [**Team Molecule**](http://henkaku.xyz/) for the `Henkaku` hard work.


## Disclaimer

*`vitallocator` is not affiliated, sponsored, or otherwise endorsed by Sony
Interactive Entertainment, LLC. PlayStation and PS Vita are trademarks or
registered trademarks of Sony Interactive Entertainment, LLC. This software
is provided "as is" without warranty of any kind under the MIT License.*
