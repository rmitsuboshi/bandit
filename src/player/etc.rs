//! The Explore-Then-Commit algorithm.
use crate::player::Player;


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
    n_arms: usize,
    losses: Vec<f64>,
}


impl Etc {
    /// Construct a new instance of `Etc`.
    pub(self) fn new(
        m: usize,
        n_arms: usize
    ) -> Self
    {
        let losses = vec![0.0; n_arms];
        Self { m, n_arms, losses, }
    }
}


impl Player for Etc {
    fn choose(&self, t: usize) -> usize {
        if t < self.m * self.n_arms {
            t % self.n_arms
        } else {
            argmax(&self.losses)
        }
    }


    fn update(&mut self, arm: usize, loss: f64) {
        self.losses[arm] += loss;
    }
}

pub(self) fn argmax(losses: &[f64]) -> usize {
    losses.iter()
        .enumerate()
        .max_by(|(_, vi), (_, vj)| vi.partial_cmp(vj).unwrap())
        .unwrap().0
}
