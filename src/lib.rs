//! Implicit Interval Tree with Interpolation Index Rust Implementation

/* std use */

/* crate use */

/* module declaration */
pub mod error;
pub mod estimator;
pub mod intervals;
pub mod node;

#[cfg(not(feature = "bench"))]
mod tree_utils;
#[cfg(feature = "bench")]
pub mod tree_utils;

/* project use */

/* pub use */
pub use estimator::Affine;
pub use estimator::Lazy;
pub use intervals::Intervals;
pub use node::Node;

pub type Iit<P, O> = Intervals<P, O, estimator::Lazy>;
pub type Iitii<P, O, const N: usize> = Intervals<P, O, estimator::Affine<P, N>>;

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    use std::io::BufRead;

    /* project use */
    use super::*;

    const REAL: &[u8] = b"2	ensembl_havana	lnc_RNA	15939898	15941723	.	-	.	ENST00000419083
2	ensembl_havana	exon	15939898	15940351	.	-	.	ENSE00001618069
2	ensembl_havana	exon	15940974	15941097	.	-	.	ENSE00001724273
2	ensembl_havana	exon	15941532	15941723	.	-	.	ENSE00001687475
2	havana	lnc_RNA	15940121	15941238	.	-	.	ENSG00000233718
2	havana	exon	15940121	15940351	.	-	.	ENSE00003814177
2	havana	exon	15940974	15941238	.	-	.	ENSE00003812770
";

    #[test]
    fn __real__() {
        let mut reader = std::io::Cursor::new(REAL.to_vec());
        let mut nodes = Vec::new();

        let mut line = Vec::new();
        while let Ok(bytes) = reader.read_until(b'\n', &mut line) {
            if bytes == 0 {
                break;
            }

            let mut split = line.split(|x| x == &b'\t');

            let start: usize = atoi::atoi(split.nth(3).unwrap()).unwrap();
            let stop: usize = atoi::atoi(split.next().unwrap()).unwrap();
            let object = split.nth(3).unwrap()[..15].to_vec();

            println!("{:?}", String::from_utf8(object.clone()));

            nodes.push(Node::new(start, stop, object));

            line.clear();
        }

        let iit: Iit<usize, Vec<u8>> = Iit::new(nodes.clone());
        let iitii: Iitii<usize, Vec<u8>, 2> = Iitii::new(nodes.clone());

        assert_eq!(
            iit.overlap(15_940_354, 15_940_972),
            vec![&b"ENST00000419083".to_vec(), &b"ENSG00000233718".to_vec()]
        );
        assert_eq!(
            iit.overlap(15_940_354, 15_940_972),
            iitii.overlap(15_940_354, 15_940_972)
        );
    }
}
