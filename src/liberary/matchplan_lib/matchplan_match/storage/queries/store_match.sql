DO $$
DECLARE
    affected_id bigint;
BEGIN
    UPDATE matches SET score_challenger = ($3), score_opponent = ($4)
    WHERE challenger = (

        SELECT id FROM participants WHERE account_id = ($1) AND division_id in (

            SELECT id FROM divisions WHERE season_name = ($5)
        )
        LIMIT 1
    )
    AND opponent = (

        SELECT id FROM participants WHERE account_id = ($2) AND division_id in (

            SELECT id FROM divisions WHERE season_name = ($5)
        )
        LIMIT 1
    )
    RETURNING id INTO affected_id;

    IF affected_id IS NULL THEN
        RAISE EXCEPTION 'No match found for the given participants and season.' USING ERRCODE = 'no_data_found';
    END IF;
END $$;