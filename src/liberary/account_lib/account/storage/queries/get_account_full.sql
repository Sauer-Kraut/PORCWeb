SELECT
  to_json(accs) AS account_info,

  COALESCE(
    (  
      SELECT json_agg(avail)
      FROM availabilities AS avail
      WHERE avail.owner_id = accs.id
    ),
    '[]'::json
  ) AS availabilities,

  COALESCE(
    (  
      SELECT json_agg(av_except)
      FROM availability_exceptions AS av_except
      WHERE av_except.owner_id = accs.id
    ),
    '[]'::json
  ) AS availability_exceptions,

  COALESCE(
    (
      SELECT json_agg(
        json_build_object(
          'id', fg.id,
          'event_id', fg.event_id,
          'season', seas.season_name,
          'status_code', fg.status,
          'challenger_id', acc1.id,
          'opponent_id', acc2.id,
          'start_timestamp', fg.date
        )
      )
      FROM matches AS ma
      JOIN fights AS fg
        ON fg.match_id = ma.id
      LEFT JOIN participants AS p1
        ON p1.id = ma.participant_1
      JOIN accounts as acc1
        ON acc1.id = p1.account_id
      JOIN divisions as div
        ON div.id = p1.division_id
      JOIN seasons as seas
        ON seas.season_name = div.season_name
      LEFT JOIN participants AS p2
        ON p2.id = ma.participant_2
      JOIN accounts AS acc2
        ON acc2.id = p2.account_id
      WHERE p1.account_id = accs.id
         OR p2.account_id = accs.id
    ),
    '[]'::json
  ) AS match_events,

  COALESCE(
    (
      SELECT json_build_object(
        'wins', COALESCE(stats.wins, 0),
        'matches', COALESCE(stats.matches, 0),
        'win_ratio', stats.win_ratio,
        'global_rank', stats.global_rank,
        'division_rank', stats.division_rank,
        'pose_stats', stats.pose_stats
      )
      FROM account_stats AS stats
      WHERE stats.account_id = accs.id
    ),
    json_build_object(
      'wins', 0,
      'matches', 0,
      'win_ratio', NULL,
      'global_rank', NULL,
      'division_rank', NULL,
      'pose_stats', NULL
    )
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
WHERE accs.id = ANY(($1))
ORDER BY array_position(($1), accs.id);