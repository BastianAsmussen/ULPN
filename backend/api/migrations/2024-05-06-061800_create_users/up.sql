CREATE TYPE access_level AS ENUM (
    'anonymous',    -- Smallest access level (not logged in).
    'child', 
    'parent',
    'professional'
);

CREATE TABLE users (
    id           BIGSERIAL PRIMARY KEY,

    unilogin     CHAR(8)      NOT NULL UNIQUE,
    access_level access_level NOT NULL,
    
    is_admin     BOOLEAN      NOT NULL DEFAULT FALSE
);

CREATE INDEX unilogin ON users (unilogin);

