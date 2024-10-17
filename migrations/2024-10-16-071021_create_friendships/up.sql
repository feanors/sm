-- Your SQL goes here

CREATE TABLE friendships (
    user1 UUID NOT NULL,
    user2 UUID NOT NULL,
    PRIMARY KEY (user1, user2),
    CONSTRAINT symmetry CHECK (user1 < user2),
    FOREIGN KEY (user1) REFERENCES users(id),
    FOREIGN KEY (user2) REFERENCES users(id)
);