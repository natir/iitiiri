//! Declaration of intervals container

/* std use */

/* crate use */
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/* project use */
use crate::estimator;
use crate::node;
use crate::tree_utils;

/// Intervals struct
pub struct Intervals<P, O, E> {
    nodes: Vec<node::Node<P, O>>,
    estimator: E,
}

#[cfg(not(feature = "parallel"))]
impl<P, O, E> std::iter::FromIterator<(core::ops::Range<P>, O)> for Intervals<P, O, E>
where
    P: num_traits::PrimInt + std::default::Default + std::fmt::Debug,
    O: std::fmt::Debug,
    E: estimator::Estimator<P, O>,
    P: std::marker::Send,
    O: std::marker::Send,
{
    /// Build a Intervals struct from an Iterator
    fn from_iter<T: IntoIterator<Item = (core::ops::Range<P>, O)>>(iter: T) -> Self {
        Intervals::new(
            iter.into_iter()
                .map(|x| node::Node::new(x.0.start, x.0.end, x.1))
                .collect(),
        )
    }
}

#[cfg(feature = "parallel")]
impl<P, O, E> std::iter::FromIterator<(core::ops::Range<P>, O)> for Intervals<P, O, E>
where
    P: num_traits::PrimInt
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Send
        + std::marker::Sync,
    O: std::fmt::Debug + std::marker::Send + std::marker::Sync,
    E: estimator::Estimator<P, O>,
{
    /// Build a Intervals struct from an Iterator
    fn from_iter<T: IntoIterator<Item = (core::ops::Range<P>, O)>>(iter: T) -> Self {
        Intervals::new(
            iter.into_iter()
                .map(|x| node::Node::new(x.0.start, x.0.end, x.1))
                .collect(),
        )
    }
}

impl<P, O, E> Intervals<P, O, E>
where
    P: num_traits::PrimInt + std::default::Default + std::fmt::Debug,
    O: std::fmt::Debug,
    E: estimator::Estimator<P, O>,
    P: std::marker::Send,
    O: std::marker::Send,
{
    /// Return object of overlap query
    pub fn overlap(&self, start: P, stop: P) -> Vec<&O> {
        let mut result = Vec::new();
        let root_index = (1 << self.nodes.len().ilog2()) - 1;

        let mut subtree_index = self.estimator.guess(start, stop);
        dbg!(subtree_index);
        while subtree_index != root_index {
            if subtree_index > self.nodes.len() {
                subtree_index = tree_utils::parent(subtree_index);
                continue;
            }

            dbg!(
                start,
                stop,
                self.nodes[subtree_index].max_end(),
                self.min_beg(subtree_index)
            );
            if stop < *self.nodes[subtree_index].max_end()
                && self
                    .min_beg(subtree_index)
                    .map(|x| &start > x)
                    .unwrap_or(false)
            {
                break;
            }

            subtree_index = tree_utils::parent(subtree_index);
        }
        if subtree_index != root_index {
            subtree_index = tree_utils::parent(subtree_index);
        }
        dbg!(subtree_index);

        self.scan(
            subtree_index,
            tree_utils::index2level(subtree_index),
            start,
            stop,
            &mut result,
        );

        result
    }

    fn scan<'a>(
        &'a self,
        subtree: usize,
        level: usize,
        start: P,
        stop: P,
        result: &mut Vec<&'a O>,
    ) {
        dbg!("SCAN CALL", subtree);
        dbg!(start);
        dbg!(stop);
        // Imaginary node if left exist explore it
        if subtree >= self.nodes.len() && level > 0 {
            if let Some(left_index) = tree_utils::left(subtree) {
                dbg!("IMAGINARY RECURSION");
                self.scan(left_index, level - 1, start, stop, result);
            }
            return;
        } else if level <= 2 {
            // Low level stop recursion
            dbg!("LOW LEVEL END");
            let level_move = (1 << level) - 1;
            let left_most = subtree.saturating_sub(level_move);
            let right_most = std::cmp::min(subtree + level_move, self.nodes.len() - 1);
            dbg!(subtree, left_most, right_most);
            for index in left_most..=right_most {
                dbg!(index);
                dbg!(self.nodes[index].start());
                dbg!(stop);
                if self.nodes[index].start() >= &stop {
                    break;
                }
                if self.nodes[index].stop() > &start {
                    dbg!("LOW LEVEL PUSH", self.nodes[subtree].object());
                    result.push(self.nodes[index].object())
                }
            }
            dbg!("LOW LEVEL RETURN");
            return;
        }

        if *self.nodes[subtree].max_end() > start {
            // subtree_max_end > qbeg
            let local_level = level - 1;

            if let Some(left_index) = tree_utils::left(subtree) {
                self.scan(left_index, local_level, start, stop, result);
            }

            if self.nodes[subtree].start() < &stop {
                if self.nodes[subtree].stop() > &start {
                    dbg!("RECURSION PUSH", self.nodes[subtree].object());
                    result.push(self.nodes[subtree].object());
                }

                if let Some(right_index) = tree_utils::right(subtree) {
                    dbg!("RIGHT RECURSION");
                    self.scan(right_index, local_level, start, stop, result);
                }
            }
        }
    }

    #[inline(always)]
    fn min_beg(&self, subtree: usize) -> Option<&P> {
        Some(self.nodes[tree_utils::leftmost_leaf(subtree)].start())
    }

    #[cfg(test)]
    pub fn get_nodes(&self) -> &[node::Node<P, O>] {
        &self.nodes
    }
}

