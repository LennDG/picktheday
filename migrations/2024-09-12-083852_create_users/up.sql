CREATE TABLE users (
    -- Relations
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    public_id VARCHAR(32) NOT NULL, -- ID to use on the web, don't expose autoincrements
    plan_id INTEGER NOT NULL,
    name VARCHAR(128) NOT NULL,

    -- Metadata
    ctime TIMESTAMPTZ NOT NULL,

    -- Constraints
    UNIQUE(plan_id, name),
    FOREIGN KEY(plan_id) REFERENCES plans(id) ON DELETE CASCADE
);

-- Create an index on plan_id for better performance when querying
CREATE INDEX idx_users_plan_id ON users(plan_id);