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
}

pub struct Client {
    name: String,
}

impl Client {
    fn new(name: String) -> Self {
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
}

pub struct Fish {
    name: String,
    rng: ThreadRng,
}

impl Fish {
    fn new() -> Self {
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
}
