CREATE TABLE Bookmarks (
    id uuid PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    added_on timestamptz NOT NULL DEFAULT NOW(),
    page INTEGER NOT NULL,
    document uuid NOT NULL,
    deleted_on timestamptz,

    CONSTRAINT fk_bookmarks_document FOREIGN KEY(document) REFERENCES Documents(id)
)