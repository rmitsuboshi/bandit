//! Player trait
pub mod player_trait;
pub mod etc;
pub mod ucb;
pub mod asymptotically_optimal_ucb;


pub use player_trait::Player;
pub use etc::{Etc, EtcBuilder};
pub use ucb::{Ucb, UcbBuilder};
pub use asymptotically_optimal_ucb::{
    AsymptoticallyOptimalUcb,
    AsymptoticallyOptimalUcbBuilder,
};
