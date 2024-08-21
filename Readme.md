# iitiiri 🧬 💻

![tests](https://github.com/natir/iitiiri/workflows/tests/badge.svg)
![lints](https://github.com/natir/iitiiri/workflows/lints/badge.svg)
![msrv](https://github.com/natir/iitiiri/workflows/msrv/badge.svg)
[![website](https://github.com/natir/iitiiri/workflows/website/badge.svg)](https://natir.github.io/iitiiri)
[![dev-doc](https://img.shields.io/badge/dev-doc-blue)](https://natir.github.io/iitiiri/doc/iitiiri)
[![license](https://img.shields.io/badge/license-MIT-purple)](/blob/main/LICENSE)
[![copier](https://img.shields.io/badge/copier-template-yellow)](https://github.com/natir/copier-rust)

Implicit Interval Tree with Interpolation Index Rust Implementation

This is a reimplementation/huge inspiration of [lh3 cgranges](https://github.com/lh3/cgranges) and [mlin iitii](https://github.com/mlin/iitii/), main original work are parallelisation of algorithm.

## Use

Add this in your Cargo.toml
```
iitiiri = { git = "git@github.com:natir/iitiiri.git" }
```

For information on how to build the different intervals tree, understand their differences and parameters you can consult the [online manual](https://natir.github.io/iitiiri/usage.html) and check the [examples](https://github.com/natir/iitiiri/tree/main/examples).

Feature:
- parallel: activate parallel code
- bench: makes some features public, this feature is necessary to launch benchmarks

## Contributors

- [Pierre Marijon](https://github.com/natir/)

## Minimum Supported Rust Version

Currently the minimum supported Rust version is 1.67.
