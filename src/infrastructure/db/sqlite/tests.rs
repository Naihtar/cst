#[cfg(test)]
mod tests {
    use crate::prelude::{
        Builder, Filter, NewTask, Priority, Repository, Sort, SortField, SortOrder, SqliteTR,
        Status, Store,
    };

    fn setup() -> SqliteTR {
        Store::init_for_tests();
        SqliteTR::new(":memory:").unwrap()
    }

    fn make_task(info: &str, priority: Priority, status: Status) -> NewTask {
        Builder::new()
            .information(info.to_string())
            .priority(Some(priority))
            .status(Some(status))
            .build()
            .unwrap()
    }

    // ── Create ────────────────────────────────────────────────────────────────

    #[test]
    fn create_returns_valid_id() {
        let repo = setup();
        let task = make_task("Test task", Priority::Low, Status::Todo);
        let id = repo.create(&task).unwrap();
        assert!(id > 0);
    }

    #[test]
    fn create_increments_id() {
        let repo = setup();
        let t1 = make_task("Task 1", Priority::Low, Status::Todo);
        let t2 = make_task("Task 2", Priority::Low, Status::Todo);
        let id1 = repo.create(&t1).unwrap();
        let id2 = repo.create(&t2).unwrap();
        assert!(id2 > id1);
    }

    // ── Read ──────────────────────────────────────────────────────────────────

    #[test]
    fn read_by_id_returns_correct_task() {
        let repo = setup();
        let task = make_task("Test task", Priority::High, Status::InProgress);
        let id = repo.create(&task).unwrap();
        let found = repo.read_by_id(id).unwrap().unwrap();
        assert_eq!(found.information(), "Test task");
        assert_eq!(found.priority(), &Priority::High);
        assert_eq!(found.status(), &Status::InProgress);
    }