#[cfg(not(feature = "parallel"))]
impl<P, O, E> Intervals<P, O, E>
where
    P: num_traits::PrimInt + std::default::Default + std::fmt::Debug,
    O: std::fmt::Debug,
    E: estimator::Estimator<P, O>,
    P: std::marker::Send,
    O: std::marker::Send,
{
    /// Create a new Intervals struct from a list of node
    pub fn new(mut nodes: Vec<node::Node<P, O>>) -> Self {
        // Sort node
        nodes.sort();

        let mut obj = Intervals {
            estimator: E::train(&nodes),
            nodes,
        };

        if !obj.nodes.is_empty() {
            obj.compute_max_end()
        }

        obj
    }

    fn compute_max_end(&mut self) {
        let index_end = self.nodes.len();
        let tree_depth = index_end.ilog2() as usize;

        for level in 1..=tree_depth {
            let x = 1 << (level - 1);
            let index_0 = (x << 1) - 1;
            let step = x << 2;

            for index in (index_0..index_end).step_by(step) {
                let left = self
                    .nodes
                    .get(tree_utils::left_uncheck(index, level))
                    .map(node::Node::max_end);
                let right = self
                    .nodes
                    .get(tree_utils::right_uncheck(index, level))
                    .map(node::Node::max_end);

                let child = match (left, right) {
                    (Some(l), Some(r)) => *std::cmp::max(l, r),
                    (None, Some(r)) => *r,
                    (Some(l), None) => *l,
                    _ => num_traits::identities::zero(),
                };

                let node = *self.nodes[index].max_end();
                self.nodes[index].set_max_end(std::cmp::max(child, node));
            }
        }
    }
}

