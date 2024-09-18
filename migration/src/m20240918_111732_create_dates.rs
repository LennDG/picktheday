use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240918_110512_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Dates {
    Table,
    Id,
    UserId,
    Date,
    Ctime,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dates::Table)
                    .if_not_exists()
                    .col(pk_auto(Dates::Id))
                    .col(integer(Dates::UserId))
                    .col(date(Dates::Date))
                    .col(timestamp_with_time_zone(Dates::Ctime))
                    .to_owned(),
            )
            .await?;

        // FK user_id
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Dates::Table, Dates::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Unique dates per user
        manager
            .create_index(
                Index::create()
                    .table(Dates::Table)
                    .col(Dates::Date)
                    .col(Dates::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on user_id
        manager
            .create_index(
                Index::create()
                    .table(Dates::Table)
                    .col(Dates::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Dates::Table).to_owned())
            .await
    }
}
