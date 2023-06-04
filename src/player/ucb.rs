//! The Upper-Confidence-Bound algorithm.
use crate::player::Player;
use crate::common::ArmInfo;


/// A struct that builds `Ucb`.
pub struct UcbBuilder {
    n_arms: usize,
    confidence: f64,
}


impl UcbBuilder {
    /// Construct a new instance of `Self`.
    pub fn new(n_arms: usize, confidence: f64) -> Self {
        Self { n_arms, confidence }
    }


    /// Set `confidence`
    pub fn confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }


    /// Build a new instance of `Etc`.
    pub fn build(self) -> Ucb {
        Ucb::new(self.n_arms, self.confidence)
    }
}


/// The UCB algorithm.
pub struct Ucb {
    arms: Vec<ArmInfo>,
    confidence: f64,
}


impl Ucb {
    /// Construct a new instance of `Etc`.
    pub(self) fn new(
        n_arms: usize,
        confidence: f64,
    ) -> Self
    {
        assert!(n_arms > 0);
        let arms = (0..n_arms).map(|_| ArmInfo::new()).collect();
        Self { arms, confidence, }
    }
}


impl Player for Ucb {
    fn choose(&self, _t: usize) -> usize {
        let delta = 1.0 / self.confidence;
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
