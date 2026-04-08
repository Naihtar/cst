//! CST – Command-line task manager.
//!
//! Organized in three layers: `domain`, `application`, and `infrastructure`.

pub(crate) mod application {
    //! Use-case services that orchestrate domain logic.
    pub(crate) mod services {
        pub(crate) mod task {
            pub(crate) mod service;
            #[cfg(test)]
            pub mod tests;
        }
    }
}

pub(crate) mod config {
    //! Application configuration and language settings.
    pub(crate) mod language;
    pub(crate) mod settings;
}

pub(crate) mod domain {
    //! Core business models, types, and repository traits.
    pub(crate) mod models {
        pub(crate) mod task {
            pub(crate) mod entity;
            pub(crate) mod filter;
            #[cfg(test)]
            pub mod tests;
            pub(crate) mod types;
        }
    }
    pub(crate) mod repositories {
        pub(crate) mod task {
            pub(crate) mod repository;
        }
    }
}

pub(crate) mod infrastructure {
    //! Adapters for the CLI, SQLite database, and file I/O.
    pub(crate) mod db {
        //! SQLite persistence layer.
        pub(crate) mod sqlite {
            pub(crate) mod db;
            pub(crate) mod mappers;
            pub(crate) mod queries;
            pub(crate) mod sql_types;
            #[cfg(test)]
            pub mod tests;
        }
    }
    pub(crate) mod cli {
        //! Command-line interface: parsing, handlers, and UI rendering.
        pub(crate) mod ui {
            //! Terminal output helpers: colors, formatting, and messages.
            pub(crate) mod colors;
            pub(crate) mod format_list;
            pub(crate) mod format_task;
            pub(crate) mod messages {
                pub(crate) mod confirm;
                pub(crate) mod error;
                pub(crate) mod export;
                pub(crate) mod help;
                pub(crate) mod import;
                pub(crate) mod task;
            }
        }
        pub(crate) mod commands;
        pub(crate) mod config_handler;
        pub(crate) mod handlers;
        pub(crate) mod interactor;
        pub(crate) mod mappers;
        pub(crate) mod parser;
        #[cfg(test)]
        pub mod tests;
    }
    pub(crate) mod io {
        //! File import and export for multiple formats.
        pub(crate) mod interactor;
        pub(crate) mod mappers;
        pub(crate) mod progress;
        pub(crate) mod types;
        pub(crate) mod export {
            //! Writers for CSV, JSON, YAML, TOML, Markdown, and Excel.
            pub(crate) mod common;
            pub(crate) mod csv;
            pub(crate) mod excel;
            pub(crate) mod json;
            pub(crate) mod markdown;
            #[cfg(test)]
            pub mod tests;
            pub(crate) mod toml;
            pub(crate) mod yaml;
        }
        pub(crate) mod import {
            //! Parsers for CSV, JSON, YAML, and TOML task files.
            pub(crate) mod common;
            pub(crate) mod csv;
            pub(crate) mod json;
            #[cfg(test)]
            pub mod tests;
            pub(crate) mod toml;
            pub(crate) mod yaml;
        }
        #[cfg(test)]
        pub mod tests;
    }
}

pub mod app;
pub(crate) mod error;

pub mod prelude {

    // ── Domain ────────────────────────────────────────────────────────────────
    pub use crate::domain::{
        models::task::{
            entity::{NewTask, Task, TaskBuilder as Builder},
            filter::{
                DEFAULT_PAGE_SIZE, Filter, Sort, SortField,
                SortField::Id,
                SortOrder,
                SortOrder::{Asc, Desc},
            },
            types::{ImportMode as Mode, Priority, Status},
        },
        repositories::task::repository::TaskRepository as Repository,
    };

    // ── Application ───────────────────────────────────────────────────────────
    pub use crate::application::services::task::service::TaskService as Service;

    // ── Config ────────────────────────────────────────────────────────────────
    pub use crate::config::{language::Language as Lang, settings::Settings as Store};

    // ── Errors ────────────────────────────────────────────────────────────────
    pub use crate::error::{
        CSTError as Err, CliError as CliErr, ConfigError as ConfigErr, DomainError as DomErr,
        IoError as IOErr,
    };

