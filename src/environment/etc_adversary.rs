//! Defines a worst-case environment for ETC algorithm.
//! This environment leads ETC algorithm to pull arm `0`
//! while other arms are the best ones.
use crate::environment::Environment;


/// An adversarial environment for ETC algorithm.
pub struct EtcAdversaryBuilder {
    n_arms: usize,
    pull_each_arm: usize,
}


impl EtcAdversaryBuilder {
    /// Construct a new builder of `EtcAdversary`.
    pub fn new(
        n_arms: usize,
        pull_each_arm: usize,
    ) -> Self
    {
        Self { n_arms, pull_each_arm }
    }


    /// Set pull-per-arm counter.
    pub fn pull_each_arm(mut self, count: usize) -> Self {
        self.pull_each_arm = count;
        self
    }


    /// Build an `EtcAdversary`.
    pub fn build(self) -> EtcAdversary {
        EtcAdversary::new(self.n_arms, self.pull_each_arm)
    }
}


/// A worst-case adversary for ETC algorithm.
pub struct EtcAdversary {
    n_arms: usize,
    pull_each_arm: usize,
    round_count: usize,
    first_arm: usize,
}


impl EtcAdversary {
    pub(self) fn new(n_arms: usize, pull_each_arm: usize) -> Self {
        let round_count = 0;
        let first_arm = 0;
        Self {
            n_arms,
            pull_each_arm,
            round_count,
            first_arm,
        }
    }
}


impl Environment for EtcAdversary {
    fn reveal(&mut self, arm: usize) -> f64 {
        self.round_count += 1;
        if self.round_count == 1 {
            self.first_arm = arm;
            return 1.0;
        }


        if self.round_count <= self.n_arms * self.pull_each_arm {
            0.0
        } else {
            if arm == self.first_arm { 0.0 } else { 1.0 }
        }
    }


    fn best_arm(&self) -> usize {
        (self.first_arm + 1) % self.n_arms
    }
}
