

use shared::{models, Id};
use sqlx::{Pool, Sqlite};

/// Get all items currently in the history
pub async fn index(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<models::Entry>, sqlx::Error> {
    sqlx::query_as!(
        models::Entry,
        r#"
SELECT e.*
FROM entries e
INNER JOIN history h ON e.id = h.entry_id
ORDER BY h.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &Pool<Sqlite>, entry_id: Id) -> anyhow::Result<Id, sqlx::Error> {
    // Remove history if it exists
    sqlx::query!("DELETE FROM history WHERE entry_id = ?", entry_id)
        .execute(pool)
        .await?;
    // Insert the history item
    Ok(
        sqlx::query!("INSERT INTO history (entry_id) VALUES (?1)", entry_id)
            .execute(pool)
            .await?
            .last_insert_rowid(),
    )
}
