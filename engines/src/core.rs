use core::fmt::Display;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub struct GameError(String);

pub enum GameResult<P: PlayerIdentifier> {
    Winner(P),
    Draw,
}

pub trait GameState: Sized {
    type PlayerId: PlayerIdentifier;
    type Move: GameMove;
    fn make_move(&mut self, m: Self::Move);
    fn get_next_player_id(&self) -> Self::PlayerId;
    fn get_legal_moves(&self) -> HashSet<Self::Move>;
    fn get_progress(&mut self) -> GameProgress<Self>;
}

pub enum GameProgress<'a, S: GameState + 'a> {
    Complete(GameResult<S::PlayerId>),
    Incomplete(&'a mut S),
}

pub trait PlayerIdentifier: Copy + Clone + Debug + Hash + Eq + Display {}

pub trait GameMove: Copy + Clone + Debug + Hash + Eq + Display {}

/// An engine for a turn-based game with enumerable moves
pub trait Engine<S: GameState> {
    fn get_state(&self) -> GameProgress<S>;
}

pub trait GamePlayer<M: GameMove> {
    fn get_next_move(&self, legal_moves: &HashSet<M>) -> M;
    fn signal_bad_move(&self, bad_move: &M);
}

pub struct GameEngine<S: GameState> {
    players: HashMap<S::PlayerId, Box<dyn GamePlayer<S::Move>>>,
    state: S,
}

impl<S: GameState> GameEngine<S> {
    fn play(&mut self) -> Result<GameResult<S::PlayerId>, GameError> {
        while let GameProgress::Incomplete(state) = self.state.get_progress() {
            let next_player_id = state.get_next_player_id();
            if let Some(next_player) = self.players.get(&next_player_id) {
                let legal_moves = state.get_legal_moves();
                let next_move = loop {
                    let next_move = next_player.get_next_move(&legal_moves);
                    if legal_moves.contains(&next_move) {
                        break next_move;
                    }
                    next_player.signal_bad_move(&next_move);
                };
                state.make_move(next_move);
            } else {
                return Err(GameError(format!("missing player {}", next_player_id)));
            }
        }

        if let GameProgress::Complete(game_result) = self.state.get_progress() {
            return Ok(game_result);
        } else {
            unreachable!();
        }
    }
}
