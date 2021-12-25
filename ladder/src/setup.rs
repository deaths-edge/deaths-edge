use sea_orm::{ConnectionTrait, DbConn, DbErr, Schema};

use crate::models::*;

pub async fn setup(db: &DbConn) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    // schema.create_enum_from_entity(Map);

    let stmt = builder.build(&schema.create_table_from_entity(Account));
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(Team2));
    db.execute(stmt).await?;

    let stmt = builder.build(&schema.create_table_from_entity(Match2));
    db.execute(stmt).await?;

    Ok(())
}
