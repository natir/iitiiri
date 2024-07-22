//! Rust-bio tree building

/* std use */
use std::io::BufRead as _;

/* crate use */

/* project use */

fn main() {
    let mut reader =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap());

    let mut tree = bio::data_structures::interval_tree::ArrayBackedIntervalTree::new();
    let mut line = Vec::with_capacity(1024);
    while let Ok(bytes) = reader.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }

        let mut split = line.split(|x| x == &b' ');
        let start: usize = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop: usize = atoi::atoi(split.next().unwrap()).unwrap();

        tree.insert(start..stop, true);

        line.clear();
    }

    tree.index();

    criterion::black_box(tree);
}
