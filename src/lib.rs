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
    #[cfg(test)]
    pub mod tests;
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
            entity::{NewTask, Task, TaskBuilder},
            filter::{DEFAULT_PAGE_SIZE, Filter, Sort, SortField, SortOrder},
            types::{ImportMode, Priority, Status},
        },
        repositories::task::repository::TaskRepository,
    };

    // ── Application ───────────────────────────────────────────────────────────
    pub use crate::application::services::task::service::TaskService;

    // ── Config ────────────────────────────────────────────────────────────────
    pub use crate::config::{language::Language, settings::Settings};

    // ── Errors ────────────────────────────────────────────────────────────────
    pub use crate::error::{
        CSTError, CliError as CliErr, ConfigError as ConfigErr, DomainError as DomErr,
        IoError as IOErr,
    };
    // ── Infrastructure – DB ───────────────────────────────────────────────────
    pub use crate::infrastructure::db::sqlite::db::SqliteTaskRepository;

    // ── Infrastructure – CLI ──────────────────────────────────────────────────
    pub use crate::infrastructure::cli::{
        commands::{ConfigCommand, TaskCommand},
        handlers::{CliHandler, CliOutput},
        parser::CliParser,
    };

    // ── Infrastructure – IO ───────────────────────────────────────────────────
    pub use crate::infrastructure::io::{progress::Progress, types::FileFormat};
}
