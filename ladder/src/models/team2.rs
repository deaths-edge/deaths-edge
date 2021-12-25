use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "team2")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    pub name: String,
    pub rating: u32,

    pub member_1: u32,
    pub member_2: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
//     belongs_to = "super::account::Entity",
//     from = "Column::Member1",
//     to = "super::account::Column::Id"
// )]
// Member1,
// #[sea_orm(
//     belongs_to = "super::account::Entity",
//     from = "Column::Member2",
//     to = "super::account::Column::Id"
// )]
// Member2,
}

impl ActiveModelBehavior for ActiveModel {}
