select
    s.season_name AS season_name,
    s.start_date AS start_date,
    s.end_date AS end_date,
    s.pause_end_date AS pause_end_date

FROM seasons s
WHERE s.season_name = ($1)
LIMIT 1;