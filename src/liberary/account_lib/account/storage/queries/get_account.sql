SELECT 
    a.id AS id,
    a.username As username,
    a.discriminator AS discriminator,
    a.avatar AS avatar,
    a.email AS email,
    a.schedule_note AS schedule_note

FROM accounts a
WHERE a.id = ($1)
LIMIT 1;