#[cfg(feature = "parallel")]
impl<P, O, E> Intervals<P, O, E>
where
    P: num_traits::PrimInt
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Send
        + std::marker::Sync,
    O: std::fmt::Debug + std::marker::Send + std::marker::Sync,
    E: estimator::Estimator<P, O>,
{
    /// Create a new Intervals struct from a list of node
    pub fn new(mut nodes: Vec<node::Node<P, O>>) -> Self {
        // Sort node
        nodes.par_sort();

        let mut obj = Intervals {
            estimator: E::train(&nodes),
            nodes,
        };

        if !obj.nodes.is_empty() {
            obj.compute_max_end();
        }

        obj
    }

    fn compute_max_end(&mut self) {
        let index_end = self.nodes.len();
        let tree_depth = index_end.ilog2() as usize;

        for level in 1..=tree_depth {
            let x = 1 << (level - 1);
            let index_0 = (x << 1) - 1;
            let step = x << 2;

            let index2max: Vec<(usize, P)> = (index_0..index_end)
                .into_par_iter()
                .step_by(step)
                .map(|index| {
                    let left = self
                        .nodes
                        .get(tree_utils::left_uncheck(index, level))
                        .map(node::Node::max_end);
                    let right = self
                        .nodes
                        .get(tree_utils::right_uncheck(index, level))
                        .map(node::Node::max_end);
                    let local = *self.nodes[index].max_end();

                    let child = match (left, right) {
                        (Some(l), Some(r)) => *std::cmp::max(l, r),
                        (None, Some(r)) => *r,
                        (Some(l), None) => *l,
                        _ => num_traits::identities::zero(),
                    };
                    (index, std::cmp::max(child, local))
                })
                .collect();

            index2max.iter().for_each(|(index, max)| {
                self.nodes[*index].set_max_end(*max);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */
    use rand::Rng as _;
    use rand::RngCore as _;
    use rand::SeedableRng as _;

    /* project use */
    use super::*;

    #[test]
    fn setup() {
        let intervals = Intervals::<usize, bool, estimator::Lazy>::new(vec![
            node::Node::<usize, bool>::new(100, 150, true),
            node::Node::<usize, bool>::new(170, 300, true),
            node::Node::<usize, bool>::new(200, 250, true),
            node::Node::<usize, bool>::new(270, 300, true),
            node::Node::<usize, bool>::new(350, 450, true), // level 0
            node::Node::<usize, bool>::new(130, 200, false),
            node::Node::<usize, bool>::new(250, 350, false),
            node::Node::<usize, bool>::new(390, 420, false), // level 1
            node::Node::<usize, bool>::new(180, 250, false), // level 2
            node::Node::<usize, bool>::new(300, 320, false), // level 3
        ]);

        assert_eq!(
            intervals.get_nodes(),
            vec![
                node::Node::new(100, 150, true),
                node::Node::new(130, 200, false),
                node::Node::new(170, 300, true),
                node::Node::new(180, 250, false),
                node::Node::new(200, 250, true),
                node::Node::new(250, 350, false),
                node::Node::new(270, 300, true),
                node::Node::new(300, 320, false),
                node::Node::new(350, 450, true),
                node::Node::new(390, 420, false)
            ]
        );

        assert_eq!(
            intervals
                .get_nodes()
                .iter()
                .map(node::Node::object)
                .cloned()
                .collect::<Vec<bool>>(),
            vec![true, false, true, false, true, false, true, false, true, false]
        );

        assert_eq!(
            intervals
                .get_nodes()
                .iter()
                .map(node::Node::max_end)
                .cloned()
                .collect::<Vec<usize>>(),
            vec![150, 300, 300, 350, 250, 350, 300, 350, 450, 450]
        );
    }

    #[test]
    fn very_large() {
        let intervals = Intervals::<usize, bool, estimator::Lazy>::new(vec![
            node::Node::<usize, bool>::new(100, 2000, true),
            node::Node::<usize, bool>::new(170, 300, true),
            node::Node::<usize, bool>::new(200, 250, true),
            node::Node::<usize, bool>::new(270, 300, true),
            node::Node::<usize, bool>::new(350, 450, true), // level 0
            node::Node::<usize, bool>::new(130, 200, false),
            node::Node::<usize, bool>::new(250, 350, false),
            node::Node::<usize, bool>::new(390, 420, false), // level 1
            node::Node::<usize, bool>::new(180, 250, false), // level 2
            node::Node::<usize, bool>::new(300, 320, false), // level 3
        ]);

        assert_eq!(
            intervals.get_nodes(),
            vec![
                node::Node::new(100, 2000, true),
                node::Node::new(130, 200, false),
                node::Node::new(170, 300, true),
                node::Node::new(180, 250, false),
                node::Node::new(200, 250, true),
                node::Node::new(250, 350, false),
                node::Node::new(270, 300, true),
                node::Node::new(300, 320, false),
                node::Node::new(350, 450, true),
                node::Node::new(390, 420, false)
            ]
        );

        assert_eq!(
            intervals
                .get_nodes()
                .iter()
                .map(node::Node::object)
                .cloned()
                .collect::<Vec<bool>>(),
            vec![true, false, true, false, true, false, true, false, true, false]
        );

        assert_eq!(
            intervals
                .get_nodes()
                .iter()
                .map(node::Node::max_end)
                .cloned()
                .collect::<Vec<usize>>(),
            vec![2000, 2000, 300, 2000, 250, 350, 300, 2000, 450, 450]
        );
    }

    #[test]
    fn overlap_lazy() {
        let intervals = Intervals::<usize, (usize, usize), estimator::Lazy>::from_iter(vec![
            (100..150, (100, 150)),
            (170..300, (170, 300)),
            (200..250, (200, 250)),
            (270..300, (270, 300)),
            (350..450, (350, 450)), // level 0
            (130..200, (130, 200)),
            (250..350, (250, 350)),
            (390..420, (390, 420)), // level 1
            (180..250, (180, 250)), // level 2
            (300..320, (300, 320)), // level 3
        ]);

        assert_eq!(
            intervals.overlap(120, 180),
            vec![&(100, 150), &(130, 200), &(170, 300),]
        );

        assert_eq!(
            intervals.overlap(120, 350),
            vec![
                &(100, 150),
                &(130, 200),
                &(170, 300),
                &(180, 250),
                &(200, 250),
                &(250, 350),
                &(270, 300),
                &(300, 320)
            ]
        );
    }

    fn _overlap_affine<const N: usize>() {
        let mut data = Vec::new();

        for i in (50..1000).step_by(50) {
            data.push(node::Node::new_full(i, i + 50, (i, i + 50), i));
        }

        let intervals = Intervals::<usize, (usize, usize), estimator::Affine<usize, N>>::new(data);

        assert_eq!(
            intervals.overlap(250, 500),
            vec![
                &(250, 300),
                &(300, 350),
                &(350, 400),
                &(400, 450),
                &(450, 500)
            ],
            "intervals overlap_affine check N = {}",
            N
        );
    }

    #[test]
    fn overlap_affine() {
        seq_macro::seq!(I in 1..128 {
            _overlap_affine::<I>();
        });
    }

    #[test]
    fn iit_equal_iitii() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        let pos_range = 0..100_000;
        let range_len = 0..200;
        let num_interval = 500;

        let nodes = (0..num_interval)
            .map(|_| {
                let a = rng.gen_range(pos_range.clone());
                let b = a + rng.gen_range(range_len.clone());

                node::Node::new(a, b, (a, b))
            })
            .collect::<Vec<node::Node<usize, (usize, usize)>>>();

        let lazy = Intervals::<usize, (usize, usize), estimator::Lazy>::new(nodes.clone());
        let affine = Intervals::<usize, (usize, usize), estimator::Affine<usize, 16>>::new(nodes);

        let a = rng.gen_range(pos_range.clone());
        let b = a + rng.gen_range(0..2000);

        assert_eq!(
            lazy.overlap(a, b),
            vec![
                &(76496, 76689),
                &(76577, 76708),
                &(76787, 76903),
                &(76885, 77047),
                &(76942, 77039),
                &(77208, 77235),
                &(77427, 77599),
                &(77527, 77668),
                &(77536, 77627),
                &(77616, 77780),
                &(77712, 77778)
            ]
        );

        assert_eq!(
            affine.overlap(a, b),
            vec![
                &(76496, 76689),
                &(76577, 76708),
                &(76787, 76903),
                &(76885, 77047),
                &(76942, 77039),
                &(77208, 77235),
                &(77427, 77599),
                &(77527, 77668),
                &(77536, 77627),
                &(77616, 77780),
                &(77712, 77778)
            ]
        );
    }

    fn _iit_equal_iitii_entropy_seed<const N: usize>() {
        let mut rng = rand::rngs::StdRng::from_entropy();
        let seed = rng.next_u64();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let pos_range = 0..100_000;
        let range_len = 0..200;
        let num_interval = 500;

        let nodes = (0..num_interval)
            .map(|_| {
                let a = rng.gen_range(pos_range.clone());
                let b = a + rng.gen_range(range_len.clone());

                node::Node::new(a, b, (a, b))
            })
            .collect::<Vec<node::Node<usize, (usize, usize)>>>();

        let lazy = Intervals::<usize, (usize, usize), estimator::Lazy>::new(nodes.clone());
        let affine = Intervals::<usize, (usize, usize), estimator::Affine<usize, 16>>::new(nodes);

        let a = rng.gen_range(pos_range.clone());
        let b = a + rng.gen_range(0..2000);

        assert_eq!(
            lazy.overlap(a, b),
            affine.overlap(a, b),
            "interval iit_equal_iitii_entropy_seed seed: {} N: {}",
            seed,
            N,
        )
    }

    #[test]
    fn iit_equal_iitii_entropy_seed() {
        seq_macro::seq!(I in 1..128 {
            _iit_equal_iitii_entropy_seed::<I>();
        });
    }

    #[test]
    fn error() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(11497814493348450862);

        let pos_range = 0..100_000;
        let range_len = 0..200;
        let num_interval = 500;

        let nodes = (0..num_interval)
            .map(|_| {
                let a = rng.gen_range(pos_range.clone());
                let b = a + rng.gen_range(range_len.clone());

                node::Node::new(a, b, (a, b))
            })
            .collect::<Vec<node::Node<usize, (usize, usize)>>>();

        let lazy = Intervals::<usize, (usize, usize), estimator::Lazy>::new(nodes.clone());
        let affine = Intervals::<usize, (usize, usize), estimator::Affine<usize, 16>>::new(nodes);

        let a = rng.gen_range(pos_range.clone());
        let b = a + rng.gen_range(0..2000);

        dbg!("LAZY");
        assert_eq!(
            lazy.overlap(a, b),
            [
                &(23513, 23706),
                &(23873, 24018),
                &(23883, 24063),
                &(23892, 23908)
            ]
        );
        println!("\n\n\n");
        dbg!("AFFINE");
        assert_eq!(
            affine.overlap(a, b),
            [
                &(23513, 23706),
                &(23873, 24018),
                &(23883, 24063),
                &(23892, 23908)
            ]
        );
    }
}
