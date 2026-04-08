// SQL queries embedded at compile time from the `queries/` directory.

pub const CREATE_TABLE_TASKS: &str = include_str!("queries/create_tasks.sql");
pub const INSERT_TASK: &str = include_str!("queries/insert_task.sql");
pub const SELECT_TASK_BY_ID: &str = include_str!("queries/select_task_by_id.sql");
pub const SELECT_ALL_TASKS: &str = include_str!("queries/select_all_tasks.sql");
pub const UPDATE_TASK: &str = include_str!("queries/update_task.sql");
pub const DELETE_TASK: &str = include_str!("queries/delete_task.sql");
pub const FILTER_TASKS: &str = include_str!("queries/filter_tasks.sql");
pub const DELETE_ALL_TASKS: &str = include_str!("queries/delete_all_tasks.sql");
pub const SELECT_PAGED_TASKS: &str = include_str!("queries/select_paged_tasks.sql");
pub const DELETE_MANY_TASKS: &str = include_str!("queries/delete_many_tasks.sql");
pub const DONE_MANY_TASKS: &str = include_str!("queries/done_many_tasks.sql");
pub const UPDATE_MANY_TASKS: &str = include_str!("queries/update_many_tasks.sql");
pub const CREATE_TABLE_UNDO_SNAPSHOT: &str = include_str!("queries/create_undo_snapshot.sql");
pub const SAVE_UNDO_SNAPSHOT: &str = include_str!("queries/save_undo_snapshot.sql");
pub const RESTORE_UNDO_SNAPSHOT: &str = include_str!("queries/restore_undo_snapshot.sql");
pub const HAS_UNDO_SNAPSHOT: &str = include_str!("queries/has_undo_snapshot.sql");
