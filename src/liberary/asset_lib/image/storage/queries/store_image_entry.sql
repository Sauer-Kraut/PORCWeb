INSERT INTO images (id, object_name, owner_id, author, link, filetype)
VALUES (($1), ($2), ($3), ($4), ($5), ($6))
ON conflict (id) DO nothing