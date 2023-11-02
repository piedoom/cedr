use shared::{models, Int};
use sqlx::{self, Pool, Sqlite};

pub async fn get_or_create(
    pool: &Pool<Sqlite>,
    entry_id: Int,
) -> anyhow::Result<models::Score, sqlx::Error> {
    // Get or create scores associated with entries
    sqlx::query_as!(
        models::Score,
        r#"
INSERT OR IGNORE INTO scores (entry_id) VALUES (?); 
SELECT scores.* FROM scores"#,
        entry_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get(pool: &Pool<Sqlite>, entry_id: Int) -> anyhow::Result<models::Score, sqlx::Error> {
    // Get or create scores associated with entries
    sqlx::query_as!(
        models::Score,
        "SELECT scores.* FROM scores WHERE id=?",
        entry_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &Pool<Sqlite>,
    entry_id: Int,
    correct: bool,
) -> anyhow::Result<models::Score, sqlx::Error> {
    // Log the score
    sqlx::query!("INSERT INTO scorelog (correct, score_id) VALUES (?1, (SELECT id FROM scores WHERE entry_id = ?2))", correct, entry_id).execute(pool)
            .await?;

    // Update the score
    match correct {
        false => {
            // Increment score by one
            sqlx::query!("UPDATE scores SET level = 0 WHERE entry_id = ?", entry_id)
                .execute(pool)
                .await?;
        }
        true => {
            // Reset score to 0
            sqlx::query!(
                "UPDATE scores SET level = level + 1 WHERE entry_id = ?",
                entry_id
            )
            .execute(pool)
            .await?;
        }
    }
    sqlx::query_as!(
        models::Score,
        "SELECT scores.* FROM scores WHERE entry_id = ?",
        entry_id
    )
    .fetch_one(pool)
    .await
}
