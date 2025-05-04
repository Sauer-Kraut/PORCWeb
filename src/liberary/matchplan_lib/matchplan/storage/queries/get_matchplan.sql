SELECT 
a1.username AS player_1_tag,
a1.id AS player_1_id,
m.score_1 AS player_1_score,

a2.username AS player_2_tag,
a2.id AS player_2_id,
m.score_2 AS player_2_score,

d.division_name AS division_name,
d.division_order AS division_order,

s.end_date AS season_end_timestamp,
s.pause_end_date AS season_pause_end_timestamp

FROM matches m 
JOIN participants p1 ON p1.id = m.participant_1
JOIN participants p2 ON p2.id = m.participant_2
JOIN accounts a1 ON p1.account_id = a1.id
JOIN accounts a2 ON p2.account_id = a2.id
JOIN divisions d ON p1.division_id = d.id
JOIN seasons s ON s.season_name = d.season_name

WHERE d.season_name = ($1)
ORDER BY d.division_order ASC;