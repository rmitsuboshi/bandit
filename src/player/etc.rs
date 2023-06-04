//! The Explore-Then-Commit algorithm.
use crate::player::Player;

use crate::common::ArmInfo;


/// A struct that builds `Etc`.
pub struct EtcBuilder {
    m: usize,
    n_arms: usize,
}


impl EtcBuilder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize) -> Self {
        let m = 1;
        Self { m, n_arms }
    }


    /// Set `m`, the number of times each arm pulled.
    pub fn pull_each_arm(mut self, m: usize) -> Self {
        self.m = m;
        self
    }


    /// Build a new instance of `Etc`.
    pub fn build(self) -> Etc {
        Etc::new(self.m, self.n_arms)
    }
}


/// The ETC algorithm.
pub struct Etc {
    m: usize,
    arms: Vec<ArmInfo>,
}


impl Etc {
    /// Construct a new instance of `Etc`.
    pub(self) fn new(
        m: usize,
        n_arms: usize
    ) -> Self
    {
        let arms = (0..n_arms).map(|_| ArmInfo::new()).collect();
        Self { m, arms, }
    }
}


impl Player for Etc {
    fn choose(&self, t: usize) -> usize {
        let n_arms = self.arms.len();
        if t < self.m * n_arms {
            t % n_arms
        } else {
            self.arms.iter()
                .map(|arm| arm.empirical_mean())
                .enumerate()
                .max_by(|(_, vi), (_, vj)| vi.partial_cmp(vj).unwrap())
                .unwrap().0
        }
    }


    fn update(&mut self, arm: usize, reward: f64) {
        self.arms[arm].update(reward);
    }


    fn cumulative_reward(&self) -> f64 {
        self.arms.iter()
            .map(|arm| arm.cumulative_reward())
            .sum()
    }
}
