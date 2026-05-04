use crate::{
    infrastructure::cli::ui::messages::confirm::Decision,
    prelude::{
        CSTError, CliErr, ConfigCommand, DEFAULT_PAGE_SIZE, Filter, Priority, Sort, SortField,
        SortOrder, Status,
    },
};

/// Maps a priority shorthand character to a [`Priority`] variant.
///
/// `l`=Low, `m`=Medium, `h`=High, `u`=Urgent.
pub fn char_to_priority(c: char) -> Option<Priority> {
    match c {
        'l' => Some(Priority::Low),
        'm' => Some(Priority::Medium),
        'h' => Some(Priority::High),
        'u' => Some(Priority::Urgent),
        _ => None,
    }
}

/// Maps a status shorthand character to a [`Status`] variant.
///
/// `t`=Todo, `w`=InProgress, `b`=Blocked, `d`=Done.
pub fn char_to_status(c: char) -> Option<Status> {
    match c {
        't' => Some(Status::Todo),
        'w' => Some(Status::InProgress),
        'b' => Some(Status::Blocked),
        'd' => Some(Status::Done),
        _ => None,
    }
}

/// Maps a sort field shorthand character to a [`SortField`] variant.
///
/// `p`=Priority, `s`=Status, `i`=Id.
pub fn char_to_sort_field(c: char) -> Option<SortField> {
    match c {
        'p' => Some(SortField::Priority),
        's' => Some(SortField::Status),
        'i' => Some(SortField::Id),
        _ => None,
    }
}

/// Maps a sort order character to a [`SortOrder`] variant.
///
/// `+`=Asc, `-`=Desc.
pub fn char_to_sort_order(c: char) -> Option<SortOrder> {
    match c {
        '+' => Some(SortOrder::Asc),
        '-' => Some(SortOrder::Desc),
        _ => None,
    }
}

/// Maps a config shorthand character to a [`ConfigCommand`] skeleton.
///
/// `l`=SetLanguage, `d`=SetDB.
pub fn char_to_config(c: char) -> Option<ConfigCommand> {
    match c {
        'l' => Some(ConfigCommand::SetLanguage(String::new())),
        'd' => Some(ConfigCommand::SetDB(String::new())),
        _ => None,
    }
}

/// Extracts modifier characters from a CLI flag string.
///
/// Skips everything up to and including the uppercase command letter,
/// then collects alphabetic chars and `+`/`-` signs.
pub fn extract_modifiers(flag: &str) -> Vec<char> {
    flag.chars()
        .skip_while(|c| !c.is_ascii_uppercase())
        .skip(1)
        .filter(|c| c.is_ascii_alphabetic() || matches!(c, '+' | '-'))
        .collect()
}

/// Returns the first [`Priority`] found in the modifier list, if any.
pub fn modifiers_to_priority(modifiers: &[char]) -> Option<Priority> {
    modifiers.iter().find_map(|&c| char_to_priority(c))
}

/// Returns the first [`Status`] found in the modifier list, if any.
pub fn modifiers_to_status(modifiers: &[char]) -> Option<Status> {
    modifiers.iter().find_map(|&c| char_to_status(c))
}

/// Builds a [`Sort`] from the modifier list. Defaults to ascending order.
pub fn modifiers_to_sort(modifiers: &[char]) -> Sort {
    Sort {
        field: modifiers.iter().find_map(|&c| char_to_sort_field(c)),
        order: modifiers
            .iter()
            .find_map(|&c| char_to_sort_order(c))
            .unwrap_or(SortOrder::Asc),
    }
}

/// Builds a [`Filter`] from modifiers, an optional search word, and a page number.
pub fn modifiers_to_filter(modifiers: &[char], word: Option<String>, page: i64) -> Filter {
    Filter {
        word,
        priority: modifiers_to_priority(modifiers),
        status: modifiers_to_status(modifiers),
        sort: modifiers_to_sort(modifiers),
        page,
        page_size: DEFAULT_PAGE_SIZE,
    }
}

/// Parses a string as an `i64` task ID.
pub fn parse_id(arg: &str) -> Result<i64, CSTError> {
    arg.parse::<i64>()
        .map_err(|_| CSTError::Cli(CliErr::InvalidIdFormat))
}

/// Parses a confirmation string into a [`Decision`].
///
/// Accepts `y/yes/s/si/sí` as Yes and `n/no` as No.
pub fn accept(input: &str) -> Result<Decision, CSTError> {
    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" | "s" | "si" | "sí" => Ok(Decision::Yes),
        "n" | "no" => Ok(Decision::No),
        _ => Err(CliErr::InvalidConfirmation(input.to_string()))?,
    }
}

/// Parses a single confirmation character into a [`Decision`], if recognized.
pub fn accept_flag(c: char) -> Option<Decision> {
    match c.to_ascii_lowercase() {
        'y' | 's' => Some(Decision::Yes),
        'n' => Some(Decision::No),
        _ => None,
    }
}

/// Parses a comma-separated string of IDs into a `Vec<i64>`.
pub fn parse_ids(arg: &str) -> Result<Vec<i64>, CSTError> {
    arg.split(',').map(|s| parse_id(s.trim())).collect()
}
