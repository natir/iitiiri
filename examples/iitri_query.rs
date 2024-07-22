//! Example that just build an iit from bed

/* std use */
use std::io::BufRead as _;

/* crate use */
use rand::Rng as _;

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
        let ctg = split.next().unwrap().to_vec();
        let start = atoi::atoi(split.next().unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        nodes.push(iitiiri::Node::new(start, stop, ctg));
    }

    let iit: iitiiri::Iit<usize, Vec<u8>> = iitiiri::Iit::new(nodes);

    let mut rng: rand::rngs::SmallRng = rand::SeedableRng::seed_from_u64(42);

    let start_min: usize = 0;
    let start_max: usize = 200_000_000;
    let length_max = 2_000;
    let number_query = 1 << 20;

    for _ in 0..number_query {
        let start = rng.gen_range(start_min..start_max);
        let length = rng.gen_range(0..length_max);

        criterion::black_box(iit.overlap(start, start + length));
    }
}
