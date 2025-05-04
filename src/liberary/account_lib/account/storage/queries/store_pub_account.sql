INSERT INTO accounts (id, username, avatar, schedule_note)
VALUES (($1), ($2), ($3), ($4))
ON conflict (id) DO update
SET
    username = EXCLUDED.username,
    avatar = EXCLUDED.avatar,
    schedule_note = EXCLUDED.schedule_note