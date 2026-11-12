SELECT id::uuid AS id, object_name, owner_id::text AS owner_id, author, link, filetype, created_at
FROM images
WHERE owner_id IS NULL OR owner_id = ($1)
ORDER BY owner_id DESC
LIMIT ($2)