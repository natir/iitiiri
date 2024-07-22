//! Implicit Interval Tree with Interpolation Index Rust Implementation

/* std use */

/* crate use */

/* module declaration */
pub mod error;
pub mod estimator;
pub mod intervals;
pub mod node;

mod tree_utils;

/* project use */

/* pub use */
pub use estimator::AffineEstimator;
pub use estimator::LazyEstimator;
pub use intervals::Intervals;
pub use node::Node;

pub use intervals::Iit;
pub use intervals::Iitii;
