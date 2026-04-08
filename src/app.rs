use crate::prelude::{
    Cmd, Err, Handler, Output, Parser, Service, SqliteTR, Store, cancelled_msg, config, done_msg,
    err_msg, help, paged_msg, tasks_msg, version,
};

/// Entry point for the application. Handles global error reporting and process exit.
pub fn run() {
    if let Err(e) = run_inner() {
        eprintln!("{}", err_msg(&e));
        std::process::exit(1);
    }
}

/// Orchestrates settings loading, command parsing, and execution.
///
/// Returns a [`Result`] to allow the caller to handle top-level errors.
fn run_inner() -> Result<(), Err> {
    // Ensure configuration is loaded before any operation
    Store::load()?;

    // Parse arguments and handle potential confirmation prompts (e.g., --force)
    let command = Parser::new().parse()?.ensure_confirmation()?;

    // Route commands. Built-in CLI commands are handled separately from
    // domain operations that require a database connection.
    let output = match command {
        Cmd::Help => help()?,
        Cmd::Version => version()?,
        Cmd::Config(cfg) => config(cfg)?,
        _ => {
            let settings = Store::get()?;

            // Initialize infrastructure and application layers
            let repo = SqliteTR::new(&settings.db_path)?;
            let service = Service::new(repo);
            let handler = Handler::new(service);

            handler.handle(command)?
        }
    };

    // Render the output of the command to the standard output
    match output {
        Output::Tasks(tasks) => {
            println!("{}", tasks_msg(&tasks));
        }
        Output::PagedTasks {
            tasks,
            page,
            page_size,
            total,
        } => {
            println!("{}", paged_msg(&tasks, page, page_size, total));
        }
        Output::Message(msg) => {
            println!("{}", msg);
        }
        Output::Success => {
            println!("{}", done_msg());
        }
        Output::Cancelled => {
            println!("{}", cancelled_msg());
        }
    }

    Ok(())
}