    // ── Infrastructure – DB ───────────────────────────────────────────────────
    pub use crate::infrastructure::db::sqlite::{
        db::SqliteTaskRepository as SqliteTR,
        mappers::row_to_task,
        queries::{
            CREATE_TABLE_TASKS, CREATE_TABLE_UNDO_SNAPSHOT, DELETE_ALL_TASKS, DELETE_MANY_TASKS,
            DELETE_TASK, DONE_MANY_TASKS, FILTER_TASKS, HAS_UNDO_SNAPSHOT, INSERT_TASK,
            RESTORE_UNDO_SNAPSHOT, SAVE_UNDO_SNAPSHOT, SELECT_ALL_TASKS, SELECT_PAGED_TASKS,
            SELECT_TASK_BY_ID, UPDATE_MANY_TASKS, UPDATE_TASK,
        },
    };

    // ── Infrastructure – CLI ──────────────────────────────────────────────────
    pub use crate::infrastructure::cli::{
        commands::{ConfigCommand as CfgCmd, TaskCommand as Cmd},
        config_handler::{handle_config as config, handle_help as help, handle_version as version},
        handlers::{CliHandler as Handler, CliOutput as Output},
        interactor::{
            ask_confirmation_clear as prompt_clear, ask_confirmation_import as prompt_import,
            ask_confirmation_remove as prompt_remove,
            ask_confirmation_remove_many as prompt_remove_many,
            ask_confirmation_restore as prompt_restore,
        },
        mappers::{
            accept, accept_flag, char_to_config as to_config_char,
            char_to_priority as to_priority_char, char_to_sort_field as to_sort_field,
            char_to_sort_order as to_sort_order, char_to_status as to_status_char,
            extract_modifiers as get_modifiers, modifiers_to_filter as to_filter,
            modifiers_to_priority as to_priority, modifiers_to_sort as to_sort,
            modifiers_to_status as to_status, parse_id, parse_ids,
        },
        parser::CliParser as Parser,
    };

    // ── Infrastructure – UI ───────────────────────────────────────────────────
    pub use crate::infrastructure::cli::ui::{
        colors::{BRIGHT_RED, CYAN, GREEN, RED, RESET, WHITE, YELLOW},
        format_list::format_task_list as list_view,
        format_task::{
            COL_ID as ID, COL_PRIORITY as PRIORITY, COL_STATUS as STATUS, col_task as task_width,
            format_task as task_view,
        },
        messages::{
            confirm::{
                Decision, format_confirm_clear as confirm_clear,
                format_confirm_import as confirm_import, format_confirm_remove as confirm_remove,
                format_confirm_remove_many as confirm_remove_many,
                format_confirm_restore as confirm_restore,
            },
            error::{
                format_error as err_msg, format_success as ok_msg, format_warning as warn_msg,
            },
            export::{
                format_export as export_msg, format_paged_tasks as paged_msg,
                format_pagination as pagination_msg, format_tasks as tasks_msg,
            },
            help::{
                format_config as config_msg, format_help as help_msg, format_version as version_msg,
            },
            import::{format_import as import_msg, format_import_preview as preview_msg},
            task::{
                format_cancelled_msg as cancelled_msg, format_cleared as cleared_msg,
                format_done as done_msg, format_many_result as many_msg,
                format_task_created as created_msg, format_task_deleted as deleted_msg,
                format_task_found as found_msg, format_task_updated as updated_msg,
            },
        },
    };

    // ── Infrastructure – IO ───────────────────────────────────────────────────
    pub use crate::infrastructure::io::{
        export::{
            common::export, csv::export_csv as to_csv, excel::export_excel as to_xls,
            json::export_json as to_json, markdown::export_markdown as to_md,
            toml::export_toml as to_toml, yaml::export_yaml as to_yaml,
        },
        import::{
            common::check_and_parse, csv::import_csv as from_csv, json::import_json as from_json,
            toml::import_toml as from_toml, yaml::import_yaml as from_yaml,
        },
        interactor::ask_filename as prompt_filename,
        mappers::{priority_to_str, status_to_str, str_to_priority, str_to_status},
        progress::Progress as Prog,
        types::FileFormat,
    };
}
