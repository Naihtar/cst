use crate::{
    infrastructure::cli::ui::messages::{
        confirm::Decision,
        export::format_export,
        import::{format_import, format_import_preview},
        task::{
            format_many_result, format_task_created, format_task_deleted, format_task_found,
            format_task_updated,
        },
    },
    prelude::{
        CSTError, DEFAULT_PAGE_SIZE, DomErr, Filter, ImportMode, Priority, Sort, Status, Task,
        TaskCommand, TaskRepository, TaskService,
    },
};

/// CliOutput variants returned by the CLI handler to the presenter layer.
pub enum CliOutput {
    Tasks(Vec<Task>),
    PagedTasks {
        tasks: Vec<Task>,
        page: i64,
        page_size: i64,
        total: i64,
    },
    Message(String),
    Success,
    Cancelled,
}

/// Dispatches [`TaskCommand`]s to the appropriate service methods.
pub struct CliHandler<R: TaskRepository> {
    service: TaskService<R>,
}

impl<R: TaskRepository> CliHandler<R> {
    /// Creates a new handler backed by the given service.
    pub fn new(service: TaskService<R>) -> Self {
        Self { service }
    }

    /// Routes a command to its handler and returns the CLI output.
    pub fn handle(&self, command: TaskCommand) -> Result<CliOutput, CSTError> {
        match command {
            TaskCommand::Add {
                information,
                priority,
                status,
            } => self.handle_add(information, priority, status),
            TaskCommand::Get { id } => self.handle_get(id),
            TaskCommand::List(sort) => self.handle_list(sort),
            TaskCommand::Done { id } => self.handle_done(id),
            TaskCommand::DoneMany { ids } => self.handle_done_many(ids),
            TaskCommand::Update {
                id,
                information,
                priority,
                status,
            } => self.handle_update(id, information, priority, status),
            TaskCommand::Remove { id, confirmed } => self.handle_remove(id, confirmed),
            TaskCommand::RemoveMany { ids, confirmed } => self.handle_remove_many(ids, confirmed),
            TaskCommand::UpdateMany {
                ids,
                priority,
                status,
            } => self.handle_update_many(ids, priority, status),
            TaskCommand::Clear { confirmed } => self.handle_clear(confirmed),
            TaskCommand::Paged(sort, page) => self.handle_paged(sort, page),
            TaskCommand::Filter(filter) => self.handle_filter(filter),
            TaskCommand::Import {
                path,
                mode,
                confirmed,
            } => self.handle_import(&path, mode, confirmed),
            TaskCommand::ExportCSV { output } => self.handle_export_csv(output),
            TaskCommand::ExportMarkdown { output } => self.handle_export_markdown(output),
            TaskCommand::ExportExcel { output } => self.handle_export_excel(output),
            TaskCommand::ExportJson { output } => self.handle_export_json(output),
            TaskCommand::ExportYaml { output } => self.handle_export_yaml(output),
            TaskCommand::ExportToml { output } => self.handle_export_toml(output),
            TaskCommand::ImportJson {
                path,
                mode,
                confirmed,
            } => self.handle_import_json(&path, mode, confirmed),
            TaskCommand::ImportYaml {
                path,
                mode,
                confirmed,
            } => self.handle_import_yaml(&path, mode, confirmed),
            TaskCommand::ImportToml {
                path,
                mode,
                confirmed,
            } => self.handle_import_toml(&path, mode, confirmed),
            TaskCommand::Undo => self.handle_undo(),
            _ => unreachable!("Help, Version y Config son manejados en app.rs"),
        }
    }

