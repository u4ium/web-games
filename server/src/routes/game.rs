use rocket::{form::Form, serde::json::Json};
use sea_orm::entity::*;
use sea_orm_rocket::Connection;

use server::database::{model::game, Db};

#[post("/games", data = "<game_form>")]
pub async fn new_game(conn: Connection<'_, Db>, game_form: Form<game::New>) {
    let db = conn.into_inner();
    let game_form = game_form.into_inner();
    let game = game::ActiveModel {
        text: Set(game_form.text.into()),
        ..Default::default()
    };
    game.insert(db).await.unwrap(); // TODO: handle err?
}

#[get("/games/<game_id>")]
pub async fn get_game(conn: Connection<'_, Db>, game_id: u32) -> Option<Json<game::Model>> {
    let db = conn.into_inner();
    game::Entity::find_by_id(game_id)
        .one(db)
        .await
        .unwrap()
        .map(|game| game.into()) // TODO: handle err?
}

#[get("/games?<of>")]
pub async fn get_games(conn: Connection<'_, Db>, of: Option<usize>) -> Json<Vec<game::Model>> {
    let db = conn.into_inner();
    // TODO: use filter
    game::Entity::find().all(db).await.unwrap().into() // TODO: handle err?
}
