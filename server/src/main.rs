#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::State;

use server::{
    database::{games::GameFilter, Db},
    jsons::*,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Db::new())
        .mount("/", FileServer::from("app/dist"))
        .mount("/api", routes![new_game, get_game, get_games])
}

#[post("/games", data = "<game>")]
async fn new_game(db: &State<Db>, game: Form<Game>) {
    db.add_game(game.into_inner()).await
}

#[get("/games/<game_id>")]
async fn get_game(db: &State<Db>, game_id: usize) -> Option<Json<Game>> {
    db.get_game(game_id).await.map(|g| g.into())
}

#[get("/games?<of>")]
async fn get_games(db: &State<Db>, of: Option<usize>) -> Json<Vec<Game>> {
    db.get_games(GameFilter { of }).await.into()
}
