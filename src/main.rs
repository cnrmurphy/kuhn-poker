use std::{cmp::max, fmt};

use rand::{Rng, rng, rngs::ThreadRng, seq::SliceRandom};

use crate::player::{Agent, Client, Fish, Player, PlayerKind};

mod player;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    Bet,
    Call,
    Check,
    Fold,
}

impl fmt::Display for PlayerAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PlayerAction::Bet => "Bet",
            PlayerAction::Call => "Call",
            PlayerAction::Check => "Check",
            PlayerAction::Fold => "Fold",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    J,
    Q,
    K,
}

impl Card {
    fn as_str(&self) -> &'static str {
        match self {
            Card::J => "J",
            Card::Q => "Q",
            Card::K => "K",
        }
    }
}

pub struct GameState {
    button: usize,
    pot: u64,
    player_antes: [u64; 2],
    player_hands: [Card; 2],
    actions: Vec<PlayerAction>,
}

impl GameState {
    fn new() -> Self {
        return Self {
            button: 0,
            pot: 0,
            player_antes: [100, 100],
            player_hands: [Card::J, Card::Q],
            actions: Vec::with_capacity(3),
        };
    }
}

pub struct Engine {
    game_state: GameState,
    rng: ThreadRng,
    players: Vec<Box<dyn Player>>,
}

impl Engine {
    fn new(
        game_state: GameState,
        rng: ThreadRng,
        player_a: Box<dyn Player>,
        player_b: Box<dyn Player>,
    ) -> Self {
        return Self {
            game_state,
            rng,
            players: vec![player_a, player_b],
        };
    }

    fn assign_button(&mut self) {
        self.game_state.button = if self.rng.random_bool(0.5) { 0 } else { 1 }
    }

    fn button(&self) -> usize {
        self.game_state.button
    }

    fn out_ouf_position_player(&self) -> usize {
        if self.button() == 1 {
            return 0;
        } else {
            return 1;
        }
    }

    fn player_to_act(&self) -> usize {
        if self.game_state.actions.is_empty() || self.game_state.actions.len() % 2 == 0 {
            return self.out_ouf_position_player();
        } else {
            return self.button();
        }
    }

    fn shuffle_and_deal(&mut self) {
        let mut deck = [Card::J, Card::Q, Card::K];
        self.assign_button();
        deck.shuffle(&mut self.rng);

        // button is the last to act and last to be dealt
        if self.button() == 0 {
            self.game_state.player_hands[1] = deck[1];
            self.game_state.player_hands[0] = deck[0];
        } else {
            self.game_state.player_hands[0] = deck[0];
            self.game_state.player_hands[1] = deck[1];
        }
    }

    fn player_hand(&self, ptr: usize) -> Card {
        self.game_state.player_hands[ptr]
    }

    fn player_name(&self, ptr: usize) -> &str {
        self.players[ptr].name()
    }

    fn start_game(&mut self) {
        let mut new_game = true;
        while self.game_state.player_antes[0] > 0 && self.game_state.player_antes[1] > 0 {
            if new_game {
                self.begin_round();
                new_game = false;
                println!(
                    "{} antes: {} | {} antes: {} | pot: {}",
                    self.player_name(0),
                    self.game_state.player_antes[0],
                    self.player_name(1),
                    self.game_state.player_antes[1],
                    self.game_state.pot
                );

                if *self.players[0].player_kind() == PlayerKind::Client {
                    println!("Your hand: {}", self.player_hand(0).as_str())
                }

                if *self.players[1].player_kind() == PlayerKind::Client {
                    println!("Your hand: {}", self.player_hand(1).as_str())
                }
            }
            let valid_actions = self.get_valid_actions();
            if valid_actions.is_empty() {
                println!(
                    "{}: {}\n{}: {}",
                    self.player_name(0),
                    self.player_hand(0).as_str(),
                    self.player_name(1),
                    self.player_hand(1).as_str(),
                );
                self.reward_winning_hand();
                new_game = true;
                continue;
            }

            let active_player = self.player_to_act();

            let chosen_action = self.players[active_player].select_action(
                valid_actions,
                &self.game_state.actions,
                self.game_state.player_hands[active_player].as_str(),
                self.game_state.pot,
                self.game_state.player_antes[active_player],
            );

            if chosen_action == PlayerAction::Fold {
                let winner = if active_player == 0 { 1 } else { 0 };
                println!("{} folds", self.player_name(active_player));
                println!(
                    "{} wins {} antes",
                    self.player_name(winner),
                    self.game_state.pot
                );
                self.game_state.player_antes[winner] += self.game_state.pot;
                new_game = true;
                println!("---------");
                continue;
            }

            if chosen_action == PlayerAction::Bet {
                println!("{} bet 1 ante", self.players[active_player].name());
                self.game_state.pot += 1;
                self.game_state.player_antes[active_player] -= 1
            } else {
                println!(
                    "{} chose {}",
                    self.players[active_player].name(),
                    chosen_action
                );
            }

            self.game_state.actions.push(chosen_action);
        }
    }

    fn reward_winning_hand(&mut self) {
        let winning_hand = max(self.player_hand(0), self.player_hand(1));
        if winning_hand == self.player_hand(0) {
            println!("{} wins {} antes", self.player_name(0), self.game_state.pot);
            self.game_state.player_antes[0] += self.game_state.pot;
        } else {
            println!("{} wins {} antes", self.player_name(1), self.game_state.pot);
            self.game_state.player_antes[1] += self.game_state.pot;
        }
        println!("---------");
    }

    fn begin_round(&mut self) {
        let antes_collected = self.collect_antes(1);
        self.game_state.actions.clear();
        self.game_state.pot = antes_collected;
        self.shuffle_and_deal();
    }

    fn collect_antes(&mut self, ante_cost: u64) -> u64 {
        self.game_state.player_antes[0] -= ante_cost;
        self.game_state.player_antes[1] -= ante_cost;
        return ante_cost * 2;
    }

    fn get_valid_actions(&self) -> Vec<PlayerAction> {
        match self.game_state.actions.as_slice() {
            [] => vec![PlayerAction::Check, PlayerAction::Bet],
            [PlayerAction::Check] => vec![PlayerAction::Check, PlayerAction::Bet],
            [PlayerAction::Bet] => vec![PlayerAction::Fold, PlayerAction::Call],
            [PlayerAction::Check, PlayerAction::Bet] => {
                vec![PlayerAction::Fold, PlayerAction::Call]
            }
            [PlayerAction::Bet, PlayerAction::Fold]
            | [PlayerAction::Bet, PlayerAction::Call]
            | [PlayerAction::Check, PlayerAction::Bet, PlayerAction::Fold]
            | [PlayerAction::Check, PlayerAction::Bet, PlayerAction::Call]
            | [PlayerAction::Check, PlayerAction::Check] => {
                vec![]
            }
            _ => vec![],
        }
    }
}

fn main() {
    let agent = Agent::from_file("./strategies/basic.json").unwrap();
    let player_a = Box::from(agent);
    let player_b = Box::from(Client::new(String::from("human")));
    let game_state = GameState::new();
    let mut engine = Engine::new(game_state, rng(), player_a, player_b);
    println!("{} vs {}", engine.player_name(0), engine.player_name(1));
    engine.start_game();
}
