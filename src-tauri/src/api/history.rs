use std::collections::{hash_map, HashMap};

use shared::models;
use sqlx::{types::chrono, Pool, Sqlite};

/// Get all items currently in the history
pub async fn index(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<models::Entry>> {
    let rows = sqlx::query!(
        r"
SELECT  
    h.id as history_id, h.created_at,
    t.id as term_id, t.traditional, t.simplified, t.pinyin, t.pinyin_numbers, t.pinyin_raw, t.tones, 
    d.id as entry_id, d.hash, d.source_id, d.definition, d.term, d.updated_at
FROM terms t
INNER JOIN history h ON t.id = h.term_id
INNER JOIN definitions d ON t.traditional = d.term
ORDER BY h.created_at DESC
            "
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .fold(
            indexmap::IndexMap::<usize, models::Entry>::default(),
            |mut acc, m| {
                match acc.entry(m.term_id as usize) {
                    indexmap::map::Entry::Occupied(mut v) => {
                        // Add the entry to the existing term
                        v.get_mut().definitions.push(models::Definition {
                            id: m.entry_id,
                            definition: m.definition.unwrap(),
                            source_id: m.source_id.unwrap(),
                            term: m.term.unwrap(),
                            hash: m.hash.unwrap(),
                            updated_at: m.updated_at.unwrap(),
                        });
                    }
                    indexmap::map::Entry::Vacant(v) => {
                        // Insert a new term to the vec if not exists
                        v.insert(models::Entry {
                            term: models::Term {
                                traditional: m.traditional,
                                simplified: m.simplified,
                                pinyin_raw: m.pinyin_raw,
                                pinyin_numbers: m.pinyin_numbers,
                                pinyin: m.pinyin,
                                tones: m.tones,
                                id: m.term_id,
                            },
                            definitions: vec![models::Definition {
                                id: m.entry_id,
                                definition: m.definition.unwrap(),
                                source_id: m.source_id.unwrap(),
                                term: m.term.unwrap(),
                                hash: m.hash.unwrap(),
                                updated_at: m.updated_at.unwrap(),
                            }],
                            examples: vec![],
                        });
                    }
                }
                acc
            },
        )
        .into_values()
        .collect::<Vec<models::Entry>>())
}

pub async fn create(pool: &Pool<Sqlite>, term_id: i64) -> anyhow::Result<()> {
    // Remove history if it exists
    sqlx::query!("DELETE FROM history WHERE term_id = ?", term_id)
        .execute(pool)
        .await
        .unwrap();
    // Insert the history item
    sqlx::query!("INSERT INTO history (term_id) VALUES (?1)", term_id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}
