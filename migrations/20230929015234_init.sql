CREATE TABLE IF NOT EXISTS sources (
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL UNIQUE,
    url                 TEXT NOT NULL,
    license             TEXT,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS entries (
    id                          INTEGER PRIMARY KEY NOT NULL,
    traditional                 TEXT NOT NULL,
    simplified                  TEXT NOT NULL,
    pinyin                      TEXT NOT NULL,
    pinyin_numbers              TEXT NOT NULL,
    pinyin_raw                  TEXT NOT NULL,
    definition                  TEXT NOT NULL,
    tones                       TEXT NOT NULL,
    updated_at                  DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    source_id                   INTEGER NOT NULL,

    UNIQUE(traditional, simplified, pinyin_numbers) ON CONFLICT REPLACE,
    FOREIGN KEY(source_id)      REFERENCES sources(id) ON UPDATE CASCADE
);


CREATE TABLE IF NOT EXISTS history (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at              DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    entry_id                INTEGER NOT NULL,

    FOREIGN KEY(entry_id)    REFERENCES entries(id)
);


CREATE TABLE IF NOT EXISTS examples (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    text                TEXT NOT NULL,
    translations        TEXT NOT NULL,

    source_id              INTEGER NOT NULL,
    FOREIGN KEY(source_id) REFERENCES sources(id)      
);

-- Used to save terms into for later reference (like card decks)
CREATE TABLE IF NOT EXISTS collections (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name                TEXT UNIQUE NOT NULL,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS collections_entries (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    collection_id       INTEGER NOT NULL,
    entry_id            INTEGER NOT NULL,

    UNIQUE(collection_id, entry_id),
    FOREIGN KEY(collection_id) REFERENCES collections(id),
    FOREIGN KEY(entry_id) REFERENCES entries(id)
);

-- -- FTS5 table for easier terms searching

-- CREATE VIRTUAL TABLE IF NOT EXISTS examples_search USING fts5(simplified, content=terms, content_rowid=id);

-- CREATE TRIGGER IF NOT EXISTS terms_after_insert AFTER INSERT ON terms
-- BEGIN
--     INSERT INTO terms_search(rowid, simplified) VALUES (new.id, new.simplified);
-- END;

-- CREATE TRIGGER IF NOT EXISTS terms_after_update AFTER UPDATE ON terms
-- BEGIN
--     UPDATE terms_search SET simplified = new.simplified WHERE rowid = old.id;
-- END;

-- CREATE TRIGGER IF NOT EXISTS terms_after_delete AFTER DELETE ON terms
-- BEGIN
--     DELETE FROM terms_search WHERE rowid = old.id;
-- END;