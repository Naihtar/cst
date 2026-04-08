INSERT INTO tasks (information, priority, status)
VALUES (:information, :priority, :status)
RETURNING id;
