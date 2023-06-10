#![warn(missing_docs)]
//! A crate that provides Bandit algorithms and Environments.

/// Bandit algorithms
pub mod player;
/// Environments (Stochastic/Adversarial Environment)
pub mod environment;

/// Provides the bandit protocol.
pub mod protocol;


/// Provides some structs used in bandit algorithms.
pub mod common;


pub use player::Player;
pub use environment::Environment;

pub use player::{
    EtcBuilder,
    Etc,

    UcbBuilder,
    Ucb,

    AsymptoticallyOptimalUcbBuilder,
    AsymptoticallyOptimalUcb,


    Exp3Builder,
    Exp3,


    Exp3IxBuilder,
    Exp3Ix,
};


pub use environment::{
    SubGaussianBuilder,
    SubGaussian,
};


pub use protocol::run;
