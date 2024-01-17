-- [MIT License] Copyright (c) 2024 Michel Novus 
-- DDL file: defines database tables and relationships
-- Target to SQLite 3.7.15 or newer (Python SQLite API)

/*
    The "documents" table defines the identification data (name and version) 
    and informational data (insertion date) of documentation files.
*/
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,  -- Semver format: "x.y.z"
    insertion_date TEXT NOT NULL,  -- ISO8601 format: "YYYY-MM-DD HH:MM:SS.SSS"
    id_file TEXT REFERENCES files (hashsum) ON DELETE CASCADE
);

/*
    The "files" table contains the files as binaries and data associated 
    (filesize and type) with them.
*/
CREATE TABLE IF NOT EXISTS files (
    hashsum TEXT PRIMARY KEY,  -- BLAKE3 hashsum of file BLOB
    file BLOB NOT NULL,
    type TEXT NOT NULL,  -- type of file: tarball, pdf, executable, etc 
    size INTEGER NOT NULL  -- size of file in bytes
);
