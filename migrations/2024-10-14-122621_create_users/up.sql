-- Your SQL goes here

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL
);