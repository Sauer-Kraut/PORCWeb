WITH
params AS (
  SELECT
    ($1)::text AS account_id,

    ($2)::text AS region_name, -- nullable (e.g. "EU")
    ($3)::int8 AS bp,          -- nullable

    ($4)::text  AS stone_1_str,
    ($5)::text  AS stone_2_str,

    ($6)::int8  AS badge_1_badge_id,
    ($7)::int8  AS badge_2_badge_id,
    ($8)::int8  AS badge_3_badge_id
),

region_map AS (
  SELECT r.id, r.region
  FROM public.regions r
  WHERE r.region = (SELECT region_name FROM params)
),

stone_map AS (
  SELECT s.id, s.shiftstone
  FROM public.shiftstones s
  WHERE s.shiftstone IN (
    SELECT stone_1_str FROM params WHERE stone_1_str IS NOT NULL
    UNION ALL
    SELECT stone_2_str FROM params WHERE stone_2_str IS NOT NULL
  )
),

badge_map AS (
  SELECT bi.id, bi.badge_id
  FROM public.badge_instances bi
  WHERE bi.account_id = (SELECT account_id FROM params)
    AND bi.badge_id IN (
      SELECT badge_1_badge_id FROM params WHERE badge_1_badge_id IS NOT NULL
      UNION ALL
      SELECT badge_2_badge_id FROM params WHERE badge_2_badge_id IS NOT NULL
      UNION ALL
      SELECT badge_3_badge_id FROM params WHERE badge_3_badge_id IS NOT NULL
    )
),

assign AS (
  SELECT
    (SELECT rm.id FROM region_map rm LIMIT 1) AS region_id,
    p.bp,

    (SELECT sm.id FROM stone_map sm WHERE sm.shiftstone = p.stone_1_str LIMIT 1) AS stone_1_id,
    (SELECT sm.id FROM stone_map sm WHERE sm.shiftstone = p.stone_2_str LIMIT 1) AS stone_2_id,

    (SELECT bm.id FROM badge_map bm WHERE bm.badge_id = p.badge_1_badge_id LIMIT 1) AS badge_1_id,
    (SELECT bm.id FROM badge_map bm WHERE bm.badge_id = p.badge_2_badge_id LIMIT 1) AS badge_2_id,
    (SELECT bm.id FROM badge_map bm WHERE bm.badge_id = p.badge_3_badge_id LIMIT 1) AS badge_3_id
  FROM params p
)

UPDATE public.account_cust ac
SET
  region  = a.region_id, -- if $2 is NULL, this becomes NULL
  bp      = a.bp,

  stone_1 = a.stone_1_id,
  stone_2 = a.stone_2_id,

  badge_1 = a.badge_1_id,
  badge_2 = a.badge_2_id,
  badge_3 = a.badge_3_id
FROM assign a
WHERE ac.account_id = (SELECT account_id FROM params);