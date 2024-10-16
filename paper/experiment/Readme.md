# Dependency

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

# Reproduce experiment

``
snakemake -p -c {number_of_cpu_you_could_allow}
``

Only compilation and data generation are run in parallel.

Result are store in `workdir`:
- `memory` store memory usage of each algorithm for building interval tree
- `time` store run time of each algorithm in building, random quering, and annotation task of each algorithm
- `thread` store run time of `iitri` and `iitiiri` with different number of threads

# Generate figure

Open jupyter notebook `figure.ipynb` and run cell.
