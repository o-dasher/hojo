ALTER TABLE osu_stats 
ADD COLUMN pp DOUBLE PRECISION DEFAULT 0 NOT NULL,
ADD COLUMN total_score BIGINT DEFAULT 0 NOT NULL,
ADD COLUMN ranked_score BIGINT DEFAULT 0 NOT NULL
