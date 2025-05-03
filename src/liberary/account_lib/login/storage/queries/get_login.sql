SELECT 
    a.id AS id,
    l.created_at AS created_at

FROM discord_logins l
JOIN accounts a ON l.user_id = a.id

WHERE l.auth_key = ($1)
LIMIT 1;