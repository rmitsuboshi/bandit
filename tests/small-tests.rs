
#[cfg(test)]
mod stochastic_bandits {
    use bandit::{
        EtcBuilder,
        UcbBuilder,
        AsymptoticallyOptimalUcbBuilder,
        SubGaussianBuilder,
        run,
    };

    const HORIZON: usize = 100_000;
    const N_ARMS: usize = 10;
    const SEED: u64 = 1111;

    #[test]
    fn etc() {
        let etc = EtcBuilder::new(N_ARMS)
            .pull_each_arm(10)
            .build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(1.0)
            .seed(SEED)
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
            .sigma(1.0)
            .seed(SEED)
            .build();


        run(ucb, env, HORIZON);
    }


    #[test]
    fn asymptotically_optimal_ucb() {
        let ucb = AsymptoticallyOptimalUcbBuilder::new(N_ARMS).build();

        let env = SubGaussianBuilder::new(N_ARMS)
            .range(0.0..1.0)
            .sigma(1.0)
            .seed(SEED)
            .build();


        run(ucb, env, HORIZON);
    }
}
