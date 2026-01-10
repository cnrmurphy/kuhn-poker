use inquire::Select;
use rand::{
    rng,
    rngs::ThreadRng,
    seq::{IndexedMutRandom, IndexedRandom, SliceRandom},
};

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
        hole_card: &str,
        pot: u64,
        antes: u64,
    ) -> PlayerAction;

    fn name(&self) -> &str;

    fn player_kind(&self) -> &PlayerKind;
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
