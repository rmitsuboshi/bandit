//! Defines A subgaussion environment.
//! A distribution `D` is said to be `sigma`-subgaussian if for all `lambda`,
//! ```tex
//! \mathbb{E}_{X \sim D} [ \exp( lambda * X )]
//! \leq \exp( \lambda^2 sigma^2 / 2)
//! ```
//! 
use std::ops::Range;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;


use crate::environment::Environment;


/// A struct that builds `subgaussian` environment.
pub struct SubGaussianBuilder {
    // # of arms
    n_arms: usize,

    // Seed
    seed: u64,

    // The range of loss value.
    range: Range<f64>,


    sigma: f64,
}


impl SubGaussianBuilder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize) -> Self {
        let seed = 1234;
        let range = 0.0..1.0;
        let sigma = 1.0;
        Self { n_arms, seed, range, sigma, }
    }


    /// Reset the range of loss value.
    pub fn range(mut self, r: Range<f64>) -> Self {
        self.range = r;
        self
    }


    /// Set seed value.
    pub fn seed(mut self, s: u64) -> Self {
        self.seed = s;
        self
    }


    /// Builds [`SubGaussian`](SubGaussian)
    pub fn build(self) -> SubGaussian {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let uni = Uniform::from(self.range);
        let distributions = uni.sample_iter(&mut rng)
            .take(self.n_arms)
            .map(|mu| Normal::new(mu, self.sigma).unwrap())
            .collect::<Vec<_>>();

        SubGaussian::new(rng, distributions)
    }
}


/// The subgaussian environment.
pub struct SubGaussian {
    rng: StdRng,
    distributions: Vec<Normal<f64>>,
}


impl SubGaussian {
    /// Construct a new instance of `SubGaussian`.
    pub(self) fn new(rng: StdRng, distributions: Vec<Normal<f64>>)
        -> Self
    {
        Self { rng, distributions }
    }
}


impl Environment for SubGaussian {
    fn reveal(&mut self, arm: usize) -> f64 {
        self.distributions[arm]
            .sample(&mut self.rng)
            .max(0.0)
            .min(1.0)
    }
}
