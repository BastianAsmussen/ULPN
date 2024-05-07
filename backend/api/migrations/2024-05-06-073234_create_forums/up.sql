CREATE TABLE forums (
    id SERIAL PRIMARY KEY,

    title VARCHAR(128) NOT NULL UNIQUE,
    description TEXT NOT NULl, -- In markdown format.

    access_level access_level NOT NULL
);

