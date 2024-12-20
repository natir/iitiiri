# Generate figure

## Dependency

To generate figure of paper you should have in your PATH:
- magick

```
magick figure/bst.svg figure/bst.png
```

# Reproduce experiment

## Dependency

To reproduce these experiment you should have in your PATH:
- a C complier (call as `cc` in snakemake)
- a C++ complier (call as `c++` in snakemake)
- [the rust compiler environment](https://rustup.rs/)
- /usr/bin/time
- [hyperfine](https://github.com/sharkdp/hyperfine)
- awk
- cp
- curl
- cut
- echo
- grep
- gunzip
- mv
- tar
- tr

## Run experiment

``
cd experiment
snakemake -p -c {number_of_cpu_you_could_allow}
``

Only compilation and data generation are run in parallel.

Result are store in `workdir`:
- `memory` store memory usage of each algorithm for building interval tree
- `time` store run time of each algorithm in building, random quering, and annotation task of each algorithm
- `thread` store run time of `clairiere::Tree` and `clairiere::InterpolateTree` with different number of threads

## Generate plot

Open jupyter notebook `figure.ipynb` and run cell.
