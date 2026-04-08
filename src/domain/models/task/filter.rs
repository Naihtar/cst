use super::types::{Priority, Status};

/// Default number of tasks returned per page.
pub const DEFAULT_PAGE_SIZE: i64 = 20;

/// Fields available for sorting a task list.
#[derive(Debug, PartialEq, Clone)]
pub enum SortField {
    Id,
    Status,
    Priority,
}

/// Direction of a sort operation.
#[derive(Debug, PartialEq, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for SortOrder {
    /// Defaults to ascending order.
    fn default() -> Self {
        SortOrder::Asc
    }
}

/// Sorting configuration: which field and in which direction.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Sort {
    pub field: Option<SortField>,
    pub order: SortOrder,
}

/// Query parameters for filtering and paginating the task list.
#[derive(Debug, PartialEq, Clone)]
pub struct Filter {
    /// Optional full-text search term.
    pub word: Option<String>,
    /// Restrict results to this status.
    pub status: Option<Status>,
    /// Restrict results to this priority.
    pub priority: Option<Priority>,
    pub sort: Sort,
    /// Zero-based page index.
    pub page: i64,
    pub page_size: i64,
}

impl Default for Filter {
    /// Returns an unfiltered query for the first page.
    fn default() -> Self {
        Self {
            word: None,
            status: None,
            priority: None,
            sort: Sort::default(),
            page: 0,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}
