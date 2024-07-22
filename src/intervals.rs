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
    /// Return object of overlap query
    pub fn overlap(&self, start: P, stop: P) -> Vec<&O> {
        let mut result = Vec::new();
        let root_index = (1 << self.nodes.len().ilog2()) - 1;

        let mut subtree_index = self.estimator.guess(start, stop);
        while subtree_index != root_index {
            if subtree_index > self.nodes.len() {
                subtree_index = tree_utils::parent(subtree_index);
                continue;
            }

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
        // Imaginary node if left exist explore it
        if subtree >= self.nodes.len() && level > 0 {
            if let Some(left_index) = tree_utils::left(subtree) {
                self.scan(left_index, level - 1, start, stop, result);
            }
            return;
        }

        // Low level stop recursion
        if level <= 2 {
            let level_move = (1 << level) - 1;
            let left_most = subtree.saturating_sub(level_move);
            let right_most = std::cmp::min(subtree + level_move, self.nodes.len() - 1);

            for index in left_most..=right_most {
                if self.nodes[index].start() >= &stop {
                    return;
                }
                if self.nodes[index].stop() > &start {
                    result.push(self.nodes[index].object())
                }
            }
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
}

#[cfg(feature = "parallel")]
impl<P, O, E> Intervals<P, O, E>
where
    P: num_traits::PrimInt
        + std::default::Default
        + std::fmt::Debug
        + std::marker::Send
        + std::marker::Sync,
    O: std::fmt::Debug + std::marker::Send,
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
        let tree_depth = self.nodes.len().ilog2() as usize;

        self.nodes
            .par_iter_mut()
            .enumerate()
            .rev()
            .filter(|pair| tree_utils::index2level(pair.0) == 0)
            .for_each(|(_, node)| node.set_max_end(*node.stop()));

        for level in 1..=tree_depth {
            let right = self
                .nodes
                .par_iter_mut()
                .map(|node| *node.max_end())
                .collect::<Vec<P>>();

            self.nodes
                .par_iter_mut()
                .enumerate()
                .rev()
                .filter(|pair| tree_utils::index2level(pair.0) == level)
                .map(|pair| (pair.0, right.get(pair.0 + (1 << (level - 1))), pair.1))
                .for_each(|(_, option_right, node)| {
                    if let Some(max_end) = option_right {
                        node.set_max_end(std::cmp::max(*node.stop(), *max_end))
                    }
                });
        }
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
                &(250, 350),
                &(270, 300),
                &(300, 320)
            ]
        );
    }

    #[test]
    fn overlap_affine() {
        let mut data = Vec::new();

        for i in (50..1000).step_by(50) {
            data.push(node::Node::new_full(i, i + 50, (i, i + 50), i));
        }

        let intervals = Iitii::<usize, (usize, usize), 16>::new(data);

        assert_eq!(
            intervals.overlap(250, 500),
            vec![
                &(250, 300),
                &(300, 350),
                &(350, 400),
                &(400, 450),
                &(450, 500)
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

    #[test]
    fn iit_equal_iitii_entropy_seed() {
        let mut rng = rand::rngs::StdRng::from_entropy();

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

        assert_eq!(lazy.overlap(a, b), affine.overlap(a, b))
    }
}
