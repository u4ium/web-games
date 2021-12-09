use super::model;
use sea_orm::sea_query::{ColumnDef, TableCreateStatement};
use sea_orm::{error::*, query::*, sea_query, DbConn, ExecResult};

async fn create_table(db: &DbConn, stmt: &TableCreateStatement) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}

pub async fn create_game_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(model::game::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(model::game::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(model::game::Column::Text)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt).await
}
