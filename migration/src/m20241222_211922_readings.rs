use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Readings::Table)
                    .col(pk_auto(Readings::Id))
                    .col(string_null(Readings::Meter))
                    .col(big_integer(Readings::Reading))
                    .col(date_null(Readings::Date))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Readings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Readings {
    Table,
    Id,
    Meter,
    Reading,
    Date,
}
