# Clairiere 🧬 💻

![tests](https://github.com/natir/clairiere/workflows/tests/badge.svg)
![lints](https://github.com/natir/clairiere/workflows/lints/badge.svg)
![msrv](https://github.com/natir/clairiere/workflows/msrv/badge.svg)
[![website](https://github.com/natir/clairiere/workflows/website/badge.svg)](https://natir.github.io/clairiere)
[![dev-doc](https://img.shields.io/badge/dev-doc-blue)](https://natir.github.io/clairiere/doc/clairiere)
[![license](https://img.shields.io/badge/license-MIT-purple)](/blob/main/LICENSE)
[![copier](https://img.shields.io/badge/copier-template-yellow)](https://github.com/natir/copier-rust)

Clairière, a rust implementation of implicit interval tree with interpolation index.

This is a reimplementation/huge inspiration of [lh3 cgranges](https://github.com/lh3/cgranges) and [mlin iitii](https://github.com/mlin/iitii/), but with parallelisation of algorithm.

Naming: *Clairière* is the French word for glade, an interval between trees.

## Use

Add this in your Cargo.toml
```
clairiere = { git = "https://github.com/natir/clairiere.git" }
```

For information on how to build the different intervals tree, understand their differences and parameters you can consult the [online manual](https://natir.github.io/clairiere/usage.html) and check the [examples](https://github.com/natir/clairiere/tree/main/examples).

Feature:
- parallel: activate parallel code
- bench: makes some function and method public, this feature is necessary to launch benchmarks
- eval_guess: a feature to let clairiere write in stdout some metrics usefull for algorithm evaluation

## Contributors

- [Pierre Marijon](https://github.com/natir/)

## Minimum Supported Rust Version

Currently the minimum supported Rust version is 1.67.
