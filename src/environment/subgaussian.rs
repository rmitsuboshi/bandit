//! Defines a subgaussion environment.
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

    // The range of reward value.
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


    /// Reset the range of reward value.
    pub fn range(mut self, r: Range<f64>) -> Self {
        self.range = r;
        self
    }


    /// Set seed value.
    pub fn seed(mut self, s: u64) -> Self {
        self.seed = s;
        self
    }


    /// Set the subgaussian parameter.
    pub fn sigma(mut self, sigma: f64) -> Self {
        self.sigma = sigma;
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


    /// Returns the mean value for each arm.
    pub fn means(&self) -> Vec<f64> {
        self.distributions.iter()
            .map(|p| p.mean())
            .collect()
    }


    /// Prints the arm information.
    pub fn summary(&self) {
        let n_arms = self.distributions.len();
        let header = (0..n_arms).map(|k| format!(" {k: ^5} "))
            .collect::<Vec<_>>()
            .join("|");

        let border = (0..n_arms).map(|_| "-------")
            .collect::<Vec<_>>()
            .join("+");
        let means = self.means();
        let content = means.iter().map(|m| format!(" {m: >1.3} "))
            .collect::<Vec<_>>()
            .join("|");
        println!("+------+{border}+");
        println!("| ARM  |{header}|");
        println!("+------+{border}+");
        println!("| MEAN |{content}|");
        println!("+------+{border}+");
    }
}


impl Environment for SubGaussian {
    fn reveal(&mut self, arm: usize) -> f64 {
        self.distributions[arm]
            .sample(&mut self.rng)
            .max(0.0)
            .min(1.0)
    }


    fn best_arm(&self) -> usize {
        self.distributions.iter()
            .enumerate()
            .max_by(|(_, p), (_, q)|
                p.mean().partial_cmp(&q.mean()).unwrap()
            )
            .unwrap().0
    }
}
