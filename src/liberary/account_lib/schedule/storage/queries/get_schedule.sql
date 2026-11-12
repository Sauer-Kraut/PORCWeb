SELECT

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
  ) AS match_events


FROM accounts AS accs
WHERE accs.id = ANY(($1))
ORDER BY array_position(($1), accs.id);