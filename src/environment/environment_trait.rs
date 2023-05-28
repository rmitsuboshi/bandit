//! This file defines a trait that Environment must satisfy.


/// A trait for Environment.
pub trait Environment {
    /// Reveals the loss value for the chosen arm.
    fn reveal(&mut self, arm: usize) -> f64;
}


