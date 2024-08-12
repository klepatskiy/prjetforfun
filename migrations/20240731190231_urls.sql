-- Add migration script here
CREATE TABLE urls (
    id UUID PRIMARY KEY,
    url_full TEXT NOT NULL,
    url_short TEXT NOT NULL UNIQUE,
    user_id UUID NULL,
    created_at TIMESTAMP NOT NULL
);
