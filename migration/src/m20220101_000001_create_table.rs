use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::DbBackend,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create Postgres enums
        manager
            .create_type(
                Type::create()
                    .as_enum("crate_orientation")
                    .values([Alias::new("short_x"), Alias::new("short_y")])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("principal_type"))
                    .values([Alias::new("user"), Alias::new("group")])
                    .to_owned(),
            )
            .await?;

        // 2. Maps table
        manager
            .create_table(
                Table::create()
                    .table(Maps::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Maps::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Maps::Name).string().not_null())
                    .col(ColumnDef::new(Maps::OwnerDn).text().not_null())
                    .col(
                        ColumnDef::new(Maps::OwnerType)
                            .enumeration("principal_type", ["user", "group"])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Maps::IsPublic)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Maps::WidthCm).double().not_null())
                    .col(ColumnDef::new(Maps::HeightCm).double().not_null())
                    .col(
                        ColumnDef::new(Maps::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".to_owned())),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Map authorizations table
        manager
            .create_table(
                Table::create()
                    .table(MapAuthorizations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MapAuthorizations::MapId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MapAuthorizations::PrincipalDn)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MapAuthorizations::PrincipalType)
                            .enumeration("principal_type", ["user", "group"])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MapAuthorizations::GrantedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".to_owned())),
                    )
                    .primary_key(
                        Index::create()
                            .col(MapAuthorizations::MapId)
                            .col(MapAuthorizations::PrincipalDn),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MapAuthorizations::Table, MapAuthorizations::MapId)
                            .to(Maps::Table, Maps::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Stacks table
        manager
            .create_table(
                Table::create()
                    .table(Stacks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Stacks::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Stacks::MapId).big_integer().not_null())
                    .col(ColumnDef::new(Stacks::Name).string())
                    .col(ColumnDef::new(Stacks::PosXCm).double().not_null())
                    .col(ColumnDef::new(Stacks::PosYCm).double().not_null())
                    .col(
                        ColumnDef::new(Stacks::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".to_owned())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Stacks::Table, Stacks::MapId)
                            .to(Maps::Table, Maps::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 5. Crate types table
        manager
            .create_table(
                Table::create()
                    .table(CrateTypes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CrateTypes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CrateTypes::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(CrateTypes::ShortCm).double().not_null())
                    .col(ColumnDef::new(CrateTypes::LongCm).double().not_null())
                    .to_owned(),
            )
            .await?;

        // 6. Crates table
        manager
            .create_table(
                Table::create()
                    .table(Crates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Crates::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Crates::CrateTypeId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Crates::Orientation)
                            .enumeration("crate_orientation", ["short_x", "short_y"])
                            .not_null(),
                    )
                    .col(ColumnDef::new(Crates::StackId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Crates::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".to_owned())),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Crates::Table, Crates::CrateTypeId)
                            .to(CrateTypes::Table, CrateTypes::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Crates::Table, Crates::StackId)
                            .to(Stacks::Table, Stacks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 7. Create extensions for fuzzy search
        let db = manager.get_connection();
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS citext")
            .await?;
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_trgm")
            .await?;

        // 8. Items table
        manager
            .create_table(
                Table::create()
                    .table(Items::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Items::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Items::MapId).big_integer().not_null())
                    .col(ColumnDef::new(Items::Name).custom("CITEXT").not_null())
                    .col(
                        ColumnDef::new(Items::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(SimpleExpr::Custom("now()".to_owned())),
                    )
                    .col(
                        ColumnDef::new(Items::SearchVector)
                            .custom("TSVECTOR")
                            .not_null()
                            .generated("setweight(to_tsvector('simple', name), 'A')", true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Items::Table, Items::MapId)
                            .to(Maps::Table, Maps::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // one item name per map
                    .index(
                        Index::create()
                            .name("uq_items_mapid_name")
                            .col(Items::MapId)
                            .col(Items::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 9. Indexes for fuzzy search
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_items_name_trgm ON items USING GIN (name gin_trgm_ops);",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_items_search_vector ON items USING GIN (search_vector);",
        )
        .await?;

        // 10. Item aliases table
        manager
            .create_table(
                Table::create()
                    .table(ItemAliases::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ItemAliases::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ItemAliases::ItemId).big_integer().not_null())
                    .col(
                        ColumnDef::new(ItemAliases::Alias)
                            .custom("CITEXT")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ItemAliases::Table, ItemAliases::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // no duplicate alias per item
                    .index(
                        Index::create()
                            .name("uq_itemaliases_itemid_alias")
                            .col(ItemAliases::ItemId)
                            .col(ItemAliases::Alias)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 11. Trigram index on alias
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_itemaliases_alias_trgm ON item_aliases USING GIN (alias gin_trgm_ops);",
        )
        .await?;

        // 12. Crate items join table
        manager
            .create_table(
                Table::create()
                    .table(CrateItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CrateItems::CrateId).big_integer().not_null())
                    .col(ColumnDef::new(CrateItems::ItemId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(CrateItems::CrateId)
                            .col(CrateItems::ItemId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CrateItems::Table, CrateItems::CrateId)
                            .to(Crates::Table, Crates::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CrateItems::Table, CrateItems::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop in reverse order
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_itemaliases_alias_trgm;")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_items_search_vector;")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_items_name_trgm;")
            .await?;

        manager
            .drop_table(Table::drop().table(CrateItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ItemAliases::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Items::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Crates::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CrateTypes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Stacks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MapAuthorizations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Maps::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("crate_orientation"))
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("principal_type"))
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared("DROP EXTENSION IF EXISTS pg_trgm;")
            .await?;
        db.execute_unprepared("DROP EXTENSION IF EXISTS citext;")
            .await?;

        Ok(())
    }
}

// Define table and column identifiers
#[derive(Iden)]
enum Maps {
    Table,
    Id,
    Name,
    OwnerDn,
    OwnerType,
    IsPublic,
    WidthCm,
    HeightCm,
    CreatedAt,
}

#[derive(Iden)]
enum MapAuthorizations {
    Table,
    MapId,
    PrincipalDn,
    PrincipalType,
    GrantedAt,
}

#[derive(Iden)]
enum Stacks {
    Table,
    Id,
    MapId,
    Name,
    PosXCm,
    PosYCm,
    CreatedAt,
}

#[derive(Iden)]
enum CrateTypes {
    Table,
    Id,
    Name,
    ShortCm,
    LongCm,
}

#[derive(Iden)]
enum Crates {
    Table,
    Id,
    CrateTypeId,
    Orientation,
    StackId,
    CreatedAt,
}

#[derive(Iden)]
enum Items {
    Table,
    Id,
    MapId,
    Name,
    CreatedAt,
    SearchVector,
}

#[derive(Iden)]
enum ItemAliases {
    Table,
    Id,
    ItemId,
    Alias,
}

#[derive(Iden)]
enum CrateItems {
    Table,
    CrateId,
    ItemId,
}
