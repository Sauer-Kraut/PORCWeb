INSERT INTO dialogues (id, user_id, dialogue_data, error, last_updated, index)
VALUES (($1), ($2), ($3)::jsonb, ($4), NOW()::timestamptz, ($5))
ON CONFLICT (id) DO UPDATE
SET
    error = EXCLUDED.error,
    dialogue_data = EXCLUDED.dialogue_data,
    last_updated = EXCLUDED.last_updated,
    index = EXCLUDED.index;