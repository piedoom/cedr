use std::collections::HashMap;

use chinese_detection::classify;
use shared::{models, InputMethod};
use sqlx::{self, Pool, Sqlite};

pub async fn query(
    pool: &Pool<Sqlite>,
    query: String,
    method: InputMethod,
) -> anyhow::Result<Vec<models::Entry>, ()> {
    match method {
        InputMethod::Auto => match classify(&query) {
            chinese_detection::ClassificationResult::ZH => query_chinese(pool, &query).await,
            chinese_detection::ClassificationResult::EN => query_english(pool, &query).await,
            chinese_detection::ClassificationResult::PY => query_pinyin(pool, query).await,
            chinese_detection::ClassificationResult::UN => query_pinyin(pool, query).await,
        },
        InputMethod::Pinyin => query_pinyin(pool, query).await,
        InputMethod::Chinese => query_chinese(pool, &query).await,
        InputMethod::English => query_english(pool, &query).await,
    }
}

pub async fn query_pinyin(
    pool: &Pool<Sqlite>,
    query: String,
) -> anyhow::Result<Vec<models::Entry>, ()> {
    let mut query = query.to_string();
    query.retain(|c| !c.is_whitespace());
    let matches = sqlx::query!(
        r#"
SELECT 
    t.id as term_id, t.traditional, t.simplified, t.pinyin, t.pinyin_numbers, t.pinyin_raw, t.tones,
    d.id as definition_id, d.hash as definition_hash, d.updated_at, d.definition, d.term, d.source_id
FROM terms t
INNER JOIN definitions d ON t.traditional = d.term
WHERE 
    REPLACE(t.pinyin, ' ', '') LIKE '%' || ?1 || '%' OR 
    REPLACE(t.pinyin_numbers, ' ', '') LIKE '%' || ?1 || '%' OR 
    REPLACE(t.pinyin_raw, ' ', '') LIKE '%' || ?1 || '%' LIMIT 100
                "#,
        query
    )
    .fetch_all(pool)
    .await
    .unwrap();

    // Duplicate code - consider refactor to fn
    let mut entries: HashMap<String, models::Entry> = HashMap::with_capacity(matches.len());
    for m in matches.into_iter() {
        let definition = models::Definition {
            id: m.definition_id,
            definition: m.definition,
            source_id: m.source_id,
            term: m.term,
            hash: m.definition_hash,
            updated_at: m.updated_at,
        };
        entries
            .get_mut(m.traditional.as_str())
            .map(|v| v.definitions.push(definition.clone()))
            .unwrap_or_else(|| {
                // Insert new entry
                entries.insert(
                    m.traditional.clone(),
                    models::Entry {
                        term: models::Term {
                            id: m.term_id,
                            traditional: m.traditional,
                            simplified: m.simplified,
                            pinyin_raw: m.pinyin_raw,
                            pinyin_numbers: m.pinyin_numbers,
                            pinyin: m.pinyin,
                            tones: m.tones,
                        },
                        definitions: vec![definition],
                        examples: vec![],
                    },
                );
            });
    }
    Ok(entries.into_values().collect())
}

