use crate::{Player, Environment};


const WIDTH: usize = 28;


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

    let name = player.name();
    let cumulative_reward = player.cumulative_reward();
    let next_arm = player.choose(horizon+1);

    let header = format!("Algorithm    │{name: >WIDTH$}");
    println!("┏━━━━━━━━━━━━━┯{:━<WIDTH$}┓", "");
    println!("┃{header}┃");
    println!("┣━━━━━━━━━━━━━┿{:━<WIDTH$}┫", "");
    let line = format!("Total Reward │{cumulative_reward: >WIDTH$}");
    println!("┃{line: <WIDTH$}┃");
    println!("┠─────────────┼{:─<WIDTH$}┨", "");
    let line = format!("Next arm     │{next_arm: >WIDTH$}");
    println!("┃{line: <WIDTH$}┃");
    println!("┗━━━━━━━━━━━━━┷{:━<WIDTH$}┛", "");


    player.arms().summary();
}
