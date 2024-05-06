CREATE TYPE access_level AS ENUM (
    'anonymous',    -- Smallest access level (not logged in).
    'child', 
    'parent',
    'professional'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    unilogin CHAR(8) NOT NULL UNIQUE,
    full_name VARCHAR(1024) NOT NULL,
    
    access_level access_level NOT NULL,
    
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX unilogin ON users (unilogin);

