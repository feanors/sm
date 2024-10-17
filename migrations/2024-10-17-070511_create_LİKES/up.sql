-- Your SQL goes here

CREATE TABLE likes (
    liked_by UUID NOT NULL,
    liked_post UUID NOT NULL,
    PRIMARY KEY (liked_by, liked_post)
);