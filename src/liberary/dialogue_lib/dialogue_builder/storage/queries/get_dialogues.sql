SELECT 
    d.id AS id,
    d.created_at AS created_at,
    d.user_id AS user_id,
    d.dialogue_data AS dialogue_data,
    d.error AS error,
    d.index AS index

FROM dialogues d
WHERE d.last_updated < ($2)::timestamptz AND index != 600 AND index != 400
ORDER BY d.last_updated desc
LIMIT ($1);