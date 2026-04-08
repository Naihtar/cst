DELETE FROM undo_snapshot;
INSERT INTO undo_snapshot SELECT id, information, priority, status FROM tasks;
