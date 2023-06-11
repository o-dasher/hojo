CREATE TABLE osu_score (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,

    pp DOUBLE PRECISION NOT NULL,
    score BIGINT NOT NULL,

    combo BIGINT NOT NULL,

    perfect BIGINT NOT NULL,
    good BIGINT NOT NULL,
    bad BIGINT NOT NULL,
    miss BIGINT NOT NULL,

    geki BIGINT NOT NULL,
    katu BIGINT NOT NULL,

    date TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_osu_user FOREIGN KEY (user_id) REFERENCES osu_user(id)
)
