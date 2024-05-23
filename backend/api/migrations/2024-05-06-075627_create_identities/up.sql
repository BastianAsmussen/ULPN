CREATE TABLE identities (
    id SERIAL PRIMARY KEY,

    owner_id SERIAL REFERENCES users(id) ON DELETE CASCADE,
    forum_id SERIAL REFERENCES forums(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL
);
