use crate::prelude::{DomErr, Err, Priority, Status};

/// Data required to create a new task (no ID yet).
#[derive(Debug, PartialEq)]
pub struct NewTask {
    pub information: String,
    pub priority: Priority,
    pub status: Status,
}

/// A persisted task with a unique ID.
#[derive(Debug, PartialEq)]
pub struct Task {
    id: i64,
    information: String,
    priority: Priority,
    status: Status,
}

impl Task {
    /// Returns the task's unique ID.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns the task description.
    pub fn information(&self) -> &str {
        &self.information
    }

    /// Returns the task priority.
    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    /// Returns the task status.
    pub fn status(&self) -> &Status {
        &self.status
    }
}

/// Builder for constructing [`NewTask`] or [`Task`] with validation.
pub struct TaskBuilder {
    information: String,
    id: Option<i64>,
    priority: Priority,
    status: Status,
}

impl Default for TaskBuilder {
    /// Defaults to low priority and todo status.
    fn default() -> Self {
        Self {
            information: String::new(),
            id: None,
            priority: Priority::Low,
            status: Status::Todo,
        }
    }
}

impl TaskBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialises the builder from an existing task.
    pub fn from_task(task: &Task) -> Self {
        Self {
            information: task.information().to_string(),
            id: Some(task.id),
            priority: *task.priority(),
            status: *task.status(),
        }
    }

    /// Sets the task ID.
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the task description.
    pub fn information(mut self, info: String) -> Self {
        self.information = info;
        self
    }

    /// Replaces the description only if `info` is `Some`.
    pub fn information_update(mut self, info: Option<String>) -> Self {
        if let Some(value) = info {
            self.information = value;
        }
        self
    }

    /// Replaces the priority only if `prio` is `Some`.
    pub fn priority(mut self, prio: Option<Priority>) -> Self {
        if let Some(value) = prio {
            self.priority = value;
        }
        self
    }

    /// Replaces the status only if `status` is `Some`.
    pub fn status(mut self, status: Option<Status>) -> Self {
        if let Some(value) = status {
            self.status = value;
        }
        self
    }

    /// Returns an error if the description is blank or whitespace-only.
    fn validate_information(information: &str) -> Result<(), DomErr> {
        if information.trim().is_empty() {
            return Err(DomErr::EmptyTaskInformation);
        }
        Ok(())
    }

    /// Builds a [`NewTask`]. Fails if the description is empty.
    pub fn build(self) -> Result<NewTask, Err> {
        Self::validate_information(&self.information)?;
        Ok(NewTask {
            information: self.information,
            priority: self.priority,
            status: self.status,
        })
    }

    /// Builds a [`Task`] with an ID. Fails if the description is empty or no ID was set.
    pub fn build_with_id(self) -> Result<Task, Err> {
        Self::validate_information(&self.information)?;
        Ok(Task {
            id: self.id.ok_or(DomErr::InvalidID)?,
            information: self.information,
            priority: self.priority,
            status: self.status,
        })
    }
}
