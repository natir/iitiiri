//! COItrees build an interval tree from first file and use the second file to query

/* std use */
use std::io::BufRead as _;

/* crate use */
use coitrees::IntervalTree as _;

/* project use */

fn main() {
    let mut annotations =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap());
    let mut variants =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(2).unwrap()).unwrap());

    let mut nodes = Vec::new();
    let mut line = Vec::with_capacity(1024);
    while let Ok(bytes) = annotations.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }

        let mut split = line.split(|x| x == &b' ');
        let start: usize = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop: usize = atoi::atoi(split.next().unwrap()).unwrap();

        nodes.push(coitrees::Interval::new(start as i32, stop as i32, true));

        line.clear();
    }

    let tree: coitrees::COITree<bool, usize> = coitrees::COITree::new(&nodes);

    while let Ok(bytes) = variants.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }
        let mut split = line.split(|x| x == &b' ');
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        let mut result: Vec<bool> = Vec::with_capacity(1024);
        tree.query(start, stop, |n| result.push(n.metadata));
        criterion::black_box(result);

        line.clear();
    }
}
