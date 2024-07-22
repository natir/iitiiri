//! Example that just build an iit from bed

/* std use */
use std::io::BufRead as _;

/* crate use */

/* project use */

fn main() {
    let mut reader =
        std::io::BufReader::new(std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap());

    let mut nodes = Vec::new();
    let mut line = Vec::new();
    while let Ok(bytes) = reader.read_until(b'\n', &mut line) {
        if bytes == 0 {
            break;
        }

        let mut split = line.split(|x| x == &b' ');
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        nodes.push(iitiiri::Node::new(start, stop, true));

        line.clear();
    }

    let iit: iitiiri::Iit<usize, bool> = iitiiri::Iit::new(nodes);

    criterion::black_box(iit);
}
