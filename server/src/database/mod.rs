use rocket::{fairing, Build, Rocket};
use sea_orm_rocket::Database;

pub mod model;
pub mod pool;

mod setup;

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(pool::SeaOrmPool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = setup::create_game_table(conn).await;
    Ok(rocket)
}
