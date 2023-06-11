-- Add migration script here
ALTER TABLE osu_user
ADD last_seen TIMESTAMP DEFAULT NOW()
