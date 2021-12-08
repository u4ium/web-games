use rocket::form::FromForm;
use rocket::serde::Serialize;

#[derive(FromForm, Serialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub of: usize,
}
