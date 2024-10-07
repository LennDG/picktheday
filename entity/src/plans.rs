//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::{entity::prelude::*, IntoActiveModel, Set};

use crate::{
    db::ModelManager,
    types::{Description, PlanName, PublicId},
    users,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "plans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub public_id: PublicId,
    pub name: PlanName,
    pub description: Option<Description>,
    pub ctime: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct NewPlan {
    name: PlanName,
    description: Option<Description>,
}

impl NewPlan {
    pub fn new(name: PlanName, description: Option<Description>) -> Self {
        NewPlan { name, description }
    }
}

impl IntoActiveModel<ActiveModel> for NewPlan {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            name: Set(self.name),
            description: Set(self.description),
            public_id: Set(PublicId::default()),
            ctime: Set(time::OffsetDateTime::now_utc()),
            ..Default::default()
        }
    }
}

impl Model {
    pub async fn get_users(&self, mm: ModelManager) -> crate::error::Result<Vec<users::Model>> {
        Ok(self.find_related(users::Entity).all(mm.db()).await?)
    }
}

// region:	  --- Helper functions
pub mod helpers {
    use super::{Column, Entity, Model};
    use crate::{
        db::ModelManager,
        error::{Error, Result},
        types::PublicId,
        ID_MAP_CACHE,
    };
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    pub async fn plan_id_by_public_id(public_id: PublicId, mm: ModelManager) -> Result<i32> {
        // First, check if the user is already in the cache
        if let Some(cached_plan_id) = ID_MAP_CACHE.get(&public_id) {
            return Ok(cached_plan_id.clone());
        }

        // If not in the cache, get it from DB and put it into the cache
        let id = plan_by_public_id(public_id.clone(), mm.clone()).await?.id;
        ID_MAP_CACHE.insert(public_id, id);

        Ok(id)
    }

    pub async fn plan_by_public_id(id: PublicId, mm: ModelManager) -> Result<Model> {
        let plan = Entity::find()
            .filter(Column::PublicId.eq(id.clone()))
            .one(mm.db())
            .await?
            .ok_or(Error::EntityNotFound(id.to_string()))?;

        Ok(plan)
    }
}
// endregion: --- Helper functions
