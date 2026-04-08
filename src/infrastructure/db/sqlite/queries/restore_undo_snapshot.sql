DELETE FROM tasks;
DELETE FROM sqlite_sequence WHERE name = 'tasks';
INSERT INTO tasks (id, information, priority, status)
    SELECT id, information, priority, status FROM undo_snapshot;
DELETE FROM undo_snapshot;
