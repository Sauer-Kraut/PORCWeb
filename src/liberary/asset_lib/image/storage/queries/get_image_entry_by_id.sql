SELECT id::uuid AS id, object_name, owner_id::text AS owner_id, author, link, filetype, created_at
FROM images
WHERE id = ($1) 