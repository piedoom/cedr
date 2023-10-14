use indexmap::IndexMap;
use shared::models;
use sqlx::{Pool, Sqlite};

pub async fn create(pool: &Pool<Sqlite>, name: String) -> anyhow::Result<u32> {
    let matches = sqlx::query!("INSERT INTO collections (name) VALUES (?1)", name)
        .execute(pool)
        .await?;
    Ok(matches.last_insert_rowid() as u32)
}

pub async fn add_term(pool: &Pool<Sqlite>, collection_id: u32, term_id: u32) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO collections_terms (collection_id, term_id) VALUES (?1, ?2)",
        collection_id,
        term_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_term(
    pool: &Pool<Sqlite>,
    collection_id: u32,
    term_id: u32,
) -> anyhow::Result<()> {
    sqlx::query!(
        "DELETE FROM collections_terms WHERE (collection_id, term_id) = (?1, ?2)",
        collection_id,
        term_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn index(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<models::Collection>> {
    Ok(
        sqlx::query_as!(models::Collection, "SELECT collections.* FROM collections")
            .fetch_all(pool)
            .await?,
    )
}

pub async fn get(pool: &Pool<Sqlite>, id: u32) -> anyhow::Result<models::CollectionWithEntries> {
    let collection = sqlx::query!("SELECT collections.* FROM collections WHERE id = ?", id)
        .fetch_one(pool)
        .await?;

    let rows = sqlx::query!(
        r#"
SELECT t.*, d.id as definition_id, d.definition, d.hash, d.source_id, d.updated_at 
    FROM terms t
    JOIN collections_terms ct ON t.id = ct.term_id
    LEFT JOIN definitions d ON t.traditional = d.term
    WHERE ct.collection_id = ?;
    "#,
        id
    )
    .fetch_all(pool)
    .await?;

    let mut entries: IndexMap<String, models::Entry> = Default::default();

    for row in rows {
        match entries.entry(row.traditional.clone()) {
            indexmap::map::Entry::Occupied(mut v) => {
                v.get_mut().definitions.push(models::Definition {
                    id: row.definition_id,
                    definition: row.definition.unwrap(),
                    source_id: row.source_id.unwrap(),
                    term: row.traditional.clone(),
                    hash: row.hash.unwrap(),
                    updated_at: row.updated_at.unwrap(),
                })
            }
            indexmap::map::Entry::Vacant(v) => {
                v.insert(models::Entry {
                    term: models::Term {
                        id: row.id,
                        traditional: row.traditional.clone(),
                        simplified: row.simplified,
                        pinyin_raw: row.pinyin_raw,
                        pinyin_numbers: row.pinyin_numbers,
                        pinyin: row.pinyin,
                        tones: row.tones,
                    },
                    definitions: vec![models::Definition {
                        id: row.definition_id,
                        definition: row.definition.unwrap(),
                        source_id: row.source_id.unwrap(),
                        term: row.traditional,
                        hash: row.hash.unwrap(),
                        updated_at: row.updated_at.unwrap(),
                    }],
                    examples: vec![],
                });
            }
        }
    }

    Ok(models::CollectionWithEntries {
        collection: models::Collection {
            id: id as i64,
            name: collection.name,
        },
        entries: entries.into_values().collect(),
    })
}
