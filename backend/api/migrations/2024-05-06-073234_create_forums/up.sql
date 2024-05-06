CREATE TABLE forums (
    id SERIAL PRIMARY KEY,

    name VARCHAR(256) NOT NULL UNIQUE,
    access_level access_level NOT NULL
);

