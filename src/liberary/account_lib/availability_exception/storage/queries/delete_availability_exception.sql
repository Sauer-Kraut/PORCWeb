DELETE FROM availability_exceptions
WHERE 
    owner_id = ($1)
    AND start_date = ($2)
    AND end_date = ($3)