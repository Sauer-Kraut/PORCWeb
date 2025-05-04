SELECT 
    f.id AS id,
    f.event_id AS event_id,
    f.date AS start_timestamp,
    f.status AS status_code,

    a1.id AS challenger_id,
    a2.id AS opponent_id,

    d.season_name AS season

FROM fights f
JOIN matches m ON f.match_id = m.id
JOIN participants p ON m.participant_1 = p.id OR m.participant_2 = p.id
JOIN divisions d ON p.division_id = d.id

JOIN participants pCha ON pCha.id = f.challenger
JOIN participants pOpp ON (pOpp.id = m.participant_1 AND pOpp.id != pCha.id) OR (pOpp.id = m.participant_2 AND pOpp.id != pCha.id)

JOIN accounts a1 ON a1.id = pCha.account_id
JOIn accounts a2 ON a2.id = pOpp.account_id

WHERE f.id = ($1)
LIMIT 1;