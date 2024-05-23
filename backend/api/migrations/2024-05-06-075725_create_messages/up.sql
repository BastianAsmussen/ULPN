CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY,

    forum_id SERIAL REFERENCES forums(id) ON DELETE CASCADE,
    sender_id SERIAL REFERENCES users(id) ON DELETE CASCADE,

    reply_id BIGSERIAL REFERENCES messages(id) ON DELETE CASCADE, -- Is this a reply to something?

    identity_id SERIAL REFERENCES identities(id) ON DELETE CASCADE,
    contents TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE, -- Can others see this yet?

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
