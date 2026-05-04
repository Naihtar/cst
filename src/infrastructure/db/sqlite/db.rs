use crate::{
    infrastructure::db::sqlite::{
        mappers::row_to_task,
        queries::{
            CREATE_TABLE_TASKS, CREATE_TABLE_UNDO_SNAPSHOT, DELETE_ALL_TASKS, DELETE_MANY_TASKS,
            DELETE_TASK, DONE_MANY_TASKS, FILTER_TASKS, HAS_UNDO_SNAPSHOT, INSERT_TASK,
            RESTORE_UNDO_SNAPSHOT, SAVE_UNDO_SNAPSHOT, SELECT_ALL_TASKS, SELECT_PAGED_TASKS,
            SELECT_TASK_BY_ID, UPDATE_MANY_TASKS, UPDATE_TASK,
        },
    },
    prelude::{
        CSTError, DEFAULT_PAGE_SIZE, Filter, NewTask, Priority, Sort, Status, Task, TaskRepository,
    },
};
use rusqlite::{Connection, Row, named_params};

/// SQLite-backed implementation of [`TaskRepository`].
pub struct SqliteTaskRepository {
    conn: Connection,
}

impl SqliteTaskRepository {
    /// Opens (or creates) the SQLite database at `path` and initializes the schema.
    pub fn new(path: &str) -> Result<Self, CSTError> {
        let conn = Connection::open(path).map_err(CSTError::from)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Creates the `tasks` and `undo_snapshot` tables if they do not exist.
    fn init_schema(&self) -> Result<(), CSTError> {
        self.conn
            .execute(CREATE_TABLE_TASKS, [])
            .map_err(CSTError::from)?;
        self.conn
            .execute(CREATE_TABLE_UNDO_SNAPSHOT, [])
            .map(|_| ())
            .map_err(CSTError::from)
    }

    /// Serializes a slice of IDs into a JSON array string for use with `json_each`.
    fn ids_to_json(ids: &[i64]) -> Result<String, CSTError> {
        serde_json::to_string(ids).map_err(CSTError::from)
    }

    /// Extracts the first column as an `i64` from a query row.
    fn row_to_id(row: &Row) -> rusqlite::Result<i64> {
        row.get::<_, i64>(0)
    }
}

impl TaskRepository for SqliteTaskRepository {
    /// Inserts a new task and returns its generated ID.
    fn create(&self, task: &NewTask) -> Result<i64, CSTError> {
        self.conn
            .prepare(INSERT_TASK)?
            .query_row(
                named_params! {
                    ":information": task.information,
                    ":priority": task.priority,
                    ":status": task.status,
                },
                Self::row_to_id,
            )
            .map_err(CSTError::from)
    }

    /// Returns the task with the given ID, or `None` if not found.
    fn read_by_id(&self, id: i64) -> Result<Option<Task>, CSTError> {
        self.conn
            .prepare(SELECT_TASK_BY_ID)?
            .query_and_then(named_params! { ":id": id }, row_to_task)?
            .next()
            .transpose()
    }

    /// Returns all tasks ordered by the given sort criteria.
    fn read_all(&self, sort: Sort) -> Result<Vec<Task>, CSTError> {
        self.conn
            .prepare(SELECT_ALL_TASKS)?
            .query_and_then(
                named_params! {
                    ":sort_by": sort.field.as_ref(),
                    ":sort_order": sort.order,
                },
                row_to_task,
            )?
            .collect()
    }

