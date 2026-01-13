use std::collections::HashMap;

use inquire::Select;
use rand::{
    Rng, rng,
    rngs::ThreadRng,
    seq::{IndexedMutRandom, IndexedRandom, SliceRandom},
};
use serde::Deserialize;

use crate::PlayerAction;

#[derive(PartialEq, Eq, Debug)]
pub enum PlayerKind {
    Agent,
    Client,
}

pub trait Player {
    fn select_action(
        &mut self,
        actions: Vec<PlayerAction>,
        actions_occured: &[PlayerAction],
        hole_card: &str,
        pot: u64,
        antes: u64,
    ) -> PlayerAction;

    fn name(&self) -> &str;

    fn player_kind(&self) -> &PlayerKind;
}

#[derive(Deserialize)]
pub struct StrategyConfig {
    pub strategy: HashMap<String, HashMap<String, ActionFrequencies>>,
}

#[derive(Deserialize)]
pub struct ActionFrequencies {
    #[serde(default)]
    pub bet: f64,
    #[serde(default)]
    pub call: f64,
    #[serde(default)]
    pub check: f64,
    #[serde(default)]
    pub fold: f64,
}

pub struct Agent {
    name: String,
    config: StrategyConfig,
    rng: ThreadRng,
    player_kind: PlayerKind,
}

impl Agent {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let file_content = std::fs::read_to_string(path)?;
        let config: StrategyConfig = serde_json::from_str(&file_content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Self {
            name: String::from("Agent"),
            config,
            rng: rng(),
            player_kind: PlayerKind::Agent,
        })
    }

    fn history_to_key(actions: &[PlayerAction]) -> String {
        if actions.is_empty() {
            return "start".to_string();
        }
        actions
            .iter()
            .map(|a| format!("{}", a).to_lowercase())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Player for Agent {
    fn select_action(
        &mut self,
        actions: Vec<PlayerAction>,
        actions_occured: &[PlayerAction],
        hole_card: &str,
        pot: u64,
        antes: u64,
    ) -> PlayerAction {
        let history_key = Self::history_to_key(actions_occured);
        let frequencies = &self.config.strategy[hole_card][&history_key];
        let roll: f64 = self.rng.random();

        if actions.contains(&PlayerAction::Check) {
            // check/bet decision
            if roll < frequencies.check {
                PlayerAction::Check
            } else {
                PlayerAction::Bet
            }
        } else {
            // fold/call decision
            if roll < frequencies.fold {
                PlayerAction::Fold
            } else {
                PlayerAction::Call
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn player_kind(&self) -> &PlayerKind {
        return &self.player_kind;
    }
}

pub struct Client {
    name: String,
    player_kind: PlayerKind,
}

impl Client {
    pub fn new(name: String) -> Self {
        return Self {
            name,
            player_kind: PlayerKind::Client,
        };
    }
}

impl Player for Client {
    fn select_action(
        &mut self,
        actions: Vec<PlayerAction>,
        actions_occured: &[PlayerAction],
        hole_card: &str,
        pot: u64,
        antes: u64,
    ) -> PlayerAction {
        let choice = Select::new("Choose action: ", actions).prompt().unwrap();
        return choice;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn player_kind(&self) -> &PlayerKind {
        return &self.player_kind;
    }
}

pub struct Fish {
    name: String,
    rng: ThreadRng,
    player_kind: PlayerKind,
}

impl Fish {
    pub fn new() -> Self {
        return Self {
            name: String::from("Fish"),
            rng: rng(),
            player_kind: PlayerKind::Agent,
        };
    }
}

impl Player for Fish {
    fn select_action(
        &mut self,
        actions: Vec<PlayerAction>,
        actions_occured: &[PlayerAction],
        hole_card: &str,
        pot: u64,
        antes: u64,
    ) -> PlayerAction {
        let action = actions.choose(&mut self.rng).unwrap();
        return *action;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn player_kind(&self) -> &PlayerKind {
        return &self.player_kind;
    }
}
