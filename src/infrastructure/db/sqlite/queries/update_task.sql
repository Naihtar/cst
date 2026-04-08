UPDATE tasks
SET information = :information,
    priority = :priority,
    status = :status
WHERE id = :id
RETURNING id;