    /// Returns a page of tasks along with the total count of all tasks.
    fn read_paged(&self, sort: Sort, page: i64) -> Result<(Vec<Task>, i64), CSTError> {
        let mut total = 0i64;
        let tasks = self
            .conn
            .prepare(SELECT_PAGED_TASKS)?
            .query_and_then(
                named_params! {
                    ":sort_by": sort.field.as_ref(),
                    ":sort_order": sort.order,
                    ":page": page,
                    ":page_size": DEFAULT_PAGE_SIZE,
                },
                |row| {
                    total = row.get(4)?;
                    row_to_task(row)
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok((tasks, total))
    }

    /// Returns tasks matching the given filter along with the total count.
    fn filter(&self, filter: Filter) -> Result<(Vec<Task>, i64), CSTError> {
        let mut total = 0i64;
        let tasks = self
            .conn
            .prepare(FILTER_TASKS)?
            .query_and_then(
                named_params! {
                    ":word": filter.word,
                    ":status": filter.status,
                    ":priority": filter.priority,
                    ":sort_by": filter.sort.field.as_ref(),
                    ":sort_order": filter.sort.order,
                    ":page": filter.page,
                    ":page_size": filter.page_size,
                },
                |row| {
                    total = row.get(4)?;
                    row_to_task(row)
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok((tasks, total))
    }

    /// Updates an existing task and returns its ID.
    fn update(&self, task: &Task) -> Result<i64, CSTError> {
        self.conn
            .prepare(UPDATE_TASK)?
            .query_row(
                named_params! {
                    ":id": task.id(),
                    ":information": task.information(),
                    ":priority": task.priority(),
                    ":status": task.status(),
                },
                Self::row_to_id,
            )
            .map_err(CSTError::from)
    }

    /// Deletes the task with the given ID and returns it.
    fn delete(&self, id: i64) -> Result<i64, CSTError> {
        self.conn
            .prepare(DELETE_TASK)?
            .query_row(named_params! { ":id": id }, Self::row_to_id)
            .map_err(CSTError::from)
    }

    /// Deletes multiple tasks by ID and returns the deleted IDs.
    ///
    /// Returns an empty vector if `ids` is empty.
    fn delete_many(&self, ids: &[i64]) -> Result<Vec<i64>, CSTError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids_json = Self::ids_to_json(ids)?;
        self.conn
            .prepare(DELETE_MANY_TASKS)?
            .query_and_then(named_params! { ":ids": ids_json }, Self::row_to_id)?
            .collect::<Result<Vec<i64>, _>>()
            .map_err(CSTError::from)
    }

    /// Deletes all tasks and resets the auto-increment sequence.
    fn delete_all(&self) -> Result<(), CSTError> {
        self.conn
            .execute_batch(DELETE_ALL_TASKS)
            .map_err(CSTError::from)
    }

    /// Marks multiple tasks as done and returns the updated IDs.
    ///
    /// Returns an empty vector if `ids` is empty.
    fn done_many(&self, ids: &[i64]) -> Result<Vec<i64>, CSTError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids_json = Self::ids_to_json(ids)?;
        self.conn
            .prepare(DONE_MANY_TASKS)?
            .query_and_then(named_params! { ":ids": ids_json }, Self::row_to_id)?
            .collect::<Result<Vec<i64>, _>>()
            .map_err(CSTError::from)
    }

    /// Updates priority and/or status for multiple tasks and returns the updated IDs.
    ///
    /// `None` values leave the corresponding field unchanged. Returns an empty
    /// vector if `ids` is empty.
    fn update_many(
        &self,
        ids: &[i64],
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<Vec<i64>, CSTError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids_json = Self::ids_to_json(ids)?;
        self.conn
            .prepare(UPDATE_MANY_TASKS)?
            .query_and_then(
                named_params! {
                    ":ids": ids_json,
                    ":priority": priority,
                    ":status": status,
                },
                Self::row_to_id,
            )?
            .collect::<Result<Vec<i64>, _>>()
            .map_err(CSTError::from)
    }

    // ── Undo ──────────────────────────────────────────────────────────────────

    /// Replaces the undo snapshot with the current state of the tasks table.
    fn save_snapshot(&self) -> Result<(), CSTError> {
        self.conn
            .execute_batch(SAVE_UNDO_SNAPSHOT)
            .map_err(CSTError::from)
    }

    /// Restores the tasks table from the undo snapshot and clears it.
    fn restore_snapshot(&self) -> Result<(), CSTError> {
        self.conn
            .execute_batch(RESTORE_UNDO_SNAPSHOT)
            .map_err(CSTError::from)
    }

    /// Returns `true` if an undo snapshot is available.
    fn has_snapshot(&self) -> Result<bool, CSTError> {
        let count: i64 = self
            .conn
            .prepare(HAS_UNDO_SNAPSHOT)?
            .query_row([], |row| row.get(0))
            .map_err(CSTError::from)?;
        Ok(count > 0)
    }
}
