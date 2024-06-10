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

INSERT INTO users (
    full_name,
    email,
    access_level
) VALUES (
    'Martin Buus',
    'mbuus87@gmail.com',
    'administrator'
);

INSERT INTO users (
    full_name,
    email,
    access_level
) VALUES (
    'Emma Marie Hansen',
    'emmamariehansen@outlook.com',
    'administrator'
);

INSERT INTO users (
    full_name,
    email,
    access_level
) VALUES (
    'Frederik Juulolsen',
    'frederikjuulolsen@gmail.com',
    'professional'
);

INSERT INTO users (
    full_name,
    email,
    access_level
) VALUES (
    'Bambi',
    'bambiergratis@gmail.com',
    'professional'
);
