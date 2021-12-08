use super::jsons::Game;
use std::sync::Mutex;

pub struct Db {
    pub games: Mutex<Vec<Game>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            games: Mutex::new(Vec::new()),
        }
    }
}

pub mod games;
