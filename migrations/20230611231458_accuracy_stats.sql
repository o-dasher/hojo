ALTER TABLE osu_user DROP COLUMN accuracy;
ALTER TABLE osu_stats ADD COLUMN accuracy DOUBLE PRECISION NOT NULL DEFAULT 100;