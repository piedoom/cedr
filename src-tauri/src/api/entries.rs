use std::collections::HashMap;

use chinese_detection::classify;
use shared::{models, Id, InputMethod};
use sqlx::{self, Pool, Sqlite};

pub async fn query(
    pool: &Pool<Sqlite>,
    query: &String,
    method: &InputMethod,
) -> anyhow::Result<Vec<models::Entry>, sqlx::Error> {
    match method {
        InputMethod::Auto => match classify(&query) {
            chinese_detection::ClassificationResult::ZH => query_chinese(pool, query).await,
            chinese_detection::ClassificationResult::EN => query_english(pool, query).await,
            chinese_detection::ClassificationResult::PY => query_pinyin(pool, query).await,
            chinese_detection::ClassificationResult::UN => query_pinyin(pool, query).await,
        },
        InputMethod::Pinyin => query_pinyin(pool, query).await,
        InputMethod::Chinese => query_chinese(pool, query).await,
        InputMethod::English => query_english(pool, query).await,
    }
}

pub async fn query_pinyin(
    pool: &Pool<Sqlite>,
    query: &String,
) -> anyhow::Result<Vec<models::Entry>, sqlx::Error> {
    sqlx::query_as!(
        models::Entry,
        r#"
SELECT e.*
FROM entries e
WHERE
    REPLACE(e.pinyin, ' ', '') LIKE '%' || ?1 || '%' OR 
    REPLACE(e.pinyin_numbers, ' ', '') LIKE '%' || ?1 || '%' LIMIT 100
"#,
        query,
    )
    .fetch_all(pool)
    .await
}

pub async fn query_chinese(
    pool: &Pool<Sqlite>,
    query: &String,
) -> anyhow::Result<Vec<models::Entry>, sqlx::Error> {
    sqlx::query_as!(
        models::Entry,
        r#"
SELECT e.*
FROM entries e
WHERE e.traditional LIKE '%' || ?1 || '%' 
    OR e.simplified LIKE '%' || ?1 || '%' 
    ORDER BY (e.traditional LIKE ?1 || '%' OR e.simplified LIKE ?1 || '%') DESC
LIMIT 100
"#,
        query
    )
    .fetch_all(pool)
    .await
}

pub async fn query_english(
    pool: &Pool<Sqlite>,
    query: &String,
) -> anyhow::Result<Vec<models::Entry>, sqlx::Error> {
    sqlx::query_as!(
        models::Entry,
        "SELECT DISTINCT e.* FROM entries e WHERE e.definition LIKE '%' || ? || '%' LIMIT 100",
        query
    )
    .fetch_all(pool)
    .await
}

pub async fn get(pool: &Pool<Sqlite>, id: Id) -> anyhow::Result<models::Entry, sqlx::Error> {
    sqlx::query_as!(models::Entry, "SELECT e.* FROM entries e WHERE id = ?", id)
        .fetch_one(pool)
        .await
}
