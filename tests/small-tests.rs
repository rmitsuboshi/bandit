// -----
// Problem setting
const HORIZON: usize = 100_000;
const N_ARMS: usize = 5;
// -----
// A parameter for ETC algorithm.
const PULL_EACH_ARM: usize = 100;
// -----
// The randomness for player.
const PLAYER_SEED: u64 = 111_111;
// -----
// Environment parameters.
const ENV_SEED: u64 = 123_456;
// Sub-gaussian constant.
const SIGMA: f64 = 1.0;
// -----

#[cfg(test)]
mod stochastic_bandits {
    use bandit::{
        EtcBuilder,
        UcbBuilder,
        AsymptoticallyOptimalUcbBuilder,
        SubGaussianBuilder,
        run,
    };
    use super::*;


    #[test]
    fn print_subgaussian_environment() {
        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();

        env.summary();
    }


    #[test]
    fn etc() {
        let etc = EtcBuilder::new(N_ARMS)
            .pull_each_arm(PULL_EACH_ARM)
            .build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();

        run(etc, env, HORIZON);
    }


    #[test]
    fn ucb() {
        let delta = 1.0 / (HORIZON as f64).powi(2);
        let ucb = UcbBuilder::new(N_ARMS, delta)
            .build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();


        run(ucb, env, HORIZON);
    }


    #[test]
    fn asymptotically_optimal_ucb() {
        let ucb = AsymptoticallyOptimalUcbBuilder::new(N_ARMS).build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();


        run(ucb, env, HORIZON);
    }
}


mod adversarial_bandits {
    use bandit::{
        EtcBuilder,
        Exp3Builder,
        Exp3IxBuilder,
        SubGaussianBuilder,
        EtcAdversaryBuilder,
        run,
    };
    use super::*;

    #[test]
    fn exp3() {
        let exp3 = Exp3Builder::new(N_ARMS)
            .seed(PLAYER_SEED)
            .horizon(HORIZON)
            .build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();


        run(exp3, env, HORIZON);
    }


    #[test]
    fn exp3ix_noconfidence() {
        let exp3ix = Exp3IxBuilder::new(N_ARMS)
            .seed(PLAYER_SEED)
            .horizon(HORIZON)
            .build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();


        run(exp3ix, env, HORIZON);
    }


    #[test]
    fn exp3ix_confidence() {
        let delta = 0.01;
        let exp3ix = Exp3IxBuilder::new(N_ARMS)
            .seed(PLAYER_SEED)
            .confidence(delta)
            .horizon(HORIZON)
            .build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(SIGMA)
            .seed(ENV_SEED)
            .build();


        run(exp3ix, env, HORIZON);
    }


    #[test]
    fn etc_vs_etc_adversarial_environment() {
        let etc = EtcBuilder::new(N_ARMS)
            .pull_each_arm(PULL_EACH_ARM)
            .build();

        let env = EtcAdversaryBuilder::new(N_ARMS, PULL_EACH_ARM)
            .build();

        run(etc, env, HORIZON);
    }


    #[test]
    fn exp3ix_vs_etc_adversarial_environment() {
        let delta = 0.01;
        let exp3ix = Exp3IxBuilder::new(N_ARMS)
            .seed(PLAYER_SEED)
            .confidence(delta)
            .horizon(HORIZON)
            .build();

        let env = EtcAdversaryBuilder::new(N_ARMS, PULL_EACH_ARM)
            .build();

        run(exp3ix, env, HORIZON);
    }
}
