DELETE FROM tasks
WHERE id IN (SELECT value FROM json_each(:ids))
RETURNING id;
