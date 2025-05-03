INSERT INTO accounts (id, username, discriminator, avatar, email, schedule_note)
VALUES ($1, $2, $3, $4, $5, $6)
ON conflict (id) DO update
SET
    username = EXCLUDED.username,
    discriminator = EXCLUDED.discriminator,
    avatar = EXCLUDED.avatar,
    email = EXCLUDED.email,
    schedule_note = EXCLUDED.schedule_note