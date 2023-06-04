//! This file defines a trait that Environment must satisfy.


/// A trait for Environment.
pub trait Environment {
    /// Reveals the reward for the chosen arm.
    fn reveal(&mut self, arm: usize) -> f64;

    /// Returns the best arm
    fn best_arm(&self) -> usize;
}


