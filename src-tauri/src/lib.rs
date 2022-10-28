use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = "sqlite://tasks.sqlite?mode=rwc".to_string();
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}