    fn handle_add(
        &self,
        information: String,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<CliOutput, CSTError> {
        self.service
            .add_task(information, priority, status)
            .map(|id| CliOutput::Message(format_task_created(id)))
    }

    fn handle_get(&self, id: i64) -> Result<CliOutput, CSTError> {
        let task = self
            .service
            .find_task_by_id(id)?
            .ok_or(DomErr::NotFoundID(id))?;
        Ok(CliOutput::Message(format_task_found(&task)))
    }

    fn handle_list(&self, sort: Sort) -> Result<CliOutput, CSTError> {
        self.service.list_tasks(sort).map(CliOutput::Tasks)
    }

    fn handle_paged(&self, sort: Sort, page: i64) -> Result<CliOutput, CSTError> {
        self.service
            .paged_tasks(sort, page)
            .map(|(tasks, total)| CliOutput::PagedTasks {
                tasks,
                page,
                page_size: DEFAULT_PAGE_SIZE,
                total,
            })
    }

    fn handle_filter(&self, filter: Filter) -> Result<CliOutput, CSTError> {
        let page = filter.page;
        let page_size = filter.page_size;
        self.service
            .filter_tasks(filter)
            .map(|(tasks, total)| CliOutput::PagedTasks {
                tasks,
                page,
                page_size,
                total,
            })
    }

    fn handle_done(&self, id: i64) -> Result<CliOutput, CSTError> {
        self.service
            .update_task(id, None, None, Some(Status::Done))
            .map(|id| CliOutput::Message(format_task_updated(id)))
    }

    fn handle_done_many(&self, ids: Vec<i64>) -> Result<CliOutput, CSTError> {
        self.service.done_many(ids).map(|(processed, missing)| {
            CliOutput::Message(format_many_result(&processed, &missing))
        })
    }

    fn handle_update(
        &self,
        id: i64,
        information: Option<String>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<CliOutput, CSTError> {
        self.service
            .update_task(id, information, priority, status)
            .map(|id| CliOutput::Message(format_task_updated(id)))
    }

    fn handle_update_many(
        &self,
        ids: Vec<i64>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<CliOutput, CSTError> {
        self.service
            .update_many(ids, priority, status)
            .map(|(processed, missing)| {
                CliOutput::Message(format_many_result(&processed, &missing))
            })
    }

    /// Returns [`CliOutput::Cancelled`] if the user did not confirm.
    fn handle_remove(&self, id: i64, confirmed: Option<Decision>) -> Result<CliOutput, CSTError> {
        match confirmed {
            Some(Decision::Yes) => self
                .service
                .remove_task(id)
                .map(|id| CliOutput::Message(format_task_deleted(id))),
            _ => Ok(CliOutput::Cancelled),
        }
    }

    /// Returns [`CliOutput::Cancelled`] if the user did not confirm.
    fn handle_remove_many(
        &self,
        ids: Vec<i64>,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, CSTError> {
        match confirmed {
            Some(Decision::Yes) => self.service.remove_many(ids).map(|(processed, missing)| {
                CliOutput::Message(format_many_result(&processed, &missing))
            }),
            _ => Ok(CliOutput::Cancelled),
        }
    }

    /// Returns [`CliOutput::Cancelled`] if the user did not confirm.
    fn handle_clear(&self, confirmed: Option<Decision>) -> Result<CliOutput, CSTError> {
        match confirmed {
            Some(Decision::Yes) => self.service.clear_all_tasks().map(|_| CliOutput::Success),
            _ => Ok(CliOutput::Cancelled),
        }
    }

    fn handle_undo(&self) -> Result<CliOutput, CSTError> {
        self.service.undo().map(|_| CliOutput::Success)
    }

    /// Shared import runner for all formats.
    ///
    /// Handles `DryRun` preview and confirmation gating for `Append`/`Restore`.
    fn run_import(
        &self,
        path: &str,
        mode: ImportMode,
        confirmed: Option<Decision>,
        do_import: impl Fn(&str, &ImportMode) -> Result<(usize, f64), CSTError>,
        do_preview: impl Fn(&str) -> Result<usize, CSTError>,
    ) -> Result<CliOutput, CSTError> {
        match mode {
            ImportMode::DryRun => {
                do_preview(path).map(|count| CliOutput::Message(format_import_preview(count)))
            }
            ImportMode::Append | ImportMode::Restore => match confirmed {
                Some(Decision::Yes) => do_import(path, &mode)
                    .map(|(count, elapsed)| CliOutput::Message(format_import(count, elapsed))),
                _ => Ok(CliOutput::Cancelled),
            },
        }
    }

    fn handle_import(
        &self,
        path: &str,
        mode: ImportMode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, CSTError> {
        self.run_import(
            path,
            mode,
            confirmed,
            |p, m| self.service.import(p, m),
            |p| self.service.import_preview(p),
        )
    }

    fn handle_import_json(
        &self,
        path: &str,
        mode: ImportMode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, CSTError> {
        self.run_import(
            path,
            mode,
            confirmed,
            |p, m| self.service.import_json(p, m),
            |p| self.service.import_json_preview(p),
        )
    }

    fn handle_import_yaml(
        &self,
        path: &str,
        mode: ImportMode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, CSTError> {
        self.run_import(
            path,
            mode,
            confirmed,
            |p, m| self.service.import_yaml(p, m),
            |p| self.service.import_yaml_preview(p),
        )
    }

    fn handle_import_toml(
        &self,
        path: &str,
        mode: ImportMode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, CSTError> {
        self.run_import(
            path,
            mode,
            confirmed,
            |p, m| self.service.import_toml(p, m),
            |p| self.service.import_toml_preview(p),
        )
    }

    fn handle_export_csv(&self, output: Option<String>) -> Result<CliOutput, CSTError> {
        self.service
            .export_csv(output)
            .map(|(count, path, elapsed)| CliOutput::Message(format_export(count, &path, elapsed)))
    }

    fn handle_export_markdown(&self, output: Option<String>) -> Result<CliOutput, CSTError> {
        self.service
            .export_markdown(output)
            .map(|(count, path, elapsed)| CliOutput::Message(format_export(count, &path, elapsed)))
    }

    fn handle_export_excel(&self, output: Option<String>) -> Result<CliOutput, CSTError> {
        self.service
            .export_excel(output)
            .map(|(count, path, elapsed)| CliOutput::Message(format_export(count, &path, elapsed)))
    }

    fn handle_export_json(&self, output: Option<String>) -> Result<CliOutput, CSTError> {
        self.service
            .export_json(output)
            .map(|(count, path, elapsed)| CliOutput::Message(format_export(count, &path, elapsed)))
    }

    fn handle_export_yaml(&self, output: Option<String>) -> Result<CliOutput, CSTError> {
        self.service
            .export_yaml(output)
            .map(|(count, path, elapsed)| CliOutput::Message(format_export(count, &path, elapsed)))
    }

    fn handle_export_toml(&self, output: Option<String>) -> Result<CliOutput, CSTError> {
        self.service
            .export_toml(output)
            .map(|(count, path, elapsed)| CliOutput::Message(format_export(count, &path, elapsed)))
    }
}
