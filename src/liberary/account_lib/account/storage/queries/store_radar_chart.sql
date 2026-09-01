BEGIN;

WITH upsert AS (
  INSERT INTO public.radar_charts (
    account_id,
    "Mobility",
    "Weight",
    "Aggressiveness",
    "Range",
    "Reactivity"
  )
  VALUES (
    ($1)::text,
    ($2)::double precision,
    ($3)::double precision,
    ($4)::double precision,
    ($5)::double precision,
    ($6)::double precision
  )
  ON CONFLICT (account_id) DO UPDATE
  SET
    "Mobility"       = EXCLUDED."Mobility",
    "Weight"         = EXCLUDED."Weight",
    "Aggressiveness" = EXCLUDED."Aggressiveness",
    "Range"          = EXCLUDED."Range",
    "Reactivity"     = EXCLUDED."Reactivity"
  RETURNING id
)
UPDATE public.account_cust ac
SET radar_chart = u.id
FROM upsert u
WHERE ac.id = $1::text;

COMMIT;