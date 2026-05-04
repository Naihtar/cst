use crate::{
    infrastructure::io::{
        export::common::export,
        mappers::{priority_to_str, status_to_str},
    },
    prelude::{CSTError, FileFormat, Settings, Task},
};

use rust_xlsxwriter::Workbook;

/// Exports tasks to an Excel `.xlsx` file at the resolved output path.
pub fn export_excel(
    tasks: &[Task],
    output: Option<&str>,
) -> Result<(usize, String, f64), CSTError> {
    export(tasks, output, FileFormat::Excel, write_excel)
}

/// Writes tasks to `path` as an Excel workbook with localized column headers.
fn write_excel(tasks: &[Task], path: &str) -> Result<(), CSTError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, Settings::t("table.id"))?;
    worksheet.write_string(0, 1, Settings::t("table.task"))?;
    worksheet.write_string(0, 2, Settings::t("table.priority"))?;
    worksheet.write_string(0, 3, Settings::t("table.status"))?;
    for (i, task) in tasks.iter().enumerate() {
        let row = (i + 1) as u32;
        worksheet.write_number(row, 0, task.id() as f64)?;
        worksheet.write_string(row, 1, task.information())?;
        worksheet.write_string(row, 2, priority_to_str(task.priority()))?;
        worksheet.write_string(row, 3, status_to_str(task.status()))?;
    }
    workbook.save(path)?;
    Ok(())
}
