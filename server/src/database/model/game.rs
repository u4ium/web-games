use rocket::{
    serde::{Deserialize, Serialize},
    FromForm,
};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct New {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "game")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
