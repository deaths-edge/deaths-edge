use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum Map {
    #[sea_orm(num_value = 0)]
    Duo,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "match2")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,

    pub blue_team: u32,
    pub red_team: u32,

    pub map: Map,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
//     belongs_to = "super::team2::Entity",
//     from = "Column::BlueTeam",
//     to = "super::team2::Column::Id"
// )]
// BlueTeam,
// #[sea_orm(
//     belongs_to = "super::team2::Entity",
//     from = "Column::RedTeam",
//     to = "super::team2::Column::Id"
// )]
// RedTeam,
}

impl ActiveModelBehavior for ActiveModel {}
