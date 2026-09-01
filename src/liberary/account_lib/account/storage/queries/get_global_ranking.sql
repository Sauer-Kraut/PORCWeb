SELECT
  (
    SELECT to_json(a) 
    FROM accounts a 
    WHERE a.id = accs.id
  ) AS account_info,

  (
    SELECT to_json(stats)
    FROM account_stats AS stats
    WHERE stats.account_id = accs.id
  ) AS stats,

  (
    SELECT to_json(
      json_build_object(
        'stone_1', ss1.shiftstone,
        'stone_2', ss2.shiftstone,
        'radar_chart',
          CASE
            WHEN rc.id IS NULL THEN NULL
            ELSE json_build_object(
              'mobility', rc."Mobility",
              'aggresiveness', rc."Aggressiveness",
              'weight', rc."Weight",
              'range', rc."Range",
              'reactivity', rc."Reactivity"
            )
          END,
        'badge_1', b1.badge_id,
        'badge_2', b2.badge_id,
        'badge_3', b3.badge_id,
        'region', reg.id,
        'bp', account_cust.bp,
        'banner', account_cust.banner,
        'unlocked_badges', COALESCE(
          (
            SELECT json_agg(bi.badge_id)
            FROM badge_instances AS bi
            WHERE bi.account_id = accs.id
          ),
          '[]'::json
        )
      )
    )
    FROM account_cust
    LEFT JOIN regions AS reg
      ON account_cust.region = reg.id
    LEFT JOIN radar_charts AS rc
      ON account_cust.radar_chart = rc.id
    LEFT JOIN shiftstones AS ss1
      ON account_cust.stone_1 = ss1.id
    LEFT JOIN shiftstones AS ss2
      ON account_cust.stone_2 = ss2.id
    LEFT JOIN badge_instances AS b1
      ON account_cust.badge_1 = b1.id
    LEFT JOIN badge_instances AS b2
      ON account_cust.badge_2 = b2.id
    LEFT JOIN badge_instances AS b3
      ON account_cust.badge_3 = b3.id
    WHERE account_cust.account_id = accs.id
  ) AS customisation

FROM accounts AS accs
JOIN account_stats AS stats
  ON stats.account_id = accs.id
WHERE stats.global_rank is not NULL
ORDER BY stats.global_rank asc;