CREATE TYPE access_level AS ENUM (
    'child', 
    'parent',
    'professional',
    'administrator'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    unilogin CHAR(8) NOT NULL UNIQUE,
    full_name VARCHAR(1024) NOT NULL,
    
    access_level access_level NOT NULL
);

CREATE INDEX unilogin ON users (unilogin);

