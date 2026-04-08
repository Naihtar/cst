use crate::prelude::{Err, FileFormat, Store, Task, export, priority_to_str, status_to_str};

use rust_xlsxwriter::Workbook;

/// Exports tasks to an Excel `.xlsx` file at the resolved output path.
pub fn export_excel(tasks: &[Task], output: Option<&str>) -> Result<(usize, String, f64), Err> {
    export(tasks, output, FileFormat::Excel, write_excel)
}

/// Writes tasks to `path` as an Excel workbook with localized column headers.
fn write_excel(tasks: &[Task], path: &str) -> Result<(), Err> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, &Store::t("table.id"))?;
    worksheet.write_string(0, 1, &Store::t("table.task"))?;
    worksheet.write_string(0, 2, &Store::t("table.priority"))?;
    worksheet.write_string(0, 3, &Store::t("table.status"))?;
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
