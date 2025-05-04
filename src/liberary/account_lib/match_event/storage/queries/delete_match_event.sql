DELETE FROM fights
WHERE
    date = ($1)::timestamptz
    AND match_id = (
        SELECT id FROM matches WHERE
            challenger = (
                SELECT id FROM participants WHERE
                    account_id = ($2)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($4)
                    )
            )
            AND opponent = (
                SELECT id FROM participants WHERE
                    account_id = ($3)
                    AND division_id IN (
                        SELECT id FROM divisions WHERE
                            season_name = ($4)
                    )
            )
        LIMIT 1
    );