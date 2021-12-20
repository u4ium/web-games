#[macro_use]
extern crate rocket;

use rocket::{fairing::AdHoc, fs::FileServer};
use sea_orm_rocket::Database;

use server::database::{run_migrations, Db};

mod routes;
use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", FileServer::from("app/dist"))
        .mount(
            "/api",
            routes![game::new_game, game::get_game, game::get_games,],
        )
}
