SELECT 
    av_exc.start_date AS start_date,
    av_exc.end_date AS end_date

FROM availability_exceptions av_exc
WHERE av_exc.owner_id = ($1);