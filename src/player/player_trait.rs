//! This file defines a trait that Baidit algorithm must satisfy.
use crate::common::Arms;


/// A trait for Bandit player.
pub trait Player {
    /// Algorithm name.
    fn name(&self) -> &str;


    /// Choose an arm .
    fn choose(&self, t: usize) -> usize;


    /// Update the state.
    fn update(&mut self, arm: usize, loss: f64);


    /// Returns the total loss value.
    fn cumulative_reward(&self) -> f64;


    /// Returns the arms, maintained by an algorithm.
    fn arms(&self) -> &Arms;
}



