INSERT INTO availabilities (owner_id, start_date, end_date, repetition_type, config_mon, config_tue, config_wed, config_thu, config_fri, config_sat, config_sun)
VALUES (1, $2::timestamptz, $3::timestamptz, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (owner_id, start_date, end_date, repetition_type) DO UPDATE
SET
    start_date = EXCLUDED.start_date,
    end_date = EXCLUDED.end_date,
    repetition_type = EXCLUDED.repetition_type,
    config_mon = EXCLUDED.config_mon,
    config_tue = EXCLUDED.config_tue,
    config_wed = EXCLUDED.config_wed,
    config_thu = EXCLUDED.config_thu,
    config_fri = EXCLUDED.config_fri,
    config_sat = EXCLUDED.config_sat,
    config_sun = EXCLUDED.config_sun;