
#[cfg(test)]
mod stochastic_bandits {
    use bandit::{EtcBuilder, SubGaussianBuilder, run};
    #[test]
    fn etc() {
        let n_arms = 2;
        let etc = EtcBuilder::new(n_arms)
            .pull_each_arm(10)
            .build();

        let env = SubGaussianBuilder::new(n_arms)
            .range(0.0..1.0)
            .sigma(1.0)
            .seed(1234)
            .build();


        run(etc, env, 100);
    }
}
