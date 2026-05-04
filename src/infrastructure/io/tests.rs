#[cfg(test)]
mod tests {
    use crate::{
        infrastructure::io::mappers::{
            priority_to_str, status_to_str, str_to_priority, str_to_status,
        },
        prelude::{Priority, Progress, Status},
    };

    // ── priority_to_str ───────────────────────────────────────────────────────

    #[test]
    fn priority_to_str_maps_all_variants() {
        assert_eq!(priority_to_str(&Priority::Low), "low");
        assert_eq!(priority_to_str(&Priority::Medium), "medium");
        assert_eq!(priority_to_str(&Priority::High), "high");
        assert_eq!(priority_to_str(&Priority::Urgent), "urgent");
    }

    // ── status_to_str ─────────────────────────────────────────────────────────

    #[test]
    fn status_to_str_maps_all_variants() {
        assert_eq!(status_to_str(&Status::Todo), "todo");
        assert_eq!(status_to_str(&Status::InProgress), "in-progress");
        assert_eq!(status_to_str(&Status::Blocked), "blocked");
        assert_eq!(status_to_str(&Status::Done), "done");
    }

    // ── str_to_priority ───────────────────────────────────────────────────────

    #[test]
    fn str_to_priority_maps_valid_strings() {
        assert_eq!(str_to_priority("low"), Some(Priority::Low));
        assert_eq!(str_to_priority("medium"), Some(Priority::Medium));
        assert_eq!(str_to_priority("high"), Some(Priority::High));
        assert_eq!(str_to_priority("urgent"), Some(Priority::Urgent));
    }

    #[test]
    fn str_to_priority_is_case_insensitive() {
        assert_eq!(str_to_priority("LOW"), Some(Priority::Low));
        assert_eq!(str_to_priority("Medium"), Some(Priority::Medium));
        assert_eq!(str_to_priority("HIGH"), Some(Priority::High));
        assert_eq!(str_to_priority("URGENT"), Some(Priority::Urgent));
    }

    #[test]
    fn str_to_priority_trims_whitespace() {
        assert_eq!(str_to_priority("  low  "), Some(Priority::Low));
        assert_eq!(str_to_priority(" high "), Some(Priority::High));
    }

    #[test]
    fn str_to_priority_returns_none_for_unknown() {
        assert_eq!(str_to_priority("critical"), None);
        assert_eq!(str_to_priority(""), None);
        assert_eq!(str_to_priority("unknown"), None);
    }

    // ── str_to_status ─────────────────────────────────────────────────────────

    #[test]
    fn str_to_status_maps_valid_strings() {
        assert_eq!(str_to_status("todo"), Some(Status::Todo));
        assert_eq!(str_to_status("in-progress"), Some(Status::InProgress));
        assert_eq!(str_to_status("inprogress"), Some(Status::InProgress));
        assert_eq!(str_to_status("blocked"), Some(Status::Blocked));
        assert_eq!(str_to_status("done"), Some(Status::Done));
    }

    #[test]
    fn str_to_status_is_case_insensitive() {
        assert_eq!(str_to_status("TODO"), Some(Status::Todo));
        assert_eq!(str_to_status("In-progress"), Some(Status::InProgress));
        assert_eq!(str_to_status("BLOCKED"), Some(Status::Blocked));
        assert_eq!(str_to_status("DONE"), Some(Status::Done));
    }

    #[test]
    fn str_to_status_trims_whitespace() {
        assert_eq!(str_to_status("  todo  "), Some(Status::Todo));
        assert_eq!(str_to_status(" done "), Some(Status::Done));
    }

    #[test]
    fn str_to_status_returns_none_for_unknown() {
        assert_eq!(str_to_status("pending"), None);
        assert_eq!(str_to_status(""), None);
        assert_eq!(str_to_status("unknown"), None);
    }

    // ── Round-trip ────────────────────────────────────────────────────────────

    #[test]
    fn priority_roundtrip() {
        let priorities: &[Priority] = &[
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Urgent,
        ];
        for priority in priorities {
            let s = priority_to_str(priority);
            assert_eq!(str_to_priority(s), Some(*priority));
        }
    }

    #[test]
    fn status_roundtrip() {
        let statuses: &[Status] = &[
            Status::Todo,
            Status::InProgress,
            Status::Blocked,
            Status::Done,
        ];
        for status in statuses {
            let s = status_to_str(status);
            assert_eq!(str_to_status(s), Some(*status));
        }
    }

    // ── Progress ──────────────────────────────────────────────────────────────

    #[test]
    fn progress_elapsed_is_non_negative() {
        let p = Progress::new();
        assert!(p.elapsed_secs() >= 0.0);
    }

    #[test]
    fn progress_elapsed_increases_over_time() {
        let p = Progress::new();
        let t1 = p.elapsed_secs();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let t2 = p.elapsed_secs();
        assert!(t2 > t1);
    }

    #[test]
    fn progress_default_same_as_new() {
        let p1 = Progress::new();
        let p2 = Progress::default();
        assert!(p1.elapsed_secs() >= 0.0);
        assert!(p2.elapsed_secs() >= 0.0);
    }
}
