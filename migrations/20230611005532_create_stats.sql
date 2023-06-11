CREATE TABLE osu_stats (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT UNIQUE NOT NULL,

    play_count BIGINT NOT NULL DEFAULT 0,

    CONSTRAINT fk_osu_user FOREIGN KEY (user_id)
    REFERENCES osu_user(id) ON DELETE CASCADE
);
