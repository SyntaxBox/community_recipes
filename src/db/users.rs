use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelBehavior;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile_pic: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub is_verified: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Recipes,
    Comments,
    Ratings,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Recipes => Entity::has_many(super::recipes::Entity).into(),
            Self::Comments => Entity::has_many(super::comments::Entity).into(),
            Self::Ratings => Entity::has_many(super::ratings::Entity).into(),
        }
    }
}

impl Related<super::recipes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipes.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::ratings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ratings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
