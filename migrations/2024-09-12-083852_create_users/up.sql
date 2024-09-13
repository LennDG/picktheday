CREATE TABLE users (
    -- Relations
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    public_id TEXT NOT NULL, -- ID to use on the web, don't expose autoincrements
    plan_id INTEGER NOT NULL,
    name TEXT NOT NULL CHECK (length(name) <= 128),

    -- Metadata
    ctime TEXT NOT NULL,

    -- Constraints
    UNIQUE(plan_id, name),
    FOREIGN KEY(plan_id) REFERENCES plans(id) ON DELETE CASCADE

);
CREATE INDEX idx_users_plan_id ON users(plan_id);