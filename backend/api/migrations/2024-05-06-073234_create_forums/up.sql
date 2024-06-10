CREATE TABLE forums (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER REFERENCES forums(id) ON DELETE CASCADE DEFAULT NULL, -- If NULL, this is a top-level forum.

    title VARCHAR(128) NOT NULL UNIQUE,
    description TEXT NOT NULl, -- In markdown format.

    is_locked BOOLEAN NOT NULL DEFAULT TRUE, -- If the forum is locked, no messages can be sent.

    access_level access_level NOT NULL
);

-- Grooming.
INSERT INTO forums (
    title,
    description,
    access_level
) VALUES (
    'Grroming',
    '''# Hvad er grooming?

    Brødtekst er længere, løbende tekst. Før i tiden blev typografernes løn beregnet ud fra antallet af linjer de havde sat. Brødteksten var hurtig at sætte og det var hovedsageligt det som gav sætteren penge til det daglige brød. Brødtekst kaldes også brødsats eller brødskrift.''',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    1,
    'Virkelige Sager',
    '''# Virkelige Sager
    Her er der et par virkelige sager.

    - [Første](http://example.com/case-1)
    - [Anden](http://example.com/case-2)
    - [Tredje](http://example.com/case-3)''',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    1,
    'Podcasts',
    '''# Podcasts
    Her er der et par podcasts.

    - [Første](http://example.com/podcast-1)
    - [Anden](http://example.com/podcast-2)
    - [Tredje](http://example.com/podcast-3)''',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    1,
    'Hvad kan man gøre?',
    '# Hvad kan man gøre?',
    'child'
);

-- Online Gaming
INSERT INTO forums (
    title,
    description,
    access_level
) VALUES (
    'Online Gaming',
    '''# Hvad er Online Gaming

    Brødtekst er længere, løbende tekst. Før i tiden blev typografernes løn beregnet ud fra antallet af linjer de havde sat. Brødteksten var hurtig at sætte og det var hovedsageligt det som gav sætteren penge til det daglige brød. Brødtekst kaldes også brødsats eller brødskrift.''',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    5,
    'E-Sport',
    '# E-Sport',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    5,
    'Skin Trade',
    '# Skin Trade',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level
) VALUES (
    5,
    'Spil Udvikling',
    '# Spil Udvikling',
    'child'
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level,
    is_locked
) VALUES (
    8,
    'Roblox',
    '# Roblox Spil Udvikling',
    'child',
    false
);

INSERT INTO forums (
    owner_id,
    title,
    description,
    access_level,
    is_locked
) VALUES (
    8,
    'Minecraft',
    '# Minecraft Spil Udvikling',
    'child',
    false
);
