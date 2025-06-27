INSERT INTO accounts (id, username, discriminator, avatar, email, schedule_note)
VALUES (($1), ($2), ($3), ($4), ($5), ($6))
ON conflict DO NOTHING