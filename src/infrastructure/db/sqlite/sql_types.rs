use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

use crate::prelude::{Priority, SortField, SortOrder, Status};

impl ToSql for Priority {
    /// Stores [`Priority`] as an integer: Low=0, Medium=1, High=2, Urgent=3.
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let val = match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
            Priority::Urgent => 3,
        };
        Ok(ToSqlOutput::from(val))
    }
}

impl FromSql for Priority {
    /// Reads [`Priority`] from an integer column.
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(Priority::Low),
            1 => Ok(Priority::Medium),
            2 => Ok(Priority::High),
            3 => Ok(Priority::Urgent),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Status {
    /// Stores [`Status`] as an integer: Todo=0, InProgress=1, Blocked=2, Done=3.
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let val = match self {
            Status::Todo => 0,
            Status::InProgress => 1,
            Status::Blocked => 2,
            Status::Done => 3,
        };
        Ok(ToSqlOutput::from(val))
    }
}

impl FromSql for Status {
    /// Reads [`Status`] from an integer column.
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            0 => Ok(Status::Todo),
            1 => Ok(Status::InProgress),
            2 => Ok(Status::Blocked),
            3 => Ok(Status::Done),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for SortField {
    /// Stores [`SortField`] as a text column name for use in dynamic ORDER BY clauses.
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let val = match self {
            SortField::Id => "id",
            SortField::Priority => "priority",
            SortField::Status => "status",
        };
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(val.as_bytes())))
    }
}

impl ToSql for SortOrder {
    /// Stores [`SortOrder`] as `"asc"` or `"desc"` for use in dynamic ORDER BY clauses.
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let val = match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        };
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(val.as_bytes())))
    }
}
