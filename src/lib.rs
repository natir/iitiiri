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
