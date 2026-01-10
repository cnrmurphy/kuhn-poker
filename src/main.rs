use std::fmt;

use rand::{rng, rngs::ThreadRng, seq::SliceRandom};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    turn: u8,
    pot: u64,
    player_a_antes: u64,
    player_b_antes: u64,
    player_a_hand: Card,
    player_b_hand: Card,
    actions: Vec<PlayerAction>,
}

impl GameState {
    fn new() -> Self {
        return Self {
            pot: 0,
            turn: 0,
            player_a_antes: 100,
            player_b_antes: 100,
            player_a_hand: Card::J,
            player_b_hand: Card::Q,
            actions: Vec::with_capacity(3),
        };
    }
}

pub struct Engine {
    game_state: GameState,
    rng: ThreadRng,
}

impl Engine {
    fn new(game_state: GameState, rng: ThreadRng) -> Self {
        return Self { game_state, rng };
    }

    fn shuffle_and_deal(&mut self) {
        let mut deck = [Card::J, Card::Q, Card::K];
        deck.shuffle(&mut self.rng);
        self.game_state.player_a_hand = deck[0];
        self.game_state.player_b_hand = deck[1];
    }

    fn start_game(&mut self) {
        while self.game_state.player_a_antes > 0 && self.game_state.player_b_antes > 0 {
            self.begin_round();
        }
    }

    fn begin_round(&mut self) {
        let antes_collected = self.collect_antes(1);
        self.game_state.pot += antes_collected;
        self.shuffle_and_deal();
    }

    fn collect_antes(&mut self, ante_cost: u64) -> u64 {
        self.game_state.player_a_antes -= ante_cost;
        self.game_state.player_b_antes -= ante_cost;
        return ante_cost * 2;
    }

    fn request_action(&self, action: PlayerAction) {
        let valid_actions = self.get_valid_actions();
    }

    fn get_valid_actions(&self) -> Vec<PlayerAction> {
        let is_last_player_to_act = self.game_state.actions.len() % 2 == 0; // The in position player
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
    let game_state = GameState::new();
    let mut engine = Engine::new(game_state, rng());
    engine.start_game();
    println!(
        "player_a: {}\nplayer_b: {}\npot: {}",
        engine.game_state.player_a_hand.as_str(),
        engine.game_state.player_b_hand.as_str(),
        engine.game_state.pot
    );
}
