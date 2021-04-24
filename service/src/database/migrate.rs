use rust_embed::RustEmbed;

use super::{Database, Transaction};
/// The embedded migrations files to apply.
#[derive(RustEmbed)]
#[folder = "migrations/"]
struct Migrations;

/// Migrate the database schema to tha latest version
#[tracing::instrument(name = "database::migrate", skip(db))]
pub async fn migrate(db: &Database) {
    tracing::debug!("Migrating database schema");

    let mut conn = db.connect().await;
    let tx = conn.begin().await;

    lock_migrations_table(&tx).await;
    let applied = list_applied_migrations(&tx).await;
    let available = list_available_migrations();

    let mut count: u32 = 0;
    for migration in &available {
        if applied.contains(migration) {
            tracing::debug!(migration = ?migration, "Migration already applied");
        } else {
            tracing::debug!(migration = ?migration, "Applying migration");
            let contents = Migrations::get(migration).expect("Failed to load migration");

            tx.batch_execute(std::str::from_utf8(&contents).expect("Failed to load migration"))
                .await
                .expect("Failed to apply migration");
            tx.execute("INSERT INTO __migrations(migration_file) VALUES ($1)", &[migration])
                .await
                .expect("Failed to record applied migration");
            count += 1;
        }
    }

    tx.commit().await.expect("Failed to commit transaction");

    tracing::info!(count = ?count, total = ?(available.len()), "Applied migrations");
}

async fn lock_migrations_table(tx: &Transaction<'_>) {
    tracing::trace!("Ensuring the migrations table exists");
    tx.execute(
        "CREATE TABLE IF NOT EXISTS __migrations(
        migration_file TEXT PRIMARY KEY,
        sequence SERIAL NOT NULL,
        executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        executed_from TEXT NOT NULL DEFAULT inet_client_addr()
      )",
        &[],
    )
    .await
    .expect("Failed to create __migrations table");

    tracing::trace!("Locking the migrations table");
    tx.execute("LOCK TABLE __migrations IN EXCLUSIVE MODE", &[])
        .await
        .expect("Failed to lock __migrations table");
}

async fn list_applied_migrations(tx: &Transaction<'_>) -> Vec<String> {
    tracing::trace!("Listing the applied migrations");

    let migrations = tx
        .query("SELECT migration_file FROM __migrations", &[])
        .await
        .expect("Failed to list applied migrations")
        .iter()
        .map(|row| row.get::<&str, String>("migration_file"))
        .collect::<Vec<String>>();
    tracing::debug!(migrations = ?migrations, "Migrations already applied");

    migrations
}

fn list_available_migrations() -> Vec<String> {
    tracing::trace!("Listing all migrations that can be applied");
    let mut migrations: Vec<String> = Migrations::iter().map(|f| f.to_string()).collect();
    migrations.sort();
    tracing::debug!(migrations = ?migrations, "All known migrations");

    migrations
}
