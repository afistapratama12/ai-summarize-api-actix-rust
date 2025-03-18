use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub name: String,
  pub email: String,
  pub password: String,
  pub created_at: DateTime,
  pub updated_at: Option<DateTime>,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel {}
