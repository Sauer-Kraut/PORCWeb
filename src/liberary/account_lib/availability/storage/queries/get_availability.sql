SELECT 
    av.start_date AS start_date,
    av.end_date AS end_date,

    av.repetition_type AS repetition_type_code,

    av.config_mon AS config_mon,
    av.config_tue AS config_tue,
    av.config_wed AS config_wed,
    av.config_thu AS config_thu,
    av.config_fri AS config_fri,
    av.config_sat AS config_sat,
    av.config_sun AS config_sun

FROM availabilities av
WHERE av.owner_id = ($1);