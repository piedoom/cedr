use indexmap::IndexMap;
use shared::{models, Id};
use sqlx::{Pool, Sqlite};

pub async fn create(pool: &Pool<Sqlite>, name: String) -> anyhow::Result<Id> {
    let matches = sqlx::query!("INSERT INTO collections (name) VALUES (?1)", name)
        .execute(pool)
        .await?;
    Ok(matches.last_insert_rowid())
}

pub async fn add_term(
    pool: &Pool<Sqlite>,
    collection_id: Id,
    term_id: Id,
) -> Result<Id, sqlx::Error> {
    Ok(sqlx::query!(
        "INSERT INTO collections_entries (collection_id, entry_id) VALUES (?1, ?2)",
        collection_id,
        term_id,
    )
    .execute(pool)
    .await?
    .last_insert_rowid())
}

pub async fn delete_term(
    pool: &Pool<Sqlite>,
    collection_id: u32,
    term_id: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM collections_entries WHERE (collection_id, entry_id) = (?1, ?2)",
        collection_id,
        term_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn index(pool: &Pool<Sqlite>) -> Result<Vec<models::Collection>, sqlx::Error> {
    sqlx::query_as!(models::Collection, "SELECT collections.* FROM collections")
        .fetch_all(pool)
        .await
}

pub async fn get(
    pool: &Pool<Sqlite>,
    id: Id,
) -> Result<models::CollectionWithEntries, sqlx::Error> {
    let collection = sqlx::query_as!(
        models::Collection,
        "SELECT collections.* FROM collections WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;

    let entries = sqlx::query_as!(
        models::Entry,
        r#"
SELECT e.*
FROM entries e
INNER JOIN collections_entries ce ON e.id = ce.entry_id
WHERE ce.collection_id = ?
    "#,
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(models::CollectionWithEntries {
        collection,
        entries,
    })
}
