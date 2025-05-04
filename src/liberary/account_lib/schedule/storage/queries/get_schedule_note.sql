SELECT
    a.schedule_note AS schedule_note

FROM accounts a
WHERE a.id = ($1)
LIMIT 1;