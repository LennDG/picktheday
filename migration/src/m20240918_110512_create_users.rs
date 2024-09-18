use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240918_104347_create_plans::Plans;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Users {
    Table, // this is a special case; will be mapped to `post`
    Id,
    PublicId,
    PlanId,
    Name,
    Ctime,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string_len(Users::PublicId, 32))
                    .col(string_len(Users::Name, 128))
                    .col(integer(Users::PlanId))
                    .col(timestamp_with_time_zone(Users::Ctime))
                    .to_owned(),
            )
            .await?;

        // FK plan_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Users::Table, Users::PlanId)
                    .to(Plans::Table, Plans::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Unique usernames within a plan
        manager
            .create_index(
                Index::create()
                    .table(Users::Table)
                    .col(Users::PlanId)
                    .col(Users::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on plan_id
        manager
            .create_index(
                Index::create()
                    .table(Users::Table)
                    .col(Users::PlanId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Users::Table).to_owned())
            .await
    }
}
