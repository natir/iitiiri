//! Error definition of iitiiri

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {}

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
