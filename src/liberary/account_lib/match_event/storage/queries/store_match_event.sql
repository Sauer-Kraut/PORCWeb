INSERT INTO fights (event_id, date, match_id, status, challenger)
VALUES (($1), ($2), ((
        SELECT id FROM matches WHERE
            (participant_2 = (
                SELECT id FROM participants WHERE
                    account_id = ($4)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )
            )
            AND participant_1 = (
                SELECT id FROM participants WHERE
                    account_id = ($5)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )
            )) OR 
            (participant_1 = (
                SELECT id FROM participants WHERE
                    account_id = ($4)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )
            )
            AND participant_2 = (
                SELECT id FROM participants WHERE
                    account_id = ($5)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )
            ))
        LIMIT 1
    )
    ),($3), (SELECT p.id FROM participants p WHERE p.account_id = ($4) AND p.division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($6)
                    )))
ON conflict (match_id, date) DO update
SET
    event_id = EXCLUDED.event_id,
    status = EXCLUDED.status;