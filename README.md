# Bandit
A small collection of Bandit algorithms, written in [Rust](https://www.rust-lang.org/)🦀.

This crate provides a generic framework for bandit protocol.
The algorithms in this crate are named 
based on [this book](https://tor-lattimore.com/downloads/book/book.pdf).


# Test run
One can reproduce the following result by `cargo test -- --nocapture` command.
![demo image](img/demo.png)


## How To Use
Write the following line to `Cargo.toml`.
```toml
bandit = { version = "0.1.0" }
```

You can find code examples in `tests/small-tests.rs`.
I'll write some documents to every algorithms in this crate.


Currently, the following algorithms are implemented.
### Stochastic Bandits
- ETC (Explore-Then-Commit),
- UCB (Upper-Confidence-Bound),
- Asymptotically Optimal UCB (Asymptotically Optimal Upper-Confidence-Bound),

### Adversarial Bandits
- Exp3 (Exponential-weight algorithm for Exploration and Exploitation)


### Environments
- Sub-Gaussian environment,
