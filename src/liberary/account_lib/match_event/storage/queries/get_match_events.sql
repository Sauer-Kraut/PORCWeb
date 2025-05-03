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
JOIN matches m ON p.id = m.challenger OR p.id = m.opponent
JOIN fights f ON f.match_id = m.id

JOIN participants pCha ON pCha.id = m.challenger
JOIN participants pOpp ON pOpp.id = m.opponent

JOIN accounts a1 ON a1.id = pCha.account_id
JOIn accounts a2 ON a2.id =pOpp.account_id

WHERE a.id = ($1);