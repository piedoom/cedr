CREATE TABLE IF NOT EXISTS sources (
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL UNIQUE,
    kind                TEXT NOT NULL, -- terms, examples
    url                 TEXT NOT NULL,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- A term is universal across all sources
CREATE TABLE IF NOT EXISTS terms (
    id                  INTEGER PRIMARY KEY NOT NULL,
    traditional         TEXT NOT NULL UNIQUE ON CONFLICT IGNORE,
    simplified          TEXT NOT NULL,
    pinyin              TEXT NOT NULL,
    pinyin_numbers      TEXT NOT NULL,
    pinyin_raw          TEXT NOT NULL,
    tones               TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS history (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at              DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    term_id                 INTEGER NOT NULL,
    FOREIGN KEY(term_id)    REFERENCES terms(id)
);

CREATE TABLE IF NOT EXISTS definitions (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    definition          TEXT NOT NULL,
    hash                INTEGER NOT NULL UNIQUE ON CONFLICT IGNORE,
    term                TEXT NOT NULL,
    source_id           INTEGER NOT NULL,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- FOREIGN KEY(source_id) REFERENCES sources(id),
    FOREIGN KEY(term) REFERENCES terms(traditional)
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
    name                TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS collections_terms (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    collection_id       INTEGER NOT NULL,
    term_id             INTEGER NOT NULL,

    UNIQUE(collection_id, term_id),
    FOREIGN KEY(collection_id) REFERENCES collections(id),
    FOREIGN KEY(term_id) REFERENCES terms(id)
);

-- FTS5 table for easier terms searching

CREATE VIRTUAL TABLE IF NOT EXISTS terms_search USING fts5(simplified, content=terms, content_rowid=id);

CREATE TRIGGER IF NOT EXISTS terms_after_insert AFTER INSERT ON terms
BEGIN
    INSERT INTO terms_search(rowid, simplified) VALUES (new.id, new.simplified);
END;

CREATE TRIGGER IF NOT EXISTS terms_after_update AFTER UPDATE ON terms
BEGIN
    UPDATE terms_search SET simplified = new.simplified WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS terms_after_delete AFTER DELETE ON terms
BEGIN
    DELETE FROM terms_search WHERE rowid = old.id;
END;