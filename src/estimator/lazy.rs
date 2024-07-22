//! Declaration of Lazy Estimator

/* std use */

/* crate use */
use crate::node;

/* project use */
use crate::estimator::Estimator;

/// A lazy Estimator guess only the root of tree
pub struct Lazy {
    root_index: usize,
}

impl<P, O> Estimator<P, O> for Lazy {
    fn train(data: &[node::Node<P, O>]) -> Self {
        Self {
            root_index: (1 << data.len().ilog2()) - 1,
        }
    }

    fn guess(&self, _start: P, _stop: P) -> usize {
        self.root_index
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

        let estimator = Lazy::train(&data);

        assert_eq!(<Lazy as Estimator<i32, bool>>::guess(&estimator, 0, 0), 7);

        data.push(node::Node::<usize, bool>::new(0, 0, true));
        let estimator = Lazy::train(&data);

        assert_eq!(<Lazy as Estimator<i32, bool>>::guess(&estimator, 0, 0), 15);
    }
}
