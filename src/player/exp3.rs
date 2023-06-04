//! The Upper-Confidence-Bound algorithm.
use crate::player::Player;
use crate::common::Arms;

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};


use std::cell::RefCell;


/// A struct that builds `Exp3`.
pub struct Exp3Builder {
    seed: u64,
    n_arms: usize,
    horizon: usize,
}


impl Exp3Builder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize) -> Self {
        let seed = 1234;
        let horizon = 1_000;
        Self { seed, n_arms, horizon, }
    }


    /// Set the seed.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }


    /// Set the time horizon.
    pub fn horizon(mut self, horizon: usize) -> Self {
        self.horizon = horizon;
        self
    }


    /// Build a new instance of `Etc`.
    pub fn build(self) -> Exp3 {
        Exp3::new(self.seed, self.n_arms, self.horizon)
    }
}


/// The UCB algorithm.
pub struct Exp3 {
    arms: Arms,
    rng: RefCell<StdRng>,
    distribution: Uniform<f64>,

    learning_rate: f64,
    probabilities: Vec<f64>,
    estimators: Vec<f64>,
}


impl Exp3 {
    /// Construct a new instance of `Etc`.
    pub(self) fn new(
        seed: u64,
        n_arms: usize,
        horizon: usize,
    ) -> Self
    {
        assert!(n_arms > 0);
        let rng = StdRng::seed_from_u64(seed);
        let rng = RefCell::new(rng);

        let distribution = Uniform::from(0.0..1.0);

        let arms = Arms::new(n_arms);
        let estimators = vec![0.0; n_arms];
        let probabilities = vec![1.0 / n_arms as f64; n_arms];

        let n_arms = n_arms as f64;
        let horizon = horizon as f64;
        let learning_rate = (n_arms.ln() / (n_arms * horizon)).sqrt();
        Self {
            arms,
            rng,
            distribution,
            learning_rate,
            probabilities,
            estimators,
        }
    }
}


impl Player for Exp3 {
    fn name(&self) -> &str {
        "EXP3"
    }


    fn choose(&self, _t: usize) -> usize {
        let mut rng = self.rng.borrow_mut();
        let r = self.distribution.sample(&mut *rng);

        let mut pdf = 0.0;
        let mut arm = 0;
        for (k, p) in self.probabilities.iter().enumerate() {
            pdf += p;
            if r < pdf {
                arm = k;
                break;
            }
        }
        arm
    }


    fn update(&mut self, arm: usize, reward: f64) {
        self.arms.update(arm, reward);

        let x = (1.0 - reward) / self.probabilities[arm];

        let n_arms = self.arms.len();

        for k in 0..n_arms {
            let r = 1.0 - if k == arm { x } else { 0.0 };
            self.estimators[k] += r;
        }

        let exp = self.estimators.iter()
            .map(|r| (self.learning_rate * r).exp())
            .collect::<Vec<_>>();

        let denom = exp.iter().sum::<f64>();
        self.probabilities = exp.into_iter().map(|e| e / denom).collect();
    }


    fn cumulative_reward(&self) -> f64 {
        self.arms.cumulative_reward()
    }


    fn arms(&self) -> &Arms {
        &self.arms
    }
}
