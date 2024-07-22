//! Declaration of estimators that determine the start of tree exploration

/* std use */

/* crate use */

/* project use */
use crate::node;
use crate::tree_utils;

pub trait Estimator<P, O> {
    /// Train estimator on node data
    fn train(data: &[node::Node<P, O>]) -> Self;

    /// Ask to estimator what is a good start for this request
    fn guess(&self, start: P, stop: P) -> usize;
}

/// A start Estimator guess only the root of tree
pub struct LazyEstimator {
    root_index: usize,
}

impl<P, O> Estimator<P, O> for LazyEstimator {
    fn train(data: &[node::Node<P, O>]) -> Self {
        Self {
            root_index: (1 << data.len().ilog2()) - 1,
        }
    }

    fn guess(&self, _start: P, _stop: P) -> usize {
        self.root_index
    }
}

// We construct useful constants for the levels of the tree to which we want to train our estimator:
// - AFFINE_TRAIN_LEVEL_LEN: just a utility const
// - AFFINE_TRAIN_LEVEL: the list of tree levels where we're going to do the training
// - AFFINE_TRAIN_LEVEL2INDEX: a lookup table that associates with a tree level its index in AFFINE_TRAIN_LEVEL or None if not present
const AFFINE_TRAIN_LEVEL_LEN: usize = 9;
const AFFINE_TRAIN_LEVEL: [usize; AFFINE_TRAIN_LEVEL_LEN] = [0, 1, 2, 4, 7, 12, 20, 33, 54];
const AFFINE_TRAIN_LEVEL2INDEX: [Option<usize>;
    AFFINE_TRAIN_LEVEL[AFFINE_TRAIN_LEVEL_LEN - 1] + 1] = {
    let mut result: [Option<usize>; AFFINE_TRAIN_LEVEL[AFFINE_TRAIN_LEVEL_LEN - 1] + 1] =
        [None; AFFINE_TRAIN_LEVEL[AFFINE_TRAIN_LEVEL_LEN - 1] + 1];

    let mut i = 0;
    while i != AFFINE_TRAIN_LEVEL[AFFINE_TRAIN_LEVEL_LEN - 1] {
        // Search train level index
        let mut train_level_index = None;
        let mut j = 0;
        while j != AFFINE_TRAIN_LEVEL_LEN {
            if i == AFFINE_TRAIN_LEVEL[j] {
                train_level_index = Some(j);
                break;
            } else if i < AFFINE_TRAIN_LEVEL[j] {
                break;
            }
            j += 1;
        }

        // Assign affine train level index of train level
        if let Some(index) = train_level_index {
            result[i] = Some(index);
        }

        i += 1;
    }

    result
};

pub struct AffineEstimator<P, const N: usize> {
    levels: [usize; N],
    a: [f64; N],
    b: [f64; N],
    min_position: P,
}

impl<P, const N: usize> AffineEstimator<P, N>
where
    P: std::default::Default
        + std::marker::Copy
        + num_traits::AsPrimitive<f64>
        + num_traits::AsPrimitive<usize>
        + std::cmp::PartialOrd,
{
    #[inline(always)]
    fn which_domain(position: &P, min_position: &P) -> usize {
        if position < min_position {
            0
        } else {
            std::cmp::min(
                N - 1,
                (<P as num_traits::AsPrimitive<usize>>::as_(*position)
                    - <P as num_traits::AsPrimitive<usize>>::as_(*min_position))
                    / N,
            )
        }
    }

    #[inline(always)]
    fn interpolate(level: usize, a: f64, b: f64, begin: P) -> usize {
        let index_in_level = a * <P as num_traits::AsPrimitive<f64>>::as_(begin) + b;

        tree_utils::index_in_level2index(level, index_in_level as usize)
    }
}

