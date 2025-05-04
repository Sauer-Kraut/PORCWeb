DELETE FROM availabilities
WHERE 
    owner_id = ($1)
    AND start_date = ($2)
    AND end_date = ($3)
    AND repetition_type = ($4);