use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .col(
                        ColumnDef::new(Todo::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todo::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Todo::UserId).big_unsigned().not_null())
                    .col(ColumnDef::new(Todo::Title).string().not_null())
                    .col(ColumnDef::new(Todo::Status).string().not_null())
                    .col(ColumnDef::new(Todo::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Todo::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Todo {
    Table,
    Id,
    Uuid,
    UserId,
    Title,
    Status,
    CreatedAt,
    UpdatedAt,
}
