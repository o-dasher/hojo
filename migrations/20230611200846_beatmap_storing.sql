CREATE TABLE osu_beatmap (
    id INT PRIMARY KEY NOT NULL,
    hash CHAR(32) NOT NULL
);

ALTER TABLE osu_score
ADD COLUMN beatmap_id INT;

ALTER TABLE osu_score ADD CONSTRAINT fk_osu_beatmap
FOREIGN KEY (beatmap_id)
REFERENCES osu_beatmap (id);
