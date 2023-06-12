/// `Arm` stores some information used in UCB.
#[derive(Debug, Clone)]
pub struct Arm {
    cumulative_reward: f64,
    pull_count: u64,
}


impl Arm {
    /// Constructs a new instance of `Arm`.
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


/// A struct that represents an arm set.
#[derive(Debug, Clone)]
pub struct Arms {
    arms: Vec<Arm>,
}


impl Arms {
    /// Construct a new instance of `Arms`.
    pub fn new(n_arms: usize) -> Self {
        let arms = vec![Arm::new(); n_arms];
        Self { arms }
    }


    /// Update the arm information.
    pub fn update(&mut self, arm: usize, reward: f64) {
        self.arms[arm].update(reward);
    }


    /// Returns `Some(arm)` if there exists an arm not pulled,
    /// `None` otherwise.
    pub fn not_pulled(&self) -> Option<usize> {
        self.arms.iter().position(|arm| arm.not_pulled())
    }


    /// Returns an iterator of UCB values over arms.
    pub fn ucb<'a>(&'a self, delta: f64) -> impl Iterator<Item=f64> + 'a {
        self.arms.iter()
            .map(move |arm| arm.ucb(delta))
    }


    /// Returns the cumulative reward of this arm.
    pub fn cumulative_reward(&self) -> f64 {
        self.arms.iter()
            .map(|arm| arm.cumulative_reward())
            .sum()
    }



    /// Returns the empirical mean of the arm reward.
    pub fn empirical_means<'a>(&'a self) -> impl Iterator<Item=f64> + 'a {
        self.arms.iter()
            .map(|arm| arm.empirical_mean().unwrap())
    }

    /// Returns the number of arms.
    pub fn len(&self) -> usize {
        self.arms.len()
    }


    /// Prints the empirical mean reward for each arm.
    pub fn summary(&self) {
        let n_arms = self.arms.len();
        let header = (0..n_arms).map(|k| format!(" {k: ^5} "))
            .collect::<Vec<_>>()
            .join("┃");

        let content = self.empirical_means()
            .map(|mean| {
                let sgn = if mean >= 0.0 { ' ' } else { '-' };
                let mean = mean.abs();
                format!(" {sgn}{mean: >1.3}")
            })
            .collect::<Vec<_>>()
            .join("┃");
        let toprule = (0..n_arms).map(|_| "━━━━━━━")
            .collect::<Vec<_>>()
            .join("┳");
        let midrule = (0..n_arms).map(|_| "━━━━━━━")
            .collect::<Vec<_>>()
            .join("╋");
        let botrule = (0..n_arms).map(|_| "━━━━━━━")
            .collect::<Vec<_>>()
            .join("┻");
        println!("┏━━━━━┳{toprule}┓");
        println!("┃ ARM ┃{header:}┃");
        println!("┣━━━━━╋{midrule}┫");
        println!("┃ AVG ┃{content}┃");
        println!("┗━━━━━┻{botrule}┛");
    }
}




