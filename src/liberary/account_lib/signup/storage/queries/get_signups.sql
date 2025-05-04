SELECT
    a.username AS username,
    a.id AS account_id,

    s.created_at AS created_at,
    s.region AS region,
    s.bp AS bp

FROM signups s 
JOIN accounts a ON s.account_id = a.id
WHERE s.created_at > ($1)::timestamptz
AND s.created_at < ($2)::timestamptz;