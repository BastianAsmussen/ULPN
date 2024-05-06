CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY,

    forum_id SERIAL REFERENCES forums(id),
    sender_id SERIAL REFERENCES users(id),

    reply_id BIGSERIAL REFERENCES messages(id), -- Is this a reply to something?

    identity SERIAL REFERENCES identities(id),
    contents TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

