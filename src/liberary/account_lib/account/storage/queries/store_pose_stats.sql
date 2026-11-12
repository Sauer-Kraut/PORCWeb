INSERT INTO account_stats (account_id, pose_stats)
VALUES (($1), ($2))
ON conflict (account_id) DO update
SET
  pose_stats = EXCLUDED.pose_stats
