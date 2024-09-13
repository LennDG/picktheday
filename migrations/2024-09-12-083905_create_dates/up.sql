CREATE TABLE dates (
    -- Relations
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    
    -- Data
    date TEXT NOT NULL CHECK (length(date) <= 128),
    
    -- Metadata
    ctime TEXT NOT NULL,

    -- Constraints
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, date) -- A date can only be picked once per user
);
CREATE INDEX idx_dates_user_id ON dates(user_id);