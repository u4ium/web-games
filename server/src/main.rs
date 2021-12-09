#[macro_use]
extern crate rocket;

use rocket::{fairing::AdHoc, form::Form, fs::FileServer, serde::json::Json};
use sea_orm::{entity::*, query::*};
use sea_orm_rocket::{Connection, Database};

use server::database::{model::game, run_migrations, Db};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", FileServer::from("app/dist"))
        .mount("/api", routes![new_game, get_game, get_games])
}

#[post("/games", data = "<game_form>")]
async fn new_game(conn: Connection<'_, Db>, game_form: Form<game::New>) {
    let db = conn.into_inner();
    let game_form = game_form.into_inner();
    let game = game::ActiveModel {
        text: Set(game_form.text.into()),
        ..Default::default()
    };
    game.insert(db).await.unwrap(); // TODO: handle err?
}

#[get("/games/<game_id>")]
async fn get_game(conn: Connection<'_, Db>, game_id: u32) -> Option<Json<game::Model>> {
    let db = conn.into_inner();
    game::Entity::find_by_id(game_id)
        .one(db)
        .await
        .unwrap()
        .map(|game| game.into()) // TODO: handle err?
}

#[get("/games?<of>")]
async fn get_games(conn: Connection<'_, Db>, of: Option<usize>) -> Json<Vec<game::Model>> {
    let db = conn.into_inner();
    // TODO: use filter
    game::Entity::find().all(db).await.unwrap().into() // TODO: handle err?
}
