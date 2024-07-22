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
        let start = atoi::atoi(split.nth(1).unwrap()).unwrap();
        let stop = atoi::atoi(split.next().unwrap()).unwrap();

        nodes.push(iitiiri::Node::new(start, stop, true));

        line.clear();
    }

    let iit: iitiiri::Iit<usize, bool> = iitiiri::Iit::new(nodes);

    let mut rng: rand::rngs::SmallRng = rand::SeedableRng::seed_from_u64(42);

    let start_min: usize = 0;
    let start_max: usize = 200_000_000;
    let length_max = 2_000;
    let number_query = 1 << 9;

    for i in 0..number_query {
        let start = rng.gen_range(start_min..start_max);
        let length = rng.gen_range(0..length_max);

        let now = std::time::Instant::now();
        for _ in 0..100 {
            criterion::black_box(iit.overlap(start, start + length));
        }
        #[cfg(not(feature = "parallel"))]
        println!("iitri,{},{}", i, now.elapsed().as_nanos());
        #[cfg(feature = "parallel")]
        println!("iitri_parallel,{},{}", i, now.elapsed().as_nanos());
    }
}
