use std::path::PathBuf;

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use shared::models;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

fn path() -> PathBuf {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "doomy".to_string(),
        author: "doomy".to_string(),
        app_name: "cedr".to_string(),
    })
    .unwrap();

    strategy
        .data_dir()
        .join("preferences")
        .with_extension("ron")
}

/// Get existing or create a default preferences file in the data location
pub async fn get_or_create() -> Result<models::Preferences, anyhow::Error> {
    let path = path();

    // Store this result before we open/create the file
    let exists = path.exists();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .await?;

    match exists {
        true => {
            // Read preferences from file
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(ron::from_str(contents.as_str())?)
        }
        false => {
            // Generate default preferences
            let preferences = models::Preferences::default();
            // Serialize the data
            let export_string = ron::to_string(&preferences)?;
            // Save to file
            file.write_all(export_string.as_bytes()).await?;
            Ok(preferences)
        }
    }
}

/// Save a preferences file to disk
pub async fn save(preferences: models::Preferences) -> Result<(), anyhow::Error> {
    let path = path();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .await?;
    // Serialize the data
    let export_string = ron::to_string(&preferences)?;
    // Save to file
    file.write_all(export_string.as_bytes()).await?;
    Ok(())
}
