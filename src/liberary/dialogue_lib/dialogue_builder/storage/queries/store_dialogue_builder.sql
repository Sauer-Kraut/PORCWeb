INSERT INTO dialogues (user_id, dialogue_data, error, last_updated, index)
VALUES (($1), (2$)::jsonb, (3$), NOW()::timestamptz, ($4));