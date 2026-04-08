CREATE TABLE IF NOT EXISTS undo_snapshot (
    id          INTEGER PRIMARY KEY,
    information TEXT NOT NULL,
    priority    INTEGER NOT NULL,
    status      INTEGER NOT NULL
);
