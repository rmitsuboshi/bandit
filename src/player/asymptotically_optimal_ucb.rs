//! The Upper-Confidence-Bound algorithm.
use crate::player::Player;
use crate::common::ArmInfo;


/// A struct that builds `AsymptoticallyOptimalUcb`.
pub struct AsymptoticallyOptimalUcbBuilder {
    n_arms: usize,
}


impl AsymptoticallyOptimalUcbBuilder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize) -> Self {
        Self { n_arms }
    }


    /// Build a new instance of `Etc`.
    pub fn build(self) -> AsymptoticallyOptimalUcb {
        AsymptoticallyOptimalUcb::new(self.n_arms)
    }
}


/// The UCB algorithm.
pub struct AsymptoticallyOptimalUcb {
    arms: Vec<ArmInfo>,
}


impl AsymptoticallyOptimalUcb {
    /// Construct a new instance of `Etc`.
    pub(self) fn new(
        n_arms: usize,
    ) -> Self
    {
        assert!(n_arms > 0);
        let arms = (0..n_arms).map(|_| ArmInfo::new()).collect();
        Self { arms, }
    }
}


impl Player for AsymptoticallyOptimalUcb {
    fn choose(&self, t: usize) -> usize {
        if let Some(arm) = self.arms.iter().position(|arm| arm.not_pulled()) {
            return arm;
        }

        assert!(t > 0);
        let t = t as f64;

        let delta = 1_f64 + t * t.ln().powi(2);
        self.arms.iter()
            .map(|arm| arm.ucb(delta))
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap())
            .unwrap().0
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
