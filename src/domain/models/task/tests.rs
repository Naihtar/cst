#[cfg(test)]
mod tests {
    // use crate::{
    //     config::settings::Settings,
    //     domain::models::task::{
    //         entity::Builder,
    //         filter::{DEFAULT_PAGE_SIZE, Filter, Sort, SortField, SortOrder},
    //         types::{Priority, Status},
    //     },
    //     DomErr},
    // };

    use crate::prelude::{
        Builder, DEFAULT_PAGE_SIZE, DomErr, Err, Filter, Priority, Sort, SortField, SortOrder,
        Status, Store,
    };

    fn setup() {
        Store::init_for_tests();
    }

    // ── Builder ───────────────────────────────────────────────────────────

    #[test]
    fn task_builder_builds_with_defaults() {
        setup();
        let task = Builder::new()
            .information("Test task".to_string())
            .build()
            .unwrap();
        assert_eq!(task.information, "Test task");
        assert_eq!(task.priority, Priority::Low);
        assert_eq!(task.status, Status::Todo);
    }

    #[test]
    fn task_builder_builds_with_priority_and_status() {
        setup();
        let task = Builder::new()
            .information("Test task".to_string())
            .priority(Some(Priority::Urgent))
            .status(Some(Status::InProgress))
            .build()
            .unwrap();
        assert_eq!(task.priority, Priority::Urgent);
        assert_eq!(task.status, Status::InProgress);
    }

    #[test]
    fn task_builder_fails_with_empty_information() {
        setup();
        let result = Builder::new().information("".to_string()).build();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Err::Domain(DomErr::EmptyTaskInformation)
        ));
    }

    #[test]
    fn task_builder_fails_with_whitespace_information() {
        setup();
        let result = Builder::new().information("   ".to_string()).build();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Err::Domain(DomErr::EmptyTaskInformation)
        ));
    }

    #[test]
    fn task_builder_with_id_builds_correctly() {
        setup();
        let task = Builder::new()
            .id(42)
            .information("Test task".to_string())
            .priority(Some(Priority::High))
            .status(Some(Status::Done))
            .build_with_id()
            .unwrap();
        assert_eq!(task.id(), 42);
        assert_eq!(task.information(), "Test task");
        assert_eq!(task.priority(), &Priority::High);
        assert_eq!(task.status(), &Status::Done);
    }

    #[test]
    fn task_builder_with_id_fails_without_id() {
        setup();
        let result = Builder::new()
            .information("Test task".to_string())
            .build_with_id();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Err::Domain(DomErr::InvalidID)
        ));
    }

    #[test]
    fn task_builder_information_update_replaces_value() {
        setup();
        let original = Builder::new()
            .id(1)
            .information("Original".to_string())
            .build_with_id()
            .unwrap();
        let updated = Builder::from_task(&original)
            .information_update(Some("Updated".to_string()))
            .build_with_id()
            .unwrap();
        assert_eq!(updated.information(), "Updated");
    }

    #[test]
    fn task_builder_information_update_keeps_original_when_none() {
        setup();
        let original = Builder::new()
            .id(1)
            .information("Original".to_string())
            .build_with_id()
            .unwrap();
        let updated = Builder::from_task(&original)
            .information_update(None)
            .build_with_id()
            .unwrap();
        assert_eq!(updated.information(), "Original");
    }

    #[test]
    fn task_builder_from_task_preserves_all_fields() {
        setup();
        let original = Builder::new()
            .id(7)
            .information("Original".to_string())
            .priority(Some(Priority::High))
            .status(Some(Status::Blocked))
            .build_with_id()
            .unwrap();
        let copy = Builder::from_task(&original).build_with_id().unwrap();
        assert_eq!(copy.id(), original.id());
        assert_eq!(copy.information(), original.information());
        assert_eq!(copy.priority(), original.priority());
        assert_eq!(copy.status(), original.status());
    }

    // ── Filter ────────────────────────────────────────────────────────────────

    #[test]
    fn filter_default_values() {
        setup();
        let filter = Filter::default();
        assert_eq!(filter.word, None);
        assert_eq!(filter.status, None);
        assert_eq!(filter.priority, None);
        assert_eq!(filter.page, 0);
        assert_eq!(filter.page_size, DEFAULT_PAGE_SIZE);
    }

    #[test]
    fn filter_with_word() {
        setup();
        let filter = Filter {
            word: Some("api".to_string()),
            ..Filter::default()
        };
        assert_eq!(filter.word, Some("api".to_string()));
    }

    #[test]
    fn filter_with_priority_and_status() {
        setup();
        let filter = Filter {
            priority: Some(Priority::Urgent),
            status: Some(Status::Blocked),
            ..Filter::default()
        };
        assert_eq!(filter.priority, Some(Priority::Urgent));
        assert_eq!(filter.status, Some(Status::Blocked));
    }

    // ── Sort ──────────────────────────────────────────────────────────────────

    #[test]
    fn sort_default_is_asc_no_field() {
        setup();
        let sort = Sort::default();
        assert_eq!(sort.field, None);
        assert_eq!(sort.order, SortOrder::Asc);
    }

    #[test]
    fn sort_with_field_and_desc_order() {
        setup();
        let sort = Sort {
            field: Some(SortField::Priority),
            order: SortOrder::Desc,
        };
        assert_eq!(sort.field, Some(SortField::Priority));
        assert_eq!(sort.order, SortOrder::Desc);
    }

    // ── Priority ──────────────────────────────────────────────────────────────

    #[test]
    fn priority_variants_are_distinct() {
        setup();
        assert_ne!(Priority::Low, Priority::Medium);
        assert_ne!(Priority::Medium, Priority::High);
        assert_ne!(Priority::High, Priority::Urgent);
    }

    // ── Status ────────────────────────────────────────────────────────────────

    #[test]
    fn status_variants_are_distinct() {
        setup();
        assert_ne!(Status::Todo, Status::InProgress);
        assert_ne!(Status::InProgress, Status::Blocked);
        assert_ne!(Status::Blocked, Status::Done);
    }
}
