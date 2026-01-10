use inquire::Select;
use rand::{
    rng,
    rngs::ThreadRng,
    seq::{IndexedMutRandom, IndexedRandom, SliceRandom},
};

use crate::PlayerAction;

pub trait Player {
    fn select_action(
        &mut self,
        actions: Vec<PlayerAction>,
        hole_card: &str,
        pot: u64,
        antes: u64,
    ) -> PlayerAction;

    fn name(&self) -> &str;
}

pub struct Client {
    name: String,
}

impl Client {
    pub fn new(name: String) -> Self {
        return Self { name };
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
}

pub struct Fish {
    name: String,
    rng: ThreadRng,
}

impl Fish {
    pub fn new() -> Self {
        return Self {
            name: String::from("Fish"),
            rng: rng(),
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
}
