CREATE TYPE access_level AS ENUM (
    'child',
    'parent',
    'professional',
    'administrator'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    full_name VARCHAR(1024) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,

    access_level access_level NOT NULL
);

CREATE INDEX email_idx ON users(email);
