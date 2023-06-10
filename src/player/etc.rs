//! The Explore-Then-Commit algorithm.
use crate::player::Player;
use crate::common::Arms;


/// A struct that builds `Etc`.
pub struct EtcBuilder {
    pull_each_arm: usize,
    n_arms: usize,
}


impl EtcBuilder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize) -> Self {
        let pull_each_arm = 1;
        Self { pull_each_arm, n_arms }
    }


    /// Set `m`, the number of times each arm pulled.
    pub fn pull_each_arm(mut self, pull_each_arm: usize) -> Self {
        self.pull_each_arm = pull_each_arm;
        self
    }


    /// Build a new instance of `Etc`.
    pub fn build(self) -> Etc {
        Etc::new(self.pull_each_arm, self.n_arms)
    }
}


/// The ETC algorithm.
pub struct Etc {
    pull_each_arm: usize,
    arms: Arms,
    best_arm: Option<usize>,
    round_count: usize,
}


impl Etc {
    /// Construct a new instance of `Etc`.
    pub(self) fn new(
        pull_each_arm: usize,
        n_arms: usize
    ) -> Self
    {
        let arms = Arms::new(n_arms);
        let best_arm = None;
        let round_count = 0;
        // let round_count = Cell::new(0);
        Self { pull_each_arm, arms, best_arm, round_count, }
    }
}


impl Player for Etc {
    fn name(&self) -> &str {
        "ETC (Explore-Then-Commit)"
    }


    fn choose(&self, t: usize) -> usize {
        assert!(t > 0, "The round counter must be positive. Got {t}");
        let n_arms = self.arms.len();
        self.best_arm.unwrap_or((t-1) % n_arms)
    }


    fn update(&mut self, arm: usize, reward: f64) {
        self.round_count += 1;
        self.arms.update(arm, reward);

        let n_arms = self.arms.len();
        if self.round_count >= self.pull_each_arm * n_arms {
            if self.best_arm.is_none() {
                self.best_arm = self.arms.empirical_means()
                    .enumerate()
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .map(|(arm, _)| arm);
            }
        }
    }


    fn cumulative_reward(&self) -> f64 {
        self.arms.cumulative_reward()
    }


    fn arms(&self) -> &Arms {
        &self.arms
    }
}
