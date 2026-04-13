use crate::{
    infrastructure::cli::{
        config_handler::{handle_config, handle_help, handle_version},
        ui::messages::{
            error::format_error,
            export::{format_paged_tasks, format_tasks},
            task::{format_cancelled_msg, format_done},
        },
    },
    prelude::{
        CSTError, CliHandler, CliOutput, CliParser, Settings, SqliteTaskRepository, TaskCommand,
        TaskService,
    },
};

/// Entry point for the application. Handles global error reporting and process exit.
pub fn run() {
    if let Err(e) = run_inner() {
        eprintln!("{}", format_error(&e));
        std::process::exit(1);
    }
}

/// Orchestrates settings loading, command parsing, and execution.
///
/// Returns a [`Result`] to allow the caller to handle top-level errors.
fn run_inner() -> Result<(), CSTError> {
    // Ensure configuration is loaded before any operation
    Settings::load()?;

    // Parse arguments and handle potential confirmation prompts (e.g., --force)
    let command = CliParser::new().parse()?.ensure_confirmation()?;

    // Route commands. Built-in CLI commands are handled separately from
    // domain operations that require a database connection.
    let output = match command {
        TaskCommand::Help => handle_help()?,
        TaskCommand::Version => handle_version()?,
        TaskCommand::Config(cfg) => handle_config(cfg)?,
        _ => {
            let settings = Settings::get()?;

            // Initialize infrastructure and application layers
            let repo = SqliteTaskRepository::new(&settings.db_path)?;
            let service = TaskService::new(repo);
            let handler = CliHandler::new(service);

            handler.handle(command)?
        }
    };

    // Render the output of the command to the standard output
    match output {
        CliOutput::Tasks(tasks) => {
            println!("{}", format_tasks(&tasks));
        }
        CliOutput::PagedTasks {
            tasks,
            page,
            page_size,
            total,
        } => {
            println!("{}", format_paged_tasks(&tasks, page, page_size, total));
        }
        CliOutput::Message(msg) => {
            println!("{}", msg);
        }
        CliOutput::Success => {
            println!("{}", format_done());
        }
        CliOutput::Cancelled => {
            println!("{}", format_cancelled_msg());
        }
    }

    Ok(())
}
