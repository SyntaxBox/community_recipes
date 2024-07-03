use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ratings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rating_id: i32,
    pub user_id: i32,
    pub recipe_id: i32,
    pub rating: i32,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Recipes,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::UserId)
                .into(),
            Self::Recipes => Entity::belongs_to(super::recipes::Entity)
                .from(Column::RecipeId)
                .to(super::recipes::Column::RecipeId)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::recipes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
