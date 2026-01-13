# Kuhn Poker
This is a work in progress implementation of a [Kuhn Poker](https://en.wikipedia.org/wiki/Kuhn_poker) engine written in Rust.
The goal of this project is to expand it to a full Kuhn Poker solver using
[Counterfactual Regret Minimization](https://proceedings.neurips.cc/paper/2007/file/08d98638c6fcd194a4b1e6992063e944-Paper.pdf) (CFR).

## Players
There are two `Player` types. A `Client` and an `Agent`. A `Client` is intended to be a human player that interfaces with the game
through the terminal (you could try to get an LLM like Claude to play it if you want). An `Agent` is an abstract decision-maker.
You can implement your own agents to play some specified strategy and have them play agains other agents or a client. Currently
included is a "dumb agent" called `Fish` that randomly selects valid actions - this is for testing.

## Creating a Strategy

Strategies are defined in JSON files and loaded by the `Agent`. A strategy specifies the probability of each action for each card and game state.

### Game States

There are 4 decision points where a player must act:

| Key         | Situation                      | Valid Actions |
|-------------|--------------------------------|---------------|
| `start`     | First to act (out of position) | Check, Bet    |
| `check`     | Opponent checked (on button)   | Check, Bet    |
| `bet`       | Opponent bet (on button)       | Fold, Call    |
| `check,bet` | You checked, opponent bet      | Fold, Call    |

### Strategy Format

```json
{
  "strategy": {
    "J": {
      "start": { "check": 0.7, "bet": 0.3 },
      "check": { "check": 0.7, "bet": 0.3 },
      "bet": { "fold": 1.0, "call": 0.0 },
      "check,bet": { "fold": 1.0, "call": 0.0 }
    },
    "Q": {
      "start": { "check": 1.0, "bet": 0.0 },
      "check": { "check": 1.0, "bet": 0.0 },
      "bet": { "fold": 0.66, "call": 0.34 },
      "check,bet": { "fold": 0.66, "call": 0.34 }
    },
    "K": {
      "start": { "check": 0.3, "bet": 0.7 },
      "check": { "check": 0.0, "bet": 1.0 },
      "bet": { "fold": 0.0, "call": 1.0 },
      "check,bet": { "fold": 0.0, "call": 1.0 }
    }
  }
}
```

### Notes

- Frequencies should be between 0.0 and 1.0
- For check/bet states, `check` is the probability of checking (bet is inferred as `1 - check`)
- For fold/call states, `fold` is the probability of folding (call is inferred as `1 - fold`)
- Place strategy files in the `strategies/` directory
