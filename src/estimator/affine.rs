//! Declaration of Affine Estimator

/* std use */

/* crate use */
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/* project use */
use crate::estimator::Estimator;
use crate::node;
use crate::tree_utils;

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

/// An Estimator try guess subtree to search by build part affine function
pub struct Affine<P, const N: usize> {
    levels: [usize; N],
    a: [f64; N],
    b: [f64; N],
    min_position: P,
    domain_size: usize,
    max_index: usize,
}

impl<P, const N: usize> Affine<P, N>
where
    P: std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + num_traits::AsPrimitive<f64>
        + num_traits::AsPrimitive<usize>
        + std::cmp::PartialOrd,
    f64: num_traits::AsPrimitive<P>,
{
    #[inline(always)]
    fn which_domain(position: &P, min_position: &P, domain_size: usize) -> usize {
        if position < min_position {
            0
        } else {
            std::cmp::min(
                N - 1,
                (<P as num_traits::AsPrimitive<usize>>::as_(*position)
                    - <P as num_traits::AsPrimitive<usize>>::as_(*min_position))
                    / domain_size,
            )
        }
    }

    #[inline(always)]
    fn interpolate(level: usize, a: f64, b: f64, begin: P, max_index: usize) -> usize {
        let index_in_level = a * <P as num_traits::AsPrimitive<f64>>::as_(begin) + b;

        let estimate_index = tree_utils::index_in_level2index(level, index_in_level as usize);
        if estimate_index < max_index {
            tree_utils::index_in_level2index(level, index_in_level as usize)
        } else {
            max_index - (2 - max_index % 2)
        }
    }

    fn level2affine<O>(
        data: &[node::Node<P, O>],
        target: &[(f64, f64)],
        level: usize,
        tree_depth: usize,
    ) -> (f64, f64, f64, usize) {
        if let Ok((local_a, local_b)) = linreg::linear_regression_of::<f64, f64, f64>(target) {
            // calculate estimate of search cost (average over all domain points)
            let mut cost: usize = 0;
            for (begin, index) in target {
                let estimate_index = Self::interpolate(
                    level,
                    local_a,
                    local_b,
                    <f64 as num_traits::AsPrimitive<P>>::as_(*begin),
                    data.len(),
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

                cost += level + std::cmp::max(error_penality as usize, overlap_penality);
            }
            let avg_cost = cost as f64 / AFFINE_TRAIN_LEVEL_LEN as f64;

            if avg_cost < tree_depth as f64 {
                (avg_cost, local_a, local_b, level)
            } else {
                (avg_cost, 0.0, 0.0, level)
            }
        } else {
            (f64::MAX, 0.0, 0.0, level)
        }
    }
}

#[cfg(not(feature = "parallel"))]
impl<P, const N: usize> Affine<P, N>
where
    P: std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + num_traits::AsPrimitive<f64>
        + num_traits::AsPrimitive<usize>
        + std::cmp::PartialOrd,
    f64: num_traits::AsPrimitive<P>,
{
    fn compute_domain<O>(
        data: &[node::Node<P, O>],
        d2l2bi: &[Vec<Vec<(f64, f64)>>],
        a: &mut [f64; N],
        b: &mut [f64; N],
        levels: &mut [usize; N],
        tree_depth: usize,
    ) {
        for domain in 0..N {
            let mut lowest_cost = f64::MAX;
            for level in AFFINE_TRAIN_LEVEL {
                if let Some(level_index) = AFFINE_TRAIN_LEVEL2INDEX[level] {
                    let target = &d2l2bi[domain][level_index];

                    if level >= tree_depth || target.len() <= 1 {
                        break; // maybe break
                    }

                    let (avg_cost, local_a, local_b, level) =
                        Self::level2affine(data, target, level, tree_depth);

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
}

#[cfg(feature = "parallel")]
impl<P, const N: usize> Affine<P, N>
where
    P: std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + num_traits::AsPrimitive<f64>
        + num_traits::AsPrimitive<usize>
        + std::cmp::PartialOrd
        + std::marker::Send
        + std::marker::Sync,
    f64: num_traits::AsPrimitive<P>,
{
    fn compute_domain<O>(
        data: &[node::Node<P, O>],
        d2l2bi: &[Vec<Vec<(f64, f64)>>],
        a: &mut [f64; N],
        b: &mut [f64; N],
        levels: &mut [usize; N],
        tree_depth: usize,
    ) where
        O: std::marker::Send + std::marker::Sync,
    {
        for domain in 0..N {
            if let Some((_, local_a, local_b, level)) = AFFINE_TRAIN_LEVEL
                .par_iter()
                .filter(|&&level| AFFINE_TRAIN_LEVEL2INDEX[level].is_some())
                .map(|&level| {
                    let level_index = AFFINE_TRAIN_LEVEL2INDEX[level].unwrap();

                    let target = &d2l2bi[domain][level_index];

                    if level >= tree_depth || target.len() <= 1 {
                        (f64::MAX, 0.0, 0.0, level)
                    } else {
                        Self::level2affine(data, target, level, tree_depth)
                    }
                })
                .min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            {
                a[domain] = local_a;
                b[domain] = local_b;
                levels[domain] = level;
            } else {
                unreachable!();
            }
        }
    }
}

impl<P, O, const N: usize> Estimator<P, O> for Affine<P, N>
where
    P: std::default::Default
        + std::fmt::Debug
        + std::marker::Copy
        + num_traits::AsPrimitive<f64>
        + num_traits::AsPrimitive<usize>
        + std::cmp::PartialOrd
        + std::marker::Send
        + std::marker::Sync,
    f64: num_traits::AsPrimitive<P>,
    O: std::marker::Send + std::marker::Sync,
{
    fn train(data: &[node::Node<P, O>]) -> Self {
        let min_position = *data[0].start();
        let domain_size = 1
            + (<P as num_traits::AsPrimitive<usize>>::as_(*data[data.len() - 1].start())
                - <P as num_traits::AsPrimitive<usize>>::as_(min_position))
                / N;
        let tree_depth = data.len().ilog2() as usize;

        let mut levels: [usize; N] = [usize::default(); N];
        let mut a: [f64; N] = [0.0; N];
        let mut b: [f64; N] = [0.0; N];

        let mut domain2level2begin_index = vec![
            (0..AFFINE_TRAIN_LEVEL_LEN)
                .map(|_| Vec::with_capacity(data.len() / N))
                .collect::<Vec<_>>();
            N
        ];
        for index in 0..data.len() {
            if let Some(level_index) = AFFINE_TRAIN_LEVEL2INDEX[tree_utils::index2level(index)] {
                domain2level2begin_index
                    [Self::which_domain(data[index].start(), &min_position, domain_size)]
                    [level_index]
                    .push((
                        <P as num_traits::AsPrimitive<f64>>::as_(*data[index].start()),
                        tree_utils::index2index_in_level(index) as f64,
                    ))
            }
        }

        Self::compute_domain(
            data,
            &domain2level2begin_index,
            &mut a,
            &mut b,
            &mut levels,
            tree_depth,
        );

        Affine {
            levels,
            a,
            b,
            min_position,
            domain_size,
            max_index: data.len(),
        }
    }

    fn guess(&self, start: P, _stop: P) -> usize {
        let domain = Self::which_domain(&start, &self.min_position, self.domain_size);

        Self::interpolate(
            self.levels[domain],
            self.a[domain],
            self.b[domain],
            start,
            self.max_index,
        )
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    #[test]
    fn affine() {
        let mut data = Vec::new();

        for i in (50..10_000).step_by(50) {
            data.push(node::Node::new_full(i, i + 50, true, i));
        }

        let truth = vec![
            15, 15, 15, 15, 15, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 9, 8,
        ];

        seq_macro::seq!(N in 1..32 {
        let estimator = Affine::<usize, N>::train(&data);

        assert_eq!(
            <Affine<usize, N> as Estimator<usize, bool>>::guess(&estimator, 500, 150),
            truth[N],
            "estimator::affine check N = {}", N
        );
        });
    }
}
