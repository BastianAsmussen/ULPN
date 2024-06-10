CREATE TABLE forums (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER REFERENCES forums(id) ON DELETE CASCADE DEFAULT NULL, -- If NULL, this is a top-level forum.

    title VARCHAR(128) NOT NULL UNIQUE,
    description TEXT NOT NULl, -- In markdown format.

    is_locked BOOLEAN NOT NULL DEFAULT TRUE, -- If the forum is locked, no messages can be sent.

    access_level access_level NOT NULL
);

INSERT INTO forums (
    title,
    description,
    access_level
) VALUES (
    'General',
    '
# Grooming or smth

This is an example description of the content. You can provide some context or information about what the video is about.

[![Click to Watch Video](https://img.youtube.com/vi/hZIYSCE-ZjY/0.jpg)](https://www.youtube.com/watch?v=hZIYSCE-ZjY)

[Virklig Cases!](https://www.youtube.com/watch?v=hZIYSCE-ZjY)
[O M G!](https://www.example.com/more_info_link)
',
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

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    1,
    'underforum',
    'pfff!',
    'parent'
);

INSERT INTO forums (
    title,
    description,
    access_level
) VALUES (
    'KEK',
    'asdaasdasd there!',
    'parent'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    2,
    'underforum2',
    'pff2!',
    'parent'
);
