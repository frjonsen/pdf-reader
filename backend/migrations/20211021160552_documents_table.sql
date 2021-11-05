CREATE TABLE Documents (
    id uuid PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    added_on timestamptz NOT NULL
)