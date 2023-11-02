use std::{
    ffi::OsStr,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use shared::{models, Int};
use sqlx::{Pool, Sqlite};

pub async fn create(pool: &Pool<Sqlite>, name: String) -> anyhow::Result<Int> {
    let matches = sqlx::query!("INSERT INTO collections (name) VALUES (?1)", name)
        .execute(pool)
        .await?;
    Ok(matches.last_insert_rowid())
}

pub async fn add_term(
    pool: &Pool<Sqlite>,
    collection_id: Int,
    entry_id: Int,
) -> Result<Int, sqlx::Error> {
    Ok(sqlx::query!(
        "INSERT INTO collections_entries (collection_id, entry_id) VALUES (?1, ?2)",
        collection_id,
        entry_id,
    )
    .execute(pool)
    .await?
    .last_insert_rowid())
}

/// Add a term to the collection by the traditional characters instead of the [`Id`]
pub async fn add_term_by_traditional(
    pool: &Pool<Sqlite>,
    collection_id: Int,
    traditional: String,
) -> Result<Int, sqlx::Error> {
    Ok(sqlx::query!(
        r#"INSERT INTO collections_entries (entry_id, collection_id)
        VALUES ((SELECT entries.id FROM entries WHERE entries.traditional = ?1), ?2)"#,
        traditional,
        collection_id,
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

/// Get all entries in a collection
pub async fn get(
    pool: &Pool<Sqlite>,
    id: Int,
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

/// Export all collections as a RON file. Saves the traditional as the key instead of an ID
///
/// # Arguments
///
/// * `pool` - The [`Sqlite`] pool
/// * `path` - The absolute path at which to save this export
pub async fn export(pool: &Pool<Sqlite>, path: PathBuf) -> anyhow::Result<()> {
    let mut path = path.clone();
    path.set_extension(OsStr::new("ron"));
    let mut export = models::export::Collections::default();
    let collections: Vec<models::Collection> = index(pool).await?;
    // Our collections here are not actually in a format that is useful to export, so we'll change that
    for models::Collection { id, name, .. } in collections {
        // get entries in the collection
        let models::CollectionWithEntries { entries, .. } = get(pool, id).await?;
        // add the collection to the export data structure
        export.push(models::export::Collection {
            name,
            entries: entries.into_iter().map(|e| e.traditional).collect(),
        })
    }

    // Validate path. Do not create new folders if the location does not exist. Overwrite existing.
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    // Serialize the data
    let export_string = ron::to_string(&export)?;
    // Save to file
    file.write_all(export_string.as_bytes())?;

    Ok(())
}

/// Import collections from a path
/// TODO: better error and duplication handling
pub async fn import(pool: &Pool<Sqlite>, path: PathBuf) -> anyhow::Result<()> {
    // Get a file path
    let mut file = OpenOptions::new().read(true).open(path)?;
    // Deserialize
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let export_collections: models::export::Collections = ron::from_str(contents.as_str())?;
    // Reconstruct collections from raw info
    for export_collection in export_collections {
        // find or create collection
        let maybe_collection = index(pool)
            .await?
            .into_iter()
            .find(|x| x.name == export_collection.name);
        let collection_id = match maybe_collection {
            Some(c) => c.id,
            None => create(pool, export_collection.name).await?,
        };
        for entry in export_collection.entries {
            add_term_by_traditional(pool, collection_id, entry).await?;
        }
    }

    Ok(())
}
