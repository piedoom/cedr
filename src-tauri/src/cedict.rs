//! Manage the dictionary
use hash32::Hasher;
use sqlx::{Pool, QueryBuilder, Sqlite};
use std::hash::Hash;

use anyhow::{anyhow, Context};
use libflate::gzip::Decoder;
use std::{
    fs::{File, OpenOptions},
    io::{self, Read},
    path::Path,
};

pub const CEDICT_FILENAME: &str = "cedict.txt";
pub const CEDICT_DOWNLOAD_URL: &str =
    "http://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz";
pub const CEDICT_NAME: &str = "cedict";
const SQLITE_MAX_ARGS: usize = 32766;

/// Download the newest version of CEDICT
pub async fn download_newest_cedict(
    url: impl AsRef<str>,
    path: impl AsRef<Path>,
) -> Result<(), anyhow::Error> {
    let resp = reqwest::get(url.as_ref()).await?.bytes().await?;
    let mut decoder = Decoder::new(resp.as_ref())?;
    let mut out = File::create(Path::new(path.as_ref()))?;
    io::copy(&mut decoder, &mut out)?;
    Ok(())
}

/// Parse and read the `.txt` file at a location into the SQL database
pub async fn load_cedict_dictionary_file(
    path: impl AsRef<Path>,
    pool: &Pool<Sqlite>,
) -> anyhow::Result<()> {
    struct TempTerm {
        traditional: String,
        simplified: String,
        pinyin: String,
        pinyin_numbers: String,
        pinyin_raw: String,
        tones: String,
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .context("no dictionary file found at path")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut terms: Vec<TempTerm> = Vec::new();
    let mut definitions: Vec<(String, String)> = Vec::new();

    // Split into lines
    for line in buf.split("\r\n") {
        // Check if a meta line
        match line.chars().take(2).collect::<String>().as_str() {
            // Ignore these lines
            "# " => {}
            // Check these lines for meta info
            "#!" => {
                // split at the first space to reteive the KV pair
                let pair = line
                    .split_once(' ')
                    .ok_or(anyhow::anyhow!(
                        "Invalid metadata format: no space delimiter found for KV pair"
                    ))?
                    .1;
                // split at `=`
                let (k, v) = pair
                    .split_once('=')
                    .ok_or(anyhow!("Invalid metadata format: no `=` found for KV pair"))?;

                // TODO: match and deserialize meta information
                //                 if let "time" = k {
                //                     let time = str::parse::<i64>(v)?;
                //                     sqlx::query!(
                //                         r"
                // INSERT INTO sources ( name, kind, url, updated_at )
                // VALUES ( ?1, ?2, ?3, ?4 )",
                //                         CEDICT_NAME,
                //                         "terms",
                //                         CEDICT_DOWNLOAD_URL,
                //                         time,
                //                     )
                //                     .execute(&pool)
                //                     .await?;
                //                 }
            }
            // Read as dictionary entries
            _ => {
                // For now, eliminate all '·' chars in proper nouns as they break our stuff
                let line: String = line.split('·').collect();

                // Split into components with a space
                let components: Vec<&str> = line.splitn(3, ' ').collect();

                let (traditional, simplified, remainder) = {
                    (
                        components
                            .first()
                            .ok_or(anyhow!(format!("missing simplified characters: {}", line)))?
                            .to_string(),
                        components
                            .get(1)
                            .ok_or(anyhow!(format!("missing traditional characters: {}", line)))?
                            .to_string(),
                        components
                            .get(2)
                            .ok_or(anyhow!(format!("missing components: {}", line)))?
                            .to_string(),
                    )
                };

                let (pinyin_numbers, definitions_string) = {
                    // Skip the initial `[` mark, and the trailing `/`
                    let res = remainder[1..remainder.len() - 1]
                        .split_once("] /")
                        .ok_or(anyhow!("missing components"))?;
                    (res.0.to_string(), res.1.to_string().replace('/', "\n"))
                };

                // Pinyin "currently1 looks5 like3 this2"
                let (pinyin, tones): (String, Vec<u8>) = {
                    (
                        prettify_pinyin::prettify(&pinyin_numbers),
                        pinyin_numbers
                            .split_whitespace()
                            .map(|pinyin_part| {
                                str::parse::<u8>(&pinyin_part[pinyin_part.len() - 1..])
                                    // If parsing a number failed, it is probably a non-chinese char, and for that we use tone 5
                                    .unwrap_or(5)
                            })
                            .collect::<Vec<_>>(),
                    )
                };
                let mut pinyin_raw = pinyin_numbers.clone();
                pinyin_raw.retain(|c| !r"12345".contains(c));

                let term = TempTerm {
                    traditional: traditional.clone(),
                    simplified,
                    pinyin,
                    pinyin_numbers,
                    pinyin_raw,
                    tones: tones.iter().map(ToString::to_string).collect(),
                };
                terms.push(term);

                // further split definitions_string into separate definitions split by a newline
                for def in definitions_string.lines() {
                    definitions.push((traditional.clone(), def.to_string()));
                }
            }
        }
    }

    // split builder into manageable chunks so we don't get an error
    for chunk in terms.chunks(SQLITE_MAX_ARGS / 6) {
        let mut builder = QueryBuilder::<Sqlite>::new(
                "INSERT INTO terms ( traditional, simplified, pinyin, pinyin_numbers, pinyin_raw, tones ) ",
            );
        builder.push_values(chunk, |mut b, term| {
            //let hash =
            b.push_bind(&term.traditional)
                .push_bind(&term.simplified)
                .push_bind(&term.pinyin)
                .push_bind(&term.pinyin_numbers)
                .push_bind(&term.pinyin_raw)
                .push_bind(&term.tones);
        });
        builder.build().execute(pool).await?;
    }

    for chunk in definitions.chunks(SQLITE_MAX_ARGS / 4) {
        let mut builder = QueryBuilder::<Sqlite>::new(
            "INSERT INTO definitions ( term, source_id, definition, hash ) ",
        );
        builder.push_values(chunk, |mut b, entry| {
            b.push_bind(&entry.0)
                .push_bind(0)
                .push_bind(&entry.1)
                .push_bind({
                    let mut hasher = hash32::Murmur3Hasher::default();
                    format!("{}{}{}", entry.0, 0, entry.1).hash(&mut hasher);
                    hasher.finish32()
                });
        });
        builder.build().execute(pool).await?;
    }

    Ok(())
}

pub(crate) async fn build_dictionary(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let filename = Path::new(&dotenv::var("DATA_DIR")?).join(CEDICT_FILENAME);
    if !filename.exists() {
        download_newest_cedict(CEDICT_DOWNLOAD_URL, &filename).await?;
    }
    load_cedict_dictionary_file(filename, pool).await
}
