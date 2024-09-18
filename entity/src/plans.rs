//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::{entity::prelude::*, IntoActiveModel, Set};

use crate::types::{Description, PlanName, PublicId};

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
            public_id: Set(PublicId::new()),
            ctime: Set(time::OffsetDateTime::now_utc()),
            ..Default::default()
        }
    }
}
