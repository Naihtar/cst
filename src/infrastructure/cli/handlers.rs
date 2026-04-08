use crate::prelude::{
    Cmd, DEFAULT_PAGE_SIZE, Decision, DomErr, Err, Filter, Mode, Priority, Repository, Service,
    Sort, Status, Task, created_msg, deleted_msg, export_msg, found_msg, import_msg, many_msg,
    preview_msg, updated_msg,
};

/// Output variants returned by the CLI handler to the presenter layer.
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

/// Dispatches [`Cmd`]s to the appropriate service methods.
pub struct CliHandler<R: Repository> {
    service: Service<R>,
}

impl<R: Repository> CliHandler<R> {
    /// Creates a new handler backed by the given service.
    pub fn new(service: Service<R>) -> Self {
        Self { service }
    }

    /// Routes a command to its handler and returns the CLI output.
    pub fn handle(&self, command: Cmd) -> Result<CliOutput, Err> {
        match command {
            Cmd::Add {
                information,
                priority,
                status,
            } => self.handle_add(information, priority, status),
            Cmd::Get { id } => self.handle_get(id),
            Cmd::List(sort) => self.handle_list(sort),
            Cmd::Done { id } => self.handle_done(id),
            Cmd::DoneMany { ids } => self.handle_done_many(ids),
            Cmd::Update {
                id,
                information,
                priority,
                status,
            } => self.handle_update(id, information, priority, status),
            Cmd::Remove { id, confirmed } => self.handle_remove(id, confirmed),
            Cmd::RemoveMany { ids, confirmed } => self.handle_remove_many(ids, confirmed),
            Cmd::UpdateMany {
                ids,
                priority,
                status,
            } => self.handle_update_many(ids, priority, status),
            Cmd::Clear { confirmed } => self.handle_clear(confirmed),
            Cmd::Paged(sort, page) => self.handle_paged(sort, page),
            Cmd::Filter(filter) => self.handle_filter(filter),
            Cmd::Import {
                path,
                mode,
                confirmed,
            } => self.handle_import(&path, mode, confirmed),
            Cmd::ExportCSV { output } => self.handle_export_csv(output),
            Cmd::ExportMarkdown { output } => self.handle_export_markdown(output),
            Cmd::ExportExcel { output } => self.handle_export_excel(output),
            Cmd::ExportJson { output } => self.handle_export_json(output),
            Cmd::ExportYaml { output } => self.handle_export_yaml(output),
            Cmd::ExportToml { output } => self.handle_export_toml(output),
            Cmd::ImportJson {
                path,
                mode,
                confirmed,
            } => self.handle_import_json(&path, mode, confirmed),
            Cmd::ImportYaml {
                path,
                mode,
                confirmed,
            } => self.handle_import_yaml(&path, mode, confirmed),
            Cmd::ImportToml {
                path,
                mode,
                confirmed,
            } => self.handle_import_toml(&path, mode, confirmed),
            Cmd::Undo => self.handle_undo(),
            _ => unreachable!("Help, Version y Config son manejados en app.rs"),
        }
    }

