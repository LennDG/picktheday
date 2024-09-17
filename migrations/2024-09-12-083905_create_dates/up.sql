CREATE TABLE dates (
    -- Relations
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL,

    -- Data
    date DATE NOT NULL,

    -- Metadata
    ctime TIMESTAMPTZ NOT NULL,

    -- Constraints
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, date) -- A date can only be picked once per user
);

-- Create an index on user_id for better performance when querying
CREATE INDEX idx_dates_user_id ON dates(user_id);