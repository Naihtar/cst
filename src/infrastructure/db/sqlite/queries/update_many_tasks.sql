UPDATE tasks
SET
    priority = COALESCE(:priority, priority),
    status = COALESCE(:status, status)
WHERE id IN (SELECT value FROM json_each(:ids))
RETURNING id;
