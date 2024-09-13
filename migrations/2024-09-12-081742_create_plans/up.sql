CREATE TABLE plans (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    public_id TEXT NOT NULL UNIQUE CHECK (length(public_id) <= 32),
    name TEXT NOT NULL CHECK (length(name) <= 128),
    description TEXT CHECK (length(description) <= 1024),

    -- Metadata
    ctime TEXT NOT NULL
);