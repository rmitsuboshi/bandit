#![warn(missing_docs)]
//! A crate that provides Bandit algorithms and Environments.

/// Bandit algorithms
pub mod player;
/// Environments (Stochastic/Adversarial Environment)
pub mod environment;


pub use player::Player;
pub use environment::Environment;
