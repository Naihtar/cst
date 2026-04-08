use crate::prelude::{
    Builder, DomErr, Err, Filter, Mode, Priority, Prog, Repository, Sort, Status, Task, from_csv,
    from_json, from_toml, from_yaml, to_csv, to_json, to_md, to_toml, to_xls, to_yaml,
};

/// Application service that coordinates task operations through a [`TaskRepository`].
pub struct TaskService<R: Repository> {
    repository: R,
}

impl<R: Repository> TaskService<R> {
    /// Creates a new service backed by `repository`.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Creates and persists a new task. Returns the new task's ID.
    pub fn add_task(
        &self,
        information: String,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<i64, Err> {
        let task = Builder::new()
            .information(information)
            .priority(priority)
            .status(status)
            .build()?;
        self.repository.create(&task)
    }

    /// Returns all tasks ordered by `sort`.
    pub fn list_tasks(&self, sort: Sort) -> Result<Vec<Task>, Err> {
        self.repository.read_all(sort)
    }

    /// Returns one page of tasks and the total count.
    pub fn paged_tasks(&self, sort: Sort, page: i64) -> Result<(Vec<Task>, i64), Err> {
        self.repository.read_paged(sort, page)
    }

    /// Looks up a single task by ID. Returns `None` if not found.
    pub fn find_task_by_id(&self, id: i64) -> Result<Option<Task>, Err> {
        self.repository.read_by_id(id)
    }

    /// Updates an existing task. Saves an undo snapshot first.
    pub fn update_task(
        &self,
        id: i64,
        information: Option<String>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<i64, Err> {
        self.repository.save_snapshot()?;
        let existing = self.repository.read_by_id(id)?.ok_or(DomErr::InvalidID)?;
        let task = Builder::from_task(&existing)
            .information_update(information)
            .priority(priority)
            .status(status)
            .build_with_id()?;
        self.repository.update(&task)
    }

    /// Deletes a task by ID. Saves an undo snapshot first.
    pub fn remove_task(&self, id: i64) -> Result<i64, Err> {
        self.repository.save_snapshot()?;
        self.repository.delete(id)
    }

    /// Deletes all tasks. Saves an undo snapshot first.
    pub fn clear_all_tasks(&self) -> Result<(), Err> {
        self.repository.save_snapshot()?;
        self.repository.delete_all()
    }

    /// Returns tasks matching `filter` and the total matching count.
    pub fn filter_tasks(&self, filter: Filter) -> Result<(Vec<Task>, i64), Err> {
        self.repository.filter(filter)
    }

    /// Restores the last undo snapshot. Errors if none exists.
    pub fn undo(&self) -> Result<(), Err> {
        if !self.repository.has_snapshot()? {
            return Err(DomErr::NothingToUndo)?;
        }
        self.repository.restore_snapshot()
    }

    /// Imports tasks from a CSV file. Returns `(count, elapsed_secs)`.
    pub fn import(&self, path: &str, mode: &Mode) -> Result<(usize, f64), Err> {
        self.import_with(path, mode, from_csv)
    }

    /// Imports tasks from a JSON file. Returns `(count, elapsed_secs)`.
    pub fn import_json(&self, path: &str, mode: &Mode) -> Result<(usize, f64), Err> {
        self.import_with(path, mode, from_json)
    }

    /// Imports tasks from a YAML file. Returns `(count, elapsed_secs)`.
    pub fn import_yaml(&self, path: &str, mode: &Mode) -> Result<(usize, f64), Err> {
        self.import_with(path, mode, from_yaml)
    }

    /// Imports tasks from a TOML file. Returns `(count, elapsed_secs)`.
    pub fn import_toml(&self, path: &str, mode: &Mode) -> Result<(usize, f64), Err> {
        self.import_with(path, mode, from_toml)
    }

    /// Returns the number of tasks that would be imported from a CSV file.
    pub fn import_preview(&self, path: &str) -> Result<usize, Err> {
        self.preview_with(path, from_csv)
    }

    /// Returns the number of tasks that would be imported from a JSON file.
    pub fn import_json_preview(&self, path: &str) -> Result<usize, Err> {
        self.preview_with(path, from_json)
    }

    /// Returns the number of tasks that would be imported from a YAML file.
    pub fn import_yaml_preview(&self, path: &str) -> Result<usize, Err> {
        self.preview_with(path, from_yaml)
    }

    /// Returns the number of tasks that would be imported from a TOML file.
    pub fn import_toml_preview(&self, path: &str) -> Result<usize, Err> {
        self.preview_with(path, from_toml)
    }

    /// Parses `path` with `parse` and returns the task count without persisting.
    fn preview_with<F>(&self, path: &str, parse: F) -> Result<usize, Err>
    where
        F: Fn(&str) -> Result<Vec<Builder>, Err>,
    {
        Ok(parse(path)?.len())
    }

    /// Parses `path` with `parse`, optionally clears existing tasks, then inserts all.
    /// Returns `(count, elapsed_secs)`.
    fn import_with<F>(&self, path: &str, mode: &Mode, parse: F) -> Result<(usize, f64), Err>
    where
        F: Fn(&str) -> Result<Vec<Builder>, Err>,
    {
        let progress = Prog::new();
        let tasks = parse(path)?
            .into_iter()
            .map(|builder| builder.build())
            .collect::<Result<Vec<_>, _>>()?;
        if matches!(mode, Mode::Restore) {
            self.repository.delete_all()?;
        }
        tasks
            .iter()
            .map(|task| self.repository.create(task))
            .collect::<Result<Vec<_>, _>>()?;
        Ok((tasks.len(), progress.elapsed_secs()))
    }

    /// Exports all tasks to CSV. Returns `(count, path, elapsed_secs)`.
    pub fn export_csv(&self, output: Option<String>) -> Result<(usize, String, f64), Err> {
        let tasks = self.repository.read_all(Sort::default())?;
        to_csv(&tasks, output.as_deref())
    }

    /// Exports all tasks to Markdown. Returns `(count, path, elapsed_secs)`.
    pub fn export_markdown(&self, output: Option<String>) -> Result<(usize, String, f64), Err> {
        let tasks = self.repository.read_all(Sort::default())?;
        to_md(&tasks, output.as_deref())
    }

    /// Exports all tasks to an Excel workbook. Returns `(count, path, elapsed_secs)`.
    pub fn export_excel(&self, output: Option<String>) -> Result<(usize, String, f64), Err> {
        let tasks = self.repository.read_all(Sort::default())?;
        to_xls(&tasks, output.as_deref())
    }

    /// Exports all tasks to JSON. Returns `(count, path, elapsed_secs)`.
    pub fn export_json(&self, output: Option<String>) -> Result<(usize, String, f64), Err> {
        let tasks = self.repository.read_all(Sort::default())?;
        to_json(&tasks, output.as_deref())
    }

    /// Exports all tasks to YAML. Returns `(count, path, elapsed_secs)`.
    pub fn export_yaml(&self, output: Option<String>) -> Result<(usize, String, f64), Err> {
        let tasks = self.repository.read_all(Sort::default())?;
        to_yaml(&tasks, output.as_deref())
    }

    /// Exports all tasks to TOML. Returns `(count, path, elapsed_secs)`.
    pub fn export_toml(&self, output: Option<String>) -> Result<(usize, String, f64), Err> {
        let tasks = self.repository.read_all(Sort::default())?;
        to_toml(&tasks, output.as_deref())
    }

    /// Deletes multiple tasks. Returns `(deleted_ids, missing_ids)`.
    pub fn remove_many(&self, ids: Vec<i64>) -> Result<(Vec<i64>, Vec<i64>), Err> {
        self.repository.save_snapshot()?;
        let processed = self.repository.delete_many(&ids)?;
        Ok(partition_results(ids, processed))
    }

    /// Marks multiple tasks as done. Returns `(updated_ids, missing_ids)`.
    pub fn done_many(&self, ids: Vec<i64>) -> Result<(Vec<i64>, Vec<i64>), Err> {
        self.repository.save_snapshot()?;
        let processed = self.repository.done_many(&ids)?;
        Ok(partition_results(ids, processed))
    }

    /// Applies `priority` and/or `status` to multiple tasks. Returns `(updated_ids, missing_ids)`.
    pub fn update_many(
        &self,
        ids: Vec<i64>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<(Vec<i64>, Vec<i64>), Err> {
        self.repository.save_snapshot()?;
        let processed = self.repository.update_many(&ids, priority, status)?;
        Ok(partition_results(ids, processed))
    }
}

/// Splits `ids` into those present in `processed` and those that are missing.
fn partition_results(ids: Vec<i64>, processed: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let missing = ids
        .into_iter()
        .filter(|id| !processed.contains(id))
        .collect();
    (processed, missing)
}
