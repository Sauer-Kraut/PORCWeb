SELECT 
    d.season_name AS season,

    f.id AS id,
    f.status AS status_code,
    f.event_id AS event_id,
    f.date AS start_timestamp,

    a1.id AS challenger_id,
    a2.id AS opponent_id

FROM accounts a
JOIN participants p ON a.id = p.account_id
JOIN divisions d ON p.division_id = d.id
JOIN matches m ON p.id = m.participant_1 OR p.id = m.participant_2
JOIN fights f ON f.match_id = m.id

JOIN participants pCha ON pCha.id = f.challenger
JOIN participants pOpp ON (pOpp.id = m.participant_1 AND pOpp.id != pCha.id) OR (pOpp.id = m.participant_2 AND pOpp.id != pCha.id)

JOIN accounts a1 ON a1.id = pCha.account_id
JOIn accounts a2 ON a2.id = pOpp.account_id

WHERE a.id = ($1);