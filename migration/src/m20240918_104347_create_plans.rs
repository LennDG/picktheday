use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Plans {
    Table, // this is a special case; will be mapped to `post`
    Id,
    PublicId,
    Name,
    Description,
    Ctime,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Plans::Table)
                    .if_not_exists()
                    .col(pk_auto(Plans::Id))
                    .col(string_len(Plans::PublicId, 32))
                    .col(string_len(Plans::Name, 128))
                    .col(string_len_null(Plans::Description, 1024))
                    .col(timestamp_with_time_zone(Plans::Ctime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Plans::Table).to_owned())
            .await
    }
}
