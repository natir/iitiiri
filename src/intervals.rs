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
    tree_depth: usize,
    estimator: E,
}

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

impl<P, O, E> Intervals<P, O, E>
where
    P: num_traits::PrimInt + std::default::Default + std::fmt::Debug,
    O: std::fmt::Debug,
    E: estimator::Estimator<P, O>,
    P: std::marker::Send,
    O: std::marker::Send,
{
    /// Create a new Intervals struct from a list of node
    pub fn new(nodes: Vec<node::Node<P, O>>) -> Self {
        let tree_depth = nodes.len().ilog2() as usize;

        let mut obj = Intervals {
            estimator: E::train(&nodes),
            nodes,
            tree_depth,
        };

        if !obj.nodes.is_empty() {
            obj.setup();
        }

        obj
    }

    /// Return object of  overlap query
    pub fn overlap(&self, start: P, stop: P) -> Vec<&O> {
        let mut result = Vec::new();
        let root_index = (1 << self.nodes.len().ilog2()) - 1;

        let mut subtree_index = self.estimator.guess(start, stop);

        if subtree_index != root_index {
            while subtree_index != root_index
                && (subtree_index >= self.nodes.len()
                    || start < *self.nodes[subtree_index].max_end()
                    || self
                        .min_beg(subtree_index)
                        .map(|x| x < &stop)
                        .unwrap_or(false))
            {
                subtree_index = tree_utils::parent(subtree_index);
            }
        }

        self.scan(subtree_index, self.tree_depth, start, stop, &mut result);

        result
    }

    fn setup(&mut self) {
        // Sort node
        #[cfg(not(feature = "parallel"))]
        self.nodes.sort();
        #[cfg(feature = "parallel")]
        self.nodes.par_sort();

        self.compute_max_end();
    }

    fn compute_max_end(&mut self) {
        let max_index = self.nodes.len();

        // Index node
        for index in (0..self.nodes.len()).rev() {
            let level = tree_utils::index2level(index);

            if level == 0 {
                continue;
            }

            if let Some(right_index) = tree_utils::right(index) {
                if right_index < max_index {
                    let new_max = std::cmp::max(
                        *self.nodes[index].max_end(),
                        *self.nodes[right_index].max_end(),
                    );
                    self.nodes[index].set_max_end(new_max);
                    continue;
                } else if let Some(right_left_index) = tree_utils::left(right_index) {
                    if right_left_index < max_index {
                        let new_max = std::cmp::max(
                            *self.nodes[index].max_end(),
                            *self.nodes[right_left_index].max_end(),
                        );
                        self.nodes[index].set_max_end(new_max);
                        continue;
                    }
                }
            }

            if let Some(left_index) = tree_utils::left(index) {
                let new_max = std::cmp::max(
                    *self.nodes[index].max_end(),
                    *self.nodes[left_index].max_end(),
                );
                self.nodes[index].set_max_end(new_max)
            }
        }
    }

    fn scan<'a>(
        &'a self,
        subtree: usize,
        level: usize,
        start: P,
        stop: P,
        result: &mut Vec<&'a O>,
    ) {
        if subtree > self.nodes.len() && level > 0 {
            if let Some(left_index) = tree_utils::left(subtree) {
                self.scan(left_index, level - 1, start, stop, result);
                return;
            }
        }

        if level <= 2 {
            let left_most = subtree.saturating_sub(3);
            let right_most = std::cmp::min(subtree + 3, self.nodes.len());

            for index in left_most..right_most {
                if self.nodes[index].start() >= &stop {
                    return;
                }
                if self.nodes[index].stop() > &start {
                    result.push(self.nodes[index].object())
                }
            }
            return;
        }

        if start < *self.nodes[subtree].max_end() {
            let local_level = level - 1;

            if let Some(left_index) = tree_utils::left(subtree) {
                self.scan(left_index, local_level, start, stop, result);
            }
            if self.nodes[subtree].start() < &start {
                if self.nodes[subtree].stop() > &stop {
                    result.push(self.nodes[subtree].object());
                }

                if let Some(right_index) = tree_utils::right(subtree) {
                    self.scan(right_index, local_level, start, stop, result);
                }
            }
        }
    }

    #[inline(always)]
    fn min_beg(&self, subtree: usize) -> Option<&P> {
        let right = tree_utils::rightmost_leaf(subtree);
        let left = tree_utils::leftmost_leaf(subtree);
        if left > 0 && self.nodes[left - 1].start() == self.nodes[subtree].start() {
            Some(self.nodes[subtree].start())
        } else if right < self.nodes.len() - 1 {
            Some(self.nodes[right + 1].start())
        } else {
            None
        }
    }

    #[cfg(test)]
    pub fn get_nodes(&self) -> &[node::Node<P, O>] {
        &self.nodes
    }
}

