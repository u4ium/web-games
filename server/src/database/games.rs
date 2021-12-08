use super::Db;
use super::Game;

trait Filter<T> {
    fn check(&self, item: &T) -> bool;
}

pub struct GameFilter {
    pub of: Option<usize>,
}

impl Filter<Game> for GameFilter {
    fn check(&self, game: &Game) -> bool {
        if let Some(of) = self.of {
            if of != game.of {
                return false;
            }
        }
        true
    }
}

impl Db {
    pub async fn get_game(&self, game_id: usize) -> Option<Game> {
        self.games.lock().unwrap().get(game_id).cloned()
    }

    pub async fn get_games(&self, filter: GameFilter) -> Vec<Game> {
        self.games
            .lock()
            .unwrap()
            .iter()
            .filter(|game| filter.check(game))
            .cloned()
            .collect()
    }

    pub async fn add_game(&self, game: Game) {
        self.games.lock().unwrap().push(game);
    }
}
