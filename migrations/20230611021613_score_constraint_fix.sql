ALTER TABLE osu_score
DROP CONSTRAINT fk_osu_user;

ALTER TABLE osu_score
ADD CONSTRAINT fk_osu_user
FOREIGN KEY (user_id)
REFERENCES osu_user(id)
ON DELETE CASCADE;