    fn handle_add(
        &self,
        information: String,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<CliOutput, Err> {
        self.service
            .add_task(information, priority, status)
            .map(|id| CliOutput::Message(created_msg(id)))
    }

    fn handle_get(&self, id: i64) -> Result<CliOutput, Err> {
        let task = self
            .service
            .find_task_by_id(id)?
            .ok_or(DomErr::NotFoundID(id))?;
        Ok(CliOutput::Message(found_msg(&task)))
    }

    fn handle_list(&self, sort: Sort) -> Result<CliOutput, Err> {
        self.service.list_tasks(sort).map(CliOutput::Tasks)
    }

    fn handle_paged(&self, sort: Sort, page: i64) -> Result<CliOutput, Err> {
        self.service
            .paged_tasks(sort, page)
            .map(|(tasks, total)| CliOutput::PagedTasks {
                tasks,
                page,
                page_size: DEFAULT_PAGE_SIZE,
                total,
            })
    }

    fn handle_filter(&self, filter: Filter) -> Result<CliOutput, Err> {
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

    fn handle_done(&self, id: i64) -> Result<CliOutput, Err> {
        self.service
            .update_task(id, None, None, Some(Status::Done))
            .map(|id| CliOutput::Message(updated_msg(id)))
    }

    fn handle_done_many(&self, ids: Vec<i64>) -> Result<CliOutput, Err> {
        self.service
            .done_many(ids)
            .map(|(processed, missing)| CliOutput::Message(many_msg(&processed, &missing)))
    }

    fn handle_update(
        &self,
        id: i64,
        information: Option<String>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<CliOutput, Err> {
        self.service
            .update_task(id, information, priority, status)
            .map(|id| CliOutput::Message(updated_msg(id)))
    }

    fn handle_update_many(
        &self,
        ids: Vec<i64>,
        priority: Option<Priority>,
        status: Option<Status>,
    ) -> Result<CliOutput, Err> {
        self.service
            .update_many(ids, priority, status)
            .map(|(processed, missing)| CliOutput::Message(many_msg(&processed, &missing)))
    }

    /// Returns [`CliOutput::Cancelled`] if the user did not confirm.
    fn handle_remove(&self, id: i64, confirmed: Option<Decision>) -> Result<CliOutput, Err> {
        match confirmed {
            Some(Decision::Yes) => self
                .service
                .remove_task(id)
                .map(|id| CliOutput::Message(deleted_msg(id))),
            _ => Ok(CliOutput::Cancelled),
        }
    }

    /// Returns [`CliOutput::Cancelled`] if the user did not confirm.
    fn handle_remove_many(
        &self,
        ids: Vec<i64>,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, Err> {
        match confirmed {
            Some(Decision::Yes) => self
                .service
                .remove_many(ids)
                .map(|(processed, missing)| CliOutput::Message(many_msg(&processed, &missing))),
            _ => Ok(CliOutput::Cancelled),
        }
    }

    /// Returns [`CliOutput::Cancelled`] if the user did not confirm.
    fn handle_clear(&self, confirmed: Option<Decision>) -> Result<CliOutput, Err> {
        match confirmed {
            Some(Decision::Yes) => self.service.clear_all_tasks().map(|_| CliOutput::Success),
            _ => Ok(CliOutput::Cancelled),
        }
    }

    fn handle_undo(&self) -> Result<CliOutput, Err> {
        self.service.undo().map(|_| CliOutput::Success)
    }

    /// Shared import runner for all formats.
    ///
    /// Handles `DryRun` preview and confirmation gating for `Append`/`Restore`.
    fn run_import(
        &self,
        path: &str,
        mode: Mode,
        confirmed: Option<Decision>,
        do_import: impl Fn(&str, &Mode) -> Result<(usize, f64), Err>,
        do_preview: impl Fn(&str) -> Result<usize, Err>,
    ) -> Result<CliOutput, Err> {
        match mode {
            Mode::DryRun => do_preview(path).map(|count| CliOutput::Message(preview_msg(count))),
            Mode::Append | Mode::Restore => match confirmed {
                Some(Decision::Yes) => do_import(path, &mode)
                    .map(|(count, elapsed)| CliOutput::Message(import_msg(count, elapsed))),
                _ => Ok(CliOutput::Cancelled),
            },
        }
    }

    fn handle_import(
        &self,
        path: &str,
        mode: Mode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, Err> {
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
        mode: Mode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, Err> {
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
        mode: Mode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, Err> {
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
        mode: Mode,
        confirmed: Option<Decision>,
    ) -> Result<CliOutput, Err> {
        self.run_import(
            path,
            mode,
            confirmed,
            |p, m| self.service.import_toml(p, m),
            |p| self.service.import_toml_preview(p),
        )
    }

    fn handle_export_csv(&self, output: Option<String>) -> Result<CliOutput, Err> {
        self.service
            .export_csv(output)
            .map(|(count, path, elapsed)| CliOutput::Message(export_msg(count, &path, elapsed)))
    }

    fn handle_export_markdown(&self, output: Option<String>) -> Result<CliOutput, Err> {
        self.service
            .export_markdown(output)
            .map(|(count, path, elapsed)| CliOutput::Message(export_msg(count, &path, elapsed)))
    }

    fn handle_export_excel(&self, output: Option<String>) -> Result<CliOutput, Err> {
        self.service
            .export_excel(output)
            .map(|(count, path, elapsed)| CliOutput::Message(export_msg(count, &path, elapsed)))
    }

    fn handle_export_json(&self, output: Option<String>) -> Result<CliOutput, Err> {
        self.service
            .export_json(output)
            .map(|(count, path, elapsed)| CliOutput::Message(export_msg(count, &path, elapsed)))
    }

    fn handle_export_yaml(&self, output: Option<String>) -> Result<CliOutput, Err> {
        self.service
            .export_yaml(output)
            .map(|(count, path, elapsed)| CliOutput::Message(export_msg(count, &path, elapsed)))
    }

    fn handle_export_toml(&self, output: Option<String>) -> Result<CliOutput, Err> {
        self.service
            .export_toml(output)
            .map(|(count, path, elapsed)| CliOutput::Message(export_msg(count, &path, elapsed)))
    }
}
