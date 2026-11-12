INSERT INTO availabilities (owner_id, start_date, end_date)
VALUES (($1), ($2)::timestamptz, ($3)::timestamptz)
ON CONFLICT (owner_id, start_date, end_date) DO UPDATE
SET
    start_date = EXCLUDED.start_date,
    end_date = EXCLUDED.end_date;