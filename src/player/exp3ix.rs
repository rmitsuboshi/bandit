//! The Upper-Confidence-Bound algorithm.
use crate::player::Player;
use crate::common::Arms;

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};


use std::cell::RefCell;


/// A struct that builds `Exp3Ix`.
/// The `Exp3` algorithm has a favorable regret guarantee in expectation.
/// `Exp3-IX` is a similar one
/// which has a high-probability regret bound with same rate.
pub struct Exp3IxBuilder {
    seed: u64,
    n_arms: usize,
    horizon: usize,
    confidence: Option<f64>,
}


impl Exp3IxBuilder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize) -> Self {
        let seed = 1234;
        let horizon = 1_000;
        let confidence = None;
        Self { seed, n_arms, horizon, confidence, }
    }


    /// Set the seed.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }


    /// Set the confidence parameter `delta ∈ (0, 1)`.
    /// The regret bound for `Exp3-IX` holds
    /// with probability at least `1 - delta`.
    /// The confidence parameter is set `None` as default.
    /// Specifying `delta` yields a better regret guarantee.
    pub fn confidence(mut self, delta: f64) -> Self {
        self.confidence = Some(delta);
        self
    }


    /// Set the time horizon.
    pub fn horizon(mut self, horizon: usize) -> Self {
        self.horizon = horizon;
        self
    }


    /// Build a new instance of `Self`.
    pub fn build(self) -> Exp3Ix {
        Exp3Ix::new(self.seed, self.n_arms, self.horizon, self.confidence)
    }
}


/// The UCB algorithm.
pub struct Exp3Ix {
    arms: Arms,
    rng: RefCell<StdRng>,
    distribution: Uniform<f64>,

    gamma: f64,
    learning_rate: f64,
    probabilities: Vec<f64>,
    estimators: Vec<f64>,
}


impl Exp3Ix {
    /// Construct a new instance of `Exp3Ix`.
    pub(self) fn new(
        seed: u64,
        n_arms: usize,
        horizon: usize,
        confidence: Option<f64>,
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

        let nk = n_arms * horizon;
        let learning_rate = match confidence {
            Some(delta)
                => ((n_arms.ln() + ((n_arms + 1.0) / delta).ln()) / nk).sqrt(),
            None
                => (2.0 * (n_arms + 1.0).ln() / nk).sqrt(),
        };

        let gamma = 0.5 * learning_rate;


        Self {
            arms,
            rng,
            distribution,
            gamma,
            learning_rate,
            probabilities,
            estimators,
        }
    }
}


impl Player for Exp3Ix {
    fn name(&self) -> &str {
        "Exp3-IX"
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

        let x = (1.0 - reward) / (self.probabilities[arm] + self.gamma);

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
