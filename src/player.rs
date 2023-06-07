//! Player trait
pub mod player_trait;
pub mod etc;
pub mod ucb;
pub mod asymptotically_optimal_ucb;
pub mod exp3;
pub mod exp3ix;


pub use player_trait::Player;
pub use etc::{Etc, EtcBuilder};
pub use ucb::{Ucb, UcbBuilder};
pub use asymptotically_optimal_ucb::{
    AsymptoticallyOptimalUcb,
    AsymptoticallyOptimalUcbBuilder,
};
pub use exp3::{Exp3, Exp3Builder};
pub use exp3ix::{Exp3Ix, Exp3IxBuilder};
