/// Task priority levels.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Execution status of a task.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Todo,
    InProgress,
    Blocked,
    Done,
}

/// Controls how imported tasks are merged with existing data.
#[derive(Debug)]
pub enum ImportMode {
    /// Adds imported tasks alongside existing ones.
    Append,
    /// Deletes all existing tasks before importing.
    Restore,
    /// Parses the file and reports what would be imported without persisting anything.
    DryRun,
}
