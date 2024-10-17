-- Your SQL goes here

CREATE TABLE posts (
    id UUID PRIMARY KEY NOT NULL,
    posted_by UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (posted_by) REFERENCES users(id)
);