impl<P, O, const N: usize> Estimator<P, O> for AffineEstimator<P, N>
where
    P: std::default::Default
        + std::marker::Copy
        + num_traits::AsPrimitive<f64>
        + num_traits::AsPrimitive<usize>
        + std::cmp::PartialOrd,
    f64: num_traits::AsPrimitive<P>,
{
    fn train(data: &[node::Node<P, O>]) -> Self {
        let min_position = *data[0].start();
        let tree_depth = data.len().ilog2() as usize;

        let mut levels: [usize; N] = [usize::default(); N];
        let mut a: [f64; N] = [0.0; N];
        let mut b: [f64; N] = [0.0; N];

        let mut domain2level2begin_index =
            vec![vec![Vec::<(f64, f64)>::new(); AFFINE_TRAIN_LEVEL_LEN]; N];
        for index in 0..data.len() {
            if let Some(level_index) = AFFINE_TRAIN_LEVEL2INDEX[tree_utils::index2level(index)] {
                domain2level2begin_index[Self::which_domain(data[index].start(), &min_position)]
                    [level_index]
                    .push((
                        <P as num_traits::AsPrimitive<f64>>::as_(*data[index].start()),
                        tree_utils::index2index_in_level(index) as f64,
                    ))
            }
        }

        for domain in 0..N {
            let mut lowest_cost = f64::MAX;
            for level in AFFINE_TRAIN_LEVEL {
                if let Some(level_index) = AFFINE_TRAIN_LEVEL2INDEX[level] {
                    let target = &domain2level2begin_index[domain][level_index];

                    if level >= tree_depth || target.len() <= 1 {
                        continue; // maybe break
                    }

                    // If linreg failled ignore this domain level
                    if let Ok((local_a, local_b)) =
                        linreg::linear_regression_of::<f64, f64, f64>(target)
                    {
                        // calculate estimate of search cost (average over all domain points)
                        let mut cost: usize = 0;
                        for (begin, index) in target {
                            let estimate_index = Self::interpolate(
                                level,
                                local_a,
                                local_b,
                                <f64 as num_traits::AsPrimitive<P>>::as_(*begin),
                            );

                            let error = estimate_index.abs_diff(*index as usize) / (1 << level);

                            let error_penality = if error != 0 {
                                2 * (1 + error.ilog2())
                            } else {
                                0
                            };
                            let overlap_penality = if data[estimate_index].max_end()
                                > &<f64 as num_traits::AsPrimitive<P>>::as_(*begin)
                            {
                                1 + tree_depth - level / 2
                            } else {
                                0
                            };

                            cost +=
                                level + std::cmp::max(error_penality as usize, overlap_penality);
                        }
                        let avg_cost = cost as f64 / AFFINE_TRAIN_LEVEL_LEN as f64;

                        if avg_cost < tree_depth as f64 && avg_cost < lowest_cost {
                            lowest_cost = avg_cost;
                            a[domain] = local_a;
                            b[domain] = local_b;
                            levels[domain] = level;
                        }
                    }
                }
            }
        }

        AffineEstimator {
            levels,
            a,
            b,
            min_position,
        }
    }

    fn guess(&self, start: P, _stop: P) -> usize {
        let domain = Self::which_domain(&start, &self.min_position);

        Self::interpolate(self.levels[domain], self.a[domain], self.b[domain], start)
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    #[test]
    fn lazy() {
        let mut data = vec![
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true), // level 0
            node::Node::<usize, bool>::new(0, 0, false),
            node::Node::<usize, bool>::new(0, 0, false),
            node::Node::<usize, bool>::new(0, 0, false), // level 1
            node::Node::<usize, bool>::new(0, 0, false), // level 2
            node::Node::<usize, bool>::new(0, 0, false), // level 3
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true),
            node::Node::<usize, bool>::new(0, 0, true), // level 0
        ];

        let estimator = LazyEstimator::train(&data);

        assert_eq!(
            <LazyEstimator as Estimator<i32, bool>>::guess(&estimator, 0, 0),
            7
        );

        data.push(node::Node::<usize, bool>::new(0, 0, true));
        let estimator = LazyEstimator::train(&data);

        assert_eq!(
            <LazyEstimator as Estimator<i32, bool>>::guess(&estimator, 0, 0),
            15
        );
    }

    #[test]
    fn affine() {
        let data = vec![
            node::Node::new_full(100, 150, true, 150),
            node::Node::new_full(130, 200, false, 300),
            node::Node::new_full(170, 300, true, 300),
            node::Node::new_full(180, 250, false, 350),
            node::Node::new_full(200, 250, true, 250),
            node::Node::new_full(250, 350, false, 350),
            node::Node::new_full(270, 300, true, 300),
            node::Node::new_full(300, 320, false, 450),
            node::Node::new_full(350, 450, true, 450),
            node::Node::new_full(390, 420, false, 450),
        ];

        let estimator = AffineEstimator::<usize, 8>::train(&data);

        // TODO: check this result
        assert_eq!(
            <AffineEstimator<usize, 8> as Estimator<usize, bool>>::guess(&estimator, 100, 150),
            0
        )
    }
}
