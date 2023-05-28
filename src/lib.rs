#![warn(missing_docs)]
//! A crate that provides Bandit algorithms and Environments.

/// Bandit algorithms
pub mod player;
/// Environments (Stochastic/Adversarial Environment)
pub mod environment;

/// Provides the bandit protocol.
pub mod protocol;


pub use player::Player;
pub use environment::Environment;

pub use player::{
    EtcBuilder,
    Etc,
};


pub use environment::{
    SubGaussianBuilder,
    SubGaussian,
};


pub use protocol::run;
