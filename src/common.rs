/// `ArmInfo` stores some information used in UCB.
#[derive(Debug, Clone)]
pub struct ArmInfo {
    cumulative_reward: f64,
    pull_count: u64,
}


impl ArmInfo {
    /// Constructs a new instance of `ArmInfo`.
    pub fn new() -> Self {
        Self { cumulative_reward: 0.0, pull_count: 0 }
    }


    /// Returns the empirical mean of the arm reward.
    pub fn empirical_mean(&self) -> Option<f64> {
        if self.pull_count == 0 {
            None
        } else {
            let mean = self.cumulative_reward / self.pull_count as f64;
            Some(mean)
        }
    }


    /// Returns the cumulative reward of this arm.
    pub fn cumulative_reward(&self) -> f64 {
        self.cumulative_reward
    }


    /// Returns the Upper-confidence-bound for this arm.
    /// This method returns
    /// - `f64::MAX` if this arm is not pulled,
    /// - UCB value, otherwise.
    pub fn ucb(&self, delta: f64) -> f64 {
        assert!(
            delta > 0.0,
            "The argument delta (= {delta}) must be positive"
        );
        let count = self.pull_count as f64;
        self.empirical_mean()
            .map(|mean| {
                let cb = (2.0 * delta.ln() / count as f64).sqrt();
                mean + cb
            })
            .unwrap_or(f64::MAX)
    }


    /// Returns `true` if this arm is not pulled, `false` otherwise.
    pub fn not_pulled(&self) -> bool {
        self.pull_count == 0
    }


    /// Update the arm information.
    pub fn update(&mut self, reward: f64) {
        self.cumulative_reward += reward;
        self.pull_count += 1;
    }
}
