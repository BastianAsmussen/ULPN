CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    
    title VARCHAR(128) NOT NULL,
    content TEXT NOT NULL
);

