UPDATE tasks
SET status = 3
WHERE id IN (SELECT value FROM json_each(:ids))
RETURNING id;
