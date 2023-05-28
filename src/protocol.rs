use crate::{Player, Environment};


/// The bandit protocol.
pub fn run<P, E>(mut player: P, mut environment: E, horizon: usize)
    where P: Player,
          E: Environment,
{
    for t in 1..=horizon {
        let arm = player.choose(t);
        let loss = environment.reveal(arm);
        player.update(arm, loss);
    }

    let cumulative_loss = player.cumulative_loss();
    let best_arm = environment.best_arm();
    let next_arm = player.choose(horizon+1);
    print!(
        "\
        ----------\n\
        Result:\n\
        ----------\n\
        * Total loss incurred: {cumulative_loss}\n\
        * Best arm in hindsight {best_arm}\n\
        * Player chooses arm {next_arm} in the next round\n\
        ----------\n\
        "
    );
}