pub async fn query_chinese(
    pool: &Pool<Sqlite>,
    query: &String,
) -> anyhow::Result<Vec<models::Entry>, ()> {
    let matches = sqlx::query!(
        r#"
        SELECT
            t.id as term_id, t.traditional, t.simplified, t.pinyin, t.pinyin_numbers, t.pinyin_raw, t.tones,
            d.id as definition_id, d.hash as definition_hash, d.updated_at, d.definition, d.term, d.source_id
        FROM terms t
        INNER JOIN definitions d ON t.traditional = d.term
        WHERE t.traditional LIKE '%' || ?1 || '%' OR t.simplified LIKE '%' || ?1 || '%' LIMIT 100
                "#,
        query
    )
    .fetch_all(pool)
    .await
    .unwrap();
    // Right now, our matches have duplicate terms, as there is a row for every definition.
    // Here, we will deduplicate those terms and organize them under a single term with a
    // Vec of definitions
    // Begin by creating a separate HM to hold unique terms
    let mut entries: HashMap<String, models::Entry> = HashMap::with_capacity(matches.len());
    for m in matches.into_iter() {
        let definition = models::Definition {
            updated_at: m.updated_at,
            id: m.definition_id,
            definition: m.definition,
            source_id: m.source_id,
            term: m.term,
            hash: m.definition_hash,
        };
        // Build entries
        entries
            .get_mut(m.traditional.as_str())
            // A term already exists - push a definition to it
            .map(|v| v.definitions.push(definition.clone()))
            // Term does not yet exist. Insert new entry
            .unwrap_or_else(|| {
                entries.insert(
                    m.traditional.clone(),
                    models::Entry {
                        term: models::Term {
                            id: m.term_id,
                            traditional: m.traditional,
                            simplified: m.simplified,
                            pinyin_raw: m.pinyin_raw,
                            pinyin_numbers: m.pinyin_numbers,
                            pinyin: m.pinyin,
                            tones: m.tones,
                        },
                        definitions: vec![definition],
                        examples: vec![],
                    },
                );
            });
    }
    Ok(entries.into_values().collect())
}

pub async fn query_english(
    pool: &Pool<Sqlite>,
    query: &String,
) -> anyhow::Result<Vec<models::Entry>, ()> {
    let definitions = sqlx::query_as!(
        models::Definition,
        r#"SELECT definitions.* FROM definitions WHERE definition LIKE '%' || ? || '%' LIMIT 100"#,
        query
    )
    .fetch_all(pool)
    .await
    .unwrap();

    // select all terms where trad matches
    // TODO: Clean this up
    let terms = sqlx::query_as!(models::Term, "SELECT id, traditional, simplified, pinyin, pinyin_numbers, pinyin_raw, tones FROM terms WHERE traditional IN (SELECT term FROM definitions WHERE definition LIKE '%' || ? || '%') LIMIT 100", query).fetch_all(pool)
            .await
            .unwrap();

    // Build the terms
    Ok(terms
        .into_iter()
        .map(|term| {
            let definitions = definitions
                .iter()
                .filter(|entry| entry.term == term.traditional)
                .cloned()
                .collect::<Vec<models::Definition>>();
            models::Entry {
                term,
                definitions,
                examples: vec![],
            }
        })
        .collect())
}

pub async fn get(pool: &Pool<Sqlite>, id: u32) -> anyhow::Result<models::Entry, ()> {
    //     let rows = sqlx::query!(
    //         r#"
    // SELECT json_object(
    //     'term', json_object(
    //         'id', t.id,
    //         'traditional', t.traditional,
    //         'simplified', t.simplified,
    //         'pinyin', t.pinyin,
    //         'pinyin_numbers', t.pinyin_numbers,
    //         'pinyin_raw', t.pinyin_raw,
    //         'tones', t.tones
    //     ),
    //     'definitions', json_group_array(
    //         json_object(
    //             'id', d.id,
    //             'definition', d.definition,
    //             'hash', d.hash,
    //             'updated_at', d.updated_at
    //         )
    //     )
    // ) AS entries
    // FROM terms t
    // JOIN definitions d ON t.traditional = d.term
    // WHERE t.id = ?;
    //         "#,
    //         id
    //     )
    //     .fetch_one(pool)
    //     .await
    //     .unwrap();

    //     // We return a JSON object here, so we'll simply deserialize it
    //     let entry: models::Entry = serde_json::from_str(&rows.entries.unwrap()).unwrap();

    //     Ok(entry)

    let term = sqlx::query_as!(models::Term, "SELECT terms.* FROM terms WHERE id = ?", id)
        .fetch_one(pool)
        .await
        .unwrap();

    let definitions: Vec<models::Definition> = sqlx::query_as!(
        models::Definition,
        "SELECT definitions.* FROM definitions WHERE term = ?",
        term.traditional
    )
    .fetch_all(pool)
    .await
    .unwrap();

    Ok(models::Entry {
        term,
        definitions,
        examples: vec![],
    })
}
