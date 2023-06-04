use crate::{Player, Environment};


/// The bandit protocol.
pub fn run<P, E>(mut player: P, mut environment: E, horizon: usize)
    where P: Player,
          E: Environment,
{
    for t in 1..=horizon {
        let arm = player.choose(t);
        let reward = environment.reveal(arm);
        player.update(arm, reward);
    }

    let cumulative_reward = player.cumulative_reward();
    let best_arm = environment.best_arm();
    let next_arm = player.choose(horizon+1);
    print!(
        "\
        ----------\n\
        Result:\n\
        ----------\n\
        * Total reward gained: {cumulative_reward}\n\
        * Best arm in hindsight {best_arm}\n\
        * Player chooses arm {next_arm} in the next round\n\
        ----------\n\
        "
    );
}
