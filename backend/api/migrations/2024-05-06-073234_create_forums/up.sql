CREATE TABLE forums (
    id SERIAL PRIMARY KEY,

    title VARCHAR(128) NOT NULL UNIQUE,
    description TEXT NOT NULl, -- In markdown format.

    access_level access_level NOT NULL
);

INSERT INTO forums (
    title,
    description,
    access_level
) VALUES (
    'General',
    'No memes in #general, please!',
    'child'
);

INSERT INTO forums (
    title,
    description,
    access_level
) VALUES (
    'Other',
    'Hello there!',
    'parent'
);
