INSERT INTO fights (event_id, date, match_id, status)
VALUES (($1), ($2), ((
        SELECT id FROM matches WHERE
            challenger = (
                SELECT id FROM participants WHERE
                    account_id = ($4)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )
            )
            AND opponent = (
                SELECT id FROM participants WHERE
                    account_id = ($5)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )
            )
        LIMIT 1
    )
    ),($3))
ON conflict (match_id, date) DO update
SET
    event_id = EXCLUDED.event_id,
    status = EXCLUDED.status;