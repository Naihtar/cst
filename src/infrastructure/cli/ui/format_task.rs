use crate::prelude::{
    Priority, Status, Store, Task, {BRIGHT_RED, CYAN, GREEN, RED, RESET, WHITE, YELLOW},
};

pub const COL_ID: usize = 6;
pub const COL_PRIORITY: usize = 12;
pub const COL_STATUS: usize = 12;
/// Total width consumed by " | " separators across column boundaries.
const COL_SEPARATORS: usize = 12;

/// Returns the dynamic task column width based on the current terminal width.
///
/// Falls back to 80 columns if the terminal size cannot be determined.
pub fn col_task() -> usize {
    terminal_size::terminal_size()
        .map_or(80, |(w, _)| w.0 as usize)
        .saturating_sub(COL_ID + COL_PRIORITY + COL_STATUS + COL_SEPARATORS)
}

/// Returns the ANSI color code for the given priority level.
fn priority_color(priority: &Priority) -> &'static str {
    match priority {
        Priority::Low => WHITE,
        Priority::Medium => YELLOW,
        Priority::High => RED,
        Priority::Urgent => BRIGHT_RED,
    }
}

/// Returns the ANSI color code for the given status.
fn status_color(status: &Status) -> &'static str {
    match status {
        Status::Todo => WHITE,
        Status::InProgress => CYAN,
        Status::Done => GREEN,
        Status::Blocked => RED,
    }
}

/// Returns the localized display string for the given priority.
fn format_priority(priority: &Priority) -> String {
    match priority {
        Priority::Low => Store::t("table.priority_low"),
        Priority::Medium => Store::t("table.priority_medium"),
        Priority::High => Store::t("table.priority_high"),
        Priority::Urgent => Store::t("table.priority_urgent"),
    }
}

/// Returns the localized display string for the given status.
fn format_status(status: &Status) -> String {
    match status {
        Status::Todo => Store::t("table.status_todo"),
        Status::InProgress => Store::t("table.status_in_progress"),
        Status::Done => Store::t("table.status_done"),
        Status::Blocked => Store::t("table.status_blocked"),
    }
}

/// Wraps text into lines of at most `max` characters, breaking on whitespace.
fn wrap(s: &str, max: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_len = 0;

    for word in s.split_whitespace() {
        let word_len = word.chars().count();

        if current_line.is_empty() {
            current_line.push_str(word);
            current_len = word_len;
        } else if current_len + 1 + word_len <= max {
            current_line.push(' ');
            current_line.push_str(word);
            current_len += 1 + word_len;
        } else {
            lines.push(std::mem::take(&mut current_line));
            current_line.push_str(word);
            current_len = word_len;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Formats a task as one or more table rows, wrapping long information text.
pub fn format_task(task: &Task) -> String {
    let width = col_task();
    let lines = wrap(task.information(), width);

    let priority_label = format_priority(task.priority());
    let status_label = format_status(task.status());

    // Format strings with ANSI colors for the first line
    let priority_colored = format!(
        "{}{:<width$}{}",
        priority_color(task.priority()),
        priority_label,
        RESET,
        width = COL_PRIORITY
    );
    let status_colored = format!(
        "{}{:<width$}{}",
        status_color(task.status()),
        status_label,
        RESET,
        width = COL_STATUS
    );

    // Build the first line
    let first = format!(
        "{:<id_w$} | {:<task_w$} | {} | {} ",
        format!("{:03}", task.id()),
        lines[0],
        priority_colored,
        status_colored,
        id_w = COL_ID,
        task_w = width
    );

    if lines.len() == 1 {
        return first;
    }

    // Build continuation lines for wrapped text
    let continuation = lines[1..].iter().map(|line| {
        format!(
            "{:<id_w$} | {:<task_w$} | {:<prio_w$} | {:<status_w$} ",
            "",
            line,
            "",
            "",
            id_w = COL_ID,
            task_w = width,
            prio_w = COL_PRIORITY,
            status_w = COL_STATUS
        )
    });

    std::iter::once(first)
        .chain(continuation)
        .collect::<Vec<_>>()
        .join("\n")
}
