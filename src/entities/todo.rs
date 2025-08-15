use sea_orm::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm( table_name = "todos" )]
pub struct Todo {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}