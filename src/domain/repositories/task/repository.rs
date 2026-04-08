use crate::prelude::{Err, Filter, NewTask, Priority, Sort, Status, Task};

/// Persistence contract for task storage.
///
/// All mutating methods that support undo should be preceded by [`save_snapshot`](TaskRepository::save_snapshot).
pub trait TaskRepository {
    /// Persists a new task and returns its generated ID.
    fn create(&self, task: &NewTask) -> Result<i64, Err>;

    /// Retrieves a task by ID, or `None` if not found.
    fn read_by_id(&self, id: i64) -> Result<Option<Task>, Err>;

    /// Returns all tasks ordered by `sort`.
    fn read_all(&self, sort: Sort) -> Result<Vec<Task>, Err>;

    /// Returns one page of tasks plus the total count.
    fn read_paged(&self, sort: Sort, page: i64) -> Result<(Vec<Task>, i64), Err>;

    /// Replaces the stored fields of a task and returns its ID.
    fn update(&self, task: &Task) -> Result<i64, Err>;

    /// Applies `priority` and/or `status` to multiple tasks. Returns the updated IDs.
    fn update_many(
        &self,
        ids: &[i64],
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<Vec<i64>, Err>;

    /// Marks multiple tasks as done. Returns the updated IDs.
    fn done_many(&self, ids: &[i64]) -> Result<Vec<i64>, Err>;

    /// Deletes a task by ID and returns it.
    fn delete(&self, id: i64) -> Result<i64, Err>;

    /// Deletes multiple tasks by ID. Returns the deleted IDs.
    fn delete_many(&self, ids: &[i64]) -> Result<Vec<i64>, Err>;

    /// Removes all tasks from storage.
    fn delete_all(&self) -> Result<(), Err>;

    /// Returns tasks matching `filter` and the total count of matching rows.
    fn filter(&self, filter: Filter) -> Result<(Vec<Task>, i64), Err>;

    /// Saves the current state as an undo snapshot.
    fn save_snapshot(&self) -> Result<(), Err>;

    /// Restores the most recent undo snapshot.
    fn restore_snapshot(&self) -> Result<(), Err>;

    /// Returns `true` if an undo snapshot exists.
    fn has_snapshot(&self) -> Result<bool, Err>;
}
