//! Declaration of estimators that determine the start of tree exploration

/* std use */

/* crate use */

/* module declaration */
mod affine;
mod lazy;

/* project use */
use crate::node;

/* pub use */
pub use affine::Affine;
pub use lazy::Lazy;

pub trait Estimator<P, O> {
    /// Train estimator on node data
    fn train(data: &[node::Node<P, O>]) -> Self;

    /// Ask to estimator what is a good start for this request
    fn guess(&self, start: P, stop: P, data: &[node::Node<P, O>]) -> usize;
}
