use crate::prelude::{Priority, Status};

/// Converts a [`Priority`] variant to its lowercase string representation.
pub fn priority_to_str(priority: &Priority) -> &'static str {
    match priority {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
        Priority::Urgent => "urgent",
    }
}

/// Converts a [`Status`] variant to its lowercase string representation.
pub fn status_to_str(status: &Status) -> &'static str {
    match status {
        Status::Todo => "todo",
        Status::InProgress => "in-progress",
        Status::Blocked => "blocked",
        Status::Done => "done",
    }
}

/// Parses a string into a [`Priority`] variant. Returns `None` if unrecognized.
pub fn str_to_priority(s: &str) -> Option<Priority> {
    match s.trim().to_lowercase().as_str() {
        "low" => Some(Priority::Low),
        "medium" => Some(Priority::Medium),
        "high" => Some(Priority::High),
        "urgent" => Some(Priority::Urgent),
        _ => None,
    }
}

/// Parses a string into a [`Status`] variant. Returns `None` if unrecognized.
///
/// Accepts both `"in-progress"` and `"inprogress"`.
pub fn str_to_status(s: &str) -> Option<Status> {
    match s.trim().to_lowercase().as_str() {
        "todo" => Some(Status::Todo),
        "in-progress" | "inprogress" => Some(Status::InProgress),
        "blocked" => Some(Status::Blocked),
        "done" => Some(Status::Done),
        _ => None,
    }
}
