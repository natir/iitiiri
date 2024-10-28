//! Example that build an interval tree from first file and use the second file to query

/* std use */
use std::io::BufRead as _;

/* crate use */

/* project use */

fn main() {
    let mut annotations =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap());
    let mut variants =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(2).unwrap()).unwrap());

    let mut nodes = Vec::new();
    let mut line = Vec::new();
    while let Ok(bytes) = annotations.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }

        let mut split = line.split(|x| x == &b' ');
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        nodes.push(clairiere::Node::new(start, stop, true));

        line.clear();
    }

    let tree: clairiere::Tree<usize, bool> = clairiere::Tree::new(nodes);

    while let Ok(bytes) = variants.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }
        let mut split = line.split(|x| x == &b' ');
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        criterion::black_box(tree.overlap(start, stop));

        line.clear();
    }
}
