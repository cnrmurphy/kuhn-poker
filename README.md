# Kuhn Poker
This is a work in progress implementation of a [Kuhn Poker](https://en.wikipedia.org/wiki/Kuhn_poker) engine written in Rust.
The goal of this project is to expand it to a full Kuhn Poker solver using
[Counterfactual Regret Minimization](https://proceedings.neurips.cc/paper/2007/file/08d98638c6fcd194a4b1e6992063e944-Paper.pdf) (CFR).

## Players
There are two `Player` types. A `Client` and an `Agent`. A `Client` is intended to be a human player that interfaces with the game
through the terminal (you could try to get an LLM like Claude to play it if you want). An `Agent` is an abstract decision-maker.
You can implement your own agents to play some specified strategy and have them play agains other agents or a client. Currently
included is a "dumb agent" called `Fish` that randomly selects valid actions - this is for testing.