pub type Iit<P, O> = Intervals<P, O, estimator::LazyEstimator>;
pub type Iitii<P, O, const N: usize> = Intervals<P, O, estimator::AffineEstimator<P, N>>;

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */
    use rand::Rng as _;
    use rand::SeedableRng as _;

    /* project use */
    use super::*;

    #[test]
    fn setup() {
        let intervals = Iit::<usize, bool>::new(vec![
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
                node::Node::new_full(100, 150, true, 150),
                node::Node::new_full(130, 200, false, 300),
                node::Node::new_full(170, 300, true, 300),
                node::Node::new_full(180, 250, false, 350),
                node::Node::new_full(200, 250, true, 250),
                node::Node::new_full(250, 350, false, 350),
                node::Node::new_full(270, 300, true, 300),
                node::Node::new_full(300, 320, false, 450),
                node::Node::new_full(350, 450, true, 450),
                node::Node::new_full(390, 420, false, 450)
            ]
        );
    }

    #[test]
    fn overlap_lazy() {
        let intervals = Iit::<usize, (usize, usize)>::from_iter(vec![
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
                &(250, 350)
            ]
        );
    }

    #[test]
    fn overlap_affine() {
        let intervals = Iitii::<usize, (usize, usize), 16>::new(vec![
            node::Node::<usize, (usize, usize)>::new(100, 150, (100, 150)),
            node::Node::<usize, (usize, usize)>::new(170, 300, (170, 300)),
            node::Node::<usize, (usize, usize)>::new(200, 250, (200, 250)),
            node::Node::<usize, (usize, usize)>::new(270, 300, (270, 300)),
            node::Node::<usize, (usize, usize)>::new(350, 450, (350, 450)), // level 0
            node::Node::<usize, (usize, usize)>::new(130, 200, (130, 200)),
            node::Node::<usize, (usize, usize)>::new(250, 350, (250, 350)),
            node::Node::<usize, (usize, usize)>::new(390, 420, (390, 420)), // level 1
            node::Node::<usize, (usize, usize)>::new(180, 250, (180, 250)), // level 2
            node::Node::<usize, (usize, usize)>::new(300, 320, (300, 320)), // level 3
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
                &(250, 350)
            ]
        );
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

        let lazy = Iit::<usize, (usize, usize)>::new(nodes.clone());
        let affine = Iitii::<usize, (usize, usize), 16>::new(nodes);

        let a = rng.gen_range(pos_range.clone());
        let b = a + rng.gen_range(0..2000);

        assert_eq!(
            lazy.overlap(a, b),
            vec![
                &(76496, 76689),
                &(76577, 76708),
                &(76787, 76903),
                &(76885, 77047),
                &(76942, 77039)
            ]
        );

        assert_eq!(
            affine.overlap(a, b),
            vec![
                &(76496, 76689),
                &(76577, 76708),
                &(76787, 76903),
                &(76885, 77047),
                &(76942, 77039)
            ]
        );

        let mut rng = rand::rngs::StdRng::from_entropy();

        let nodes = (0..num_interval)
            .map(|_| {
                let a = rng.gen_range(pos_range.clone());
                let b = a + rng.gen_range(range_len.clone());

                node::Node::new(a, b, (a, b))
            })
            .collect::<Vec<node::Node<usize, (usize, usize)>>>();

        let lazy = Iit::<usize, (usize, usize)>::new(nodes.clone());
        let affine = Iitii::<usize, (usize, usize), 16>::new(nodes);

        let a = rng.gen_range(pos_range.clone());
        let b = a + rng.gen_range(0..2000);

        assert_eq!(lazy.overlap(a, b), affine.overlap(a, b))
    }
}