    #[test]
    fn read_by_id_returns_none_for_nonexistent() {
        let repo = setup();
        let result = repo.read_by_id(999).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn read_all_returns_all_tasks() {
        let repo = setup();
        repo.create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task("Task 2", Priority::High, Status::Done))
            .unwrap();
        repo.create(&make_task("Task 3", Priority::Medium, Status::Blocked))
            .unwrap();
        let tasks = repo.read_all(Sort::default()).unwrap();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn read_all_empty_returns_empty_vec() {
        let repo = setup();
        let tasks = repo.read_all(Sort::default()).unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn read_all_sorted_by_priority_asc() {
        let repo = setup();
        repo.create(&make_task("Urgent", Priority::Urgent, Status::Todo))
            .unwrap();
        repo.create(&make_task("Low", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task("High", Priority::High, Status::Todo))
            .unwrap();
        let tasks = repo
            .read_all(Sort {
                field: Some(SortField::Priority),
                order: SortOrder::Asc,
            })
            .unwrap();
        assert_eq!(tasks[0].priority(), &Priority::Low);
        assert_eq!(tasks[1].priority(), &Priority::High);
        assert_eq!(tasks[2].priority(), &Priority::Urgent);
    }

    #[test]
    fn read_all_sorted_by_priority_desc() {
        let repo = setup();
        repo.create(&make_task("Low", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task("Urgent", Priority::Urgent, Status::Todo))
            .unwrap();
        let tasks = repo
            .read_all(Sort {
                field: Some(SortField::Priority),
                order: SortOrder::Desc,
            })
            .unwrap();
        assert_eq!(tasks[0].priority(), &Priority::Urgent);
        assert_eq!(tasks[1].priority(), &Priority::Low);
    }

    // ── Paged ─────────────────────────────────────────────────────────────────

    #[test]
    fn read_paged_returns_correct_page() {
        let repo = setup();
        for i in 1..=25 {
            repo.create(&make_task(
                &format!("Task {}", i),
                Priority::Low,
                Status::Todo,
            ))
            .unwrap();
        }
        let (tasks, total) = repo.read_paged(Sort::default(), 0).unwrap();
        assert_eq!(tasks.len(), 20);
        assert_eq!(total, 25);
    }

    #[test]
    fn read_paged_second_page() {
        let repo = setup();
        for i in 1..=25 {
            repo.create(&make_task(
                &format!("Task {}", i),
                Priority::Low,
                Status::Todo,
            ))
            .unwrap();
        }
        let (tasks, total) = repo.read_paged(Sort::default(), 1).unwrap();
        assert_eq!(tasks.len(), 5);
        assert_eq!(total, 25);
    }

    // ── Update ────────────────────────────────────────────────────────────────

    #[test]
    fn update_modifies_task_correctly() {
        let repo = setup();
        let id = repo
            .create(&make_task("Original", Priority::Low, Status::Todo))
            .unwrap();
        let existing = repo.read_by_id(id).unwrap().unwrap();
        let updated = Builder::from_task(&existing)
            .information_update(Some("Updated".to_string()))
            .priority(Some(Priority::Urgent))
            .status(Some(Status::Done))
            .build_with_id()
            .unwrap();
        repo.update(&updated).unwrap();
        let found = repo.read_by_id(id).unwrap().unwrap();
        assert_eq!(found.information(), "Updated");
        assert_eq!(found.priority(), &Priority::Urgent);
        assert_eq!(found.status(), &Status::Done);
    }

    // ── Update Many ───────────────────────────────────────────────────────────

    #[test]
    fn update_many_modifies_multiple_tasks() {
        let repo = setup();
        let id1 = repo
            .create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        let id2 = repo
            .create(&make_task("Task 2", Priority::Low, Status::Todo))
            .unwrap();
        let id3 = repo
            .create(&make_task("Task 3", Priority::Low, Status::Todo))
            .unwrap();
        let processed = repo
            .update_many(&[id1, id2], Some(Priority::High), Some(Status::Done))
            .unwrap();
        assert_eq!(processed.len(), 2);
        assert_eq!(
            repo.read_by_id(id1).unwrap().unwrap().priority(),
            &Priority::High
        );
        assert_eq!(
            repo.read_by_id(id2).unwrap().unwrap().status(),
            &Status::Done
        );
        assert_eq!(
            repo.read_by_id(id3).unwrap().unwrap().priority(),
            &Priority::Low
        );
    }

    #[test]
    fn update_many_ignores_nonexistent_ids() {
        let repo = setup();
        let id = repo
            .create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        let processed = repo
            .update_many(&[id, 999], Some(Priority::High), None)
            .unwrap();
        assert_eq!(processed.len(), 1);
        assert!(processed.contains(&id));
    }

    // ── Done Many ─────────────────────────────────────────────────────────────

    #[test]
    fn done_many_marks_tasks_as_done() {
        let repo = setup();
        let id1 = repo
            .create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        let id2 = repo
            .create(&make_task("Task 2", Priority::Low, Status::InProgress))
            .unwrap();
        repo.done_many(&[id1, id2]).unwrap();
        assert_eq!(
            repo.read_by_id(id1).unwrap().unwrap().status(),
            &Status::Done
        );
        assert_eq!(
            repo.read_by_id(id2).unwrap().unwrap().status(),
            &Status::Done
        );
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    #[test]
    fn delete_removes_task() {
        let repo = setup();
        let id = repo
            .create(&make_task("Task", Priority::Low, Status::Todo))
            .unwrap();
        repo.delete(id).unwrap();
        assert!(repo.read_by_id(id).unwrap().is_none());
    }

    #[test]
    fn delete_many_removes_multiple_tasks() {
        let repo = setup();
        let id1 = repo
            .create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        let id2 = repo
            .create(&make_task("Task 2", Priority::Low, Status::Todo))
            .unwrap();
        let id3 = repo
            .create(&make_task("Task 3", Priority::Low, Status::Todo))
            .unwrap();
        let processed = repo.delete_many(&[id1, id2]).unwrap();
        assert_eq!(processed.len(), 2);
        assert!(repo.read_by_id(id1).unwrap().is_none());
        assert!(repo.read_by_id(id2).unwrap().is_none());
        assert!(repo.read_by_id(id3).unwrap().is_some());
    }

    #[test]
    fn delete_many_ignores_nonexistent_ids() {
        let repo = setup();
        let id = repo
            .create(&make_task("Task", Priority::Low, Status::Todo))
            .unwrap();
        let processed = repo.delete_many(&[id, 999]).unwrap();
        assert_eq!(processed.len(), 1);
    }

    #[test]
    fn delete_all_removes_all_tasks() {
        let repo = setup();
        repo.create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task("Task 2", Priority::Low, Status::Todo))
            .unwrap();
        repo.delete_all().unwrap();
        assert!(repo.read_all(Sort::default()).unwrap().is_empty());
    }

    // ── Filter ────────────────────────────────────────────────────────────────

    #[test]
    fn filter_by_word() {
        let repo = setup();
        repo.create(&make_task("Crear API REST", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task(
            "Documentar endpoints",
            Priority::Low,
            Status::Todo,
        ))
        .unwrap();
        repo.create(&make_task("Testear API REST", Priority::Low, Status::Todo))
            .unwrap();
        let (tasks, total) = repo
            .filter(Filter {
                word: Some("API".to_string()),
                ..Filter::default()
            })
            .unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(total, 2);
    }

    #[test]
    fn filter_by_priority() {
        let repo = setup();
        repo.create(&make_task("Task 1", Priority::Urgent, Status::Todo))
            .unwrap();
        repo.create(&make_task("Task 2", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task("Task 3", Priority::Urgent, Status::Done))
            .unwrap();
        let (tasks, total) = repo
            .filter(Filter {
                priority: Some(Priority::Urgent),
                ..Filter::default()
            })
            .unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(total, 2);
    }

    #[test]
    fn filter_by_status() {
        let repo = setup();
        repo.create(&make_task("Task 1", Priority::Low, Status::Blocked))
            .unwrap();
        repo.create(&make_task("Task 2", Priority::Low, Status::Todo))
            .unwrap();
        repo.create(&make_task("Task 3", Priority::Low, Status::Blocked))
            .unwrap();
        let (tasks, total) = repo
            .filter(Filter {
                status: Some(Status::Blocked),
                ..Filter::default()
            })
            .unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(total, 2);
    }

    #[test]
    fn filter_no_results() {
        let repo = setup();
        repo.create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        let (tasks, total) = repo
            .filter(Filter {
                word: Some("xyz_inexistente".to_string()),
                ..Filter::default()
            })
            .unwrap();
        assert!(tasks.is_empty());
        assert_eq!(total, 0);
    }

    // ── Undo / Snapshot ───────────────────────────────────────────────────────

    #[test]
    fn has_snapshot_false_initially() {
        let repo = setup();
        assert!(!repo.has_snapshot().unwrap());
    }

    #[test]
    fn save_snapshot_creates_snapshot() {
        let repo = setup();
        repo.create(&make_task("Task", Priority::Low, Status::Todo))
            .unwrap();
        repo.save_snapshot().unwrap();
        assert!(repo.has_snapshot().unwrap());
    }

    #[test]
    fn restore_snapshot_restores_previous_state() {
        let repo = setup();
        let id = repo
            .create(&make_task("Original", Priority::Low, Status::Todo))
            .unwrap();
        repo.save_snapshot().unwrap();
        repo.delete(id).unwrap();
        assert!(repo.read_by_id(id).unwrap().is_none());
        repo.restore_snapshot().unwrap();
        assert!(repo.read_by_id(id).unwrap().is_some());
        assert_eq!(
            repo.read_by_id(id).unwrap().unwrap().information(),
            "Original"
        );
    }

    #[test]
    fn restore_snapshot_clears_snapshot_after_restore() {
        let repo = setup();
        repo.create(&make_task("Task", Priority::Low, Status::Todo))
            .unwrap();
        repo.save_snapshot().unwrap();
        repo.restore_snapshot().unwrap();
        assert!(!repo.has_snapshot().unwrap());
    }

    #[test]
    fn save_snapshot_overwrites_previous_snapshot() {
        let repo = setup();
        let id1 = repo
            .create(&make_task("Task 1", Priority::Low, Status::Todo))
            .unwrap();
        repo.save_snapshot().unwrap();
        repo.create(&make_task("Task 2", Priority::Low, Status::Todo))
            .unwrap();
        repo.save_snapshot().unwrap();
        repo.delete_all().unwrap();
        repo.restore_snapshot().unwrap();
        let tasks = repo.read_all(Sort::default()).unwrap();
        assert_eq!(tasks.len(), 2);
        assert!(tasks.iter().any(|t| t.id() == id1));
    }
}
