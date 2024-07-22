//! Example that just build an iitii from bed

/* std use */
use std::io::BufRead as _;

/* crate use */

/* project use */

const fn parse_usize(option: Option<&str>) -> usize {
    if let Some(string) = option {
        let mut res: usize = 0;
        let mut bytes = string.as_bytes();
        while let [byte, rest @ ..] = bytes {
            bytes = rest;
            if let b'0'..=b'9' = byte {
                res *= 10;
                res += (*byte - b'0') as usize;
            } else {
                panic!("not a number")
            }
        }
        res
    } else {
        8
    }
}

const DOMAIN_NUMBER: usize = parse_usize(std::option_env!("IITIIRI_DOMAIN"));

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

    let iitii: iitiiri::Iitii<usize, bool, DOMAIN_NUMBER> = iitiiri::Iitii::new(nodes);

    criterion::black_box(iitii);
}
