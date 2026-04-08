#[cfg(test)]
mod tests {
    use crate::prelude::{
        Decision, Priority, SortField, SortOrder, Status, Store, accept, accept_flag,
        get_modifiers, parse_id, parse_ids, to_filter, to_priority, to_priority_char, to_sort,
        to_sort_field, to_sort_order, to_status, to_status_char,
    };

    fn setup() {
        Store::init_for_tests();
    }

    // ── to_priority_char ──────────────────────────────────────────────────────

    #[test]
    fn char_to_priority_maps_correctly() {
        setup();
        assert_eq!(to_priority_char('l'), Some(Priority::Low));
        assert_eq!(to_priority_char('m'), Some(Priority::Medium));
        assert_eq!(to_priority_char('h'), Some(Priority::High));
        assert_eq!(to_priority_char('u'), Some(Priority::Urgent));
        assert_eq!(to_priority_char('x'), None);
    }

    // ── to_status_char ────────────────────────────────────────────────────────

    #[test]
    fn char_to_status_maps_correctly() {
        setup();
        assert_eq!(to_status_char('t'), Some(Status::Todo));
        assert_eq!(to_status_char('w'), Some(Status::InProgress));
        assert_eq!(to_status_char('b'), Some(Status::Blocked));
        assert_eq!(to_status_char('d'), Some(Status::Done));
        assert_eq!(to_status_char('x'), None);
    }

    // ── char_to_sort_field ────────────────────────────────────────────────────

    #[test]
    fn char_to_sort_field_maps_correctly() {
        setup();
        assert_eq!(to_sort_field('p'), Some(SortField::Priority));
        assert_eq!(to_sort_field('s'), Some(SortField::Status));
        assert_eq!(to_sort_field('i'), Some(SortField::Id));
        assert_eq!(to_sort_field('x'), None);
    }

    // ── char_to_sort_order ────────────────────────────────────────────────────

    #[test]
    fn char_to_sort_order_maps_correctly() {
        setup();
        assert_eq!(to_sort_order('+'), Some(SortOrder::Asc));
        assert_eq!(to_sort_order('-'), Some(SortOrder::Desc));
        assert_eq!(to_sort_order('x'), None);
    }

    // ── extract_modifiers ─────────────────────────────────────────────────────

    #[test]
    fn extract_modifiers_returns_chars_after_uppercase() {
        setup();
        assert_eq!(get_modifiers("-Rhm"), vec!['h', 'm']);
        assert_eq!(get_modifiers("-Lp+"), vec!['p', '+']);
        assert_eq!(get_modifiers("-L"), Vec::<char>::new());
    }

    #[test]
    fn extract_modifiers_ignores_non_alpha() {
        setup();
        assert_eq!(get_modifiers("-R123"), Vec::<char>::new());
    }

    // ── modifiers_to_priority ─────────────────────────────────────────────────

    #[test]
    fn modifiers_to_priority_finds_first_match() {
        setup();
        assert_eq!(to_priority(&['h']), Some(Priority::High));
        assert_eq!(to_priority(&['u']), Some(Priority::Urgent));
        assert_eq!(to_priority(&['x']), None);
    }

    // ── modifiers_to_status ───────────────────────────────────────────────────

    #[test]
    fn modifiers_to_status_finds_first_match() {
        setup();
        assert_eq!(to_status(&['d']), Some(Status::Done));
        assert_eq!(to_status(&['w']), Some(Status::InProgress));
        assert_eq!(to_status(&['x']), None);
    }

    // ── modifiers_to_sort ─────────────────────────────────────────────────────

    #[test]
    fn modifiers_to_sort_default_order_is_asc() {
        setup();
        let sort = to_sort(&['p']);
        assert_eq!(sort.field, Some(SortField::Priority));
        assert_eq!(sort.order, SortOrder::Asc);
    }

    #[test]
    fn modifiers_to_sort_desc_order() {
        setup();
        let sort = to_sort(&['s', '-']);
        assert_eq!(sort.field, Some(SortField::Status));
        assert_eq!(sort.order, SortOrder::Desc);
    }

    #[test]
    fn modifiers_to_sort_no_field() {
        setup();
        let sort = to_sort(&[]);
        assert_eq!(sort.field, None);
        assert_eq!(sort.order, SortOrder::Asc);
    }

    // ── modifiers_to_filter ───────────────────────────────────────────────────

    #[test]
    fn modifiers_to_filter_builds_correctly() {
        setup();
        let filter = to_filter(&['h', 'd'], Some("api".to_string()), 0);
        assert_eq!(filter.priority, Some(Priority::High));
        assert_eq!(filter.status, Some(Status::Done));
        assert_eq!(filter.word, Some("api".to_string()));
        assert_eq!(filter.page, 0);
    }

    #[test]
    fn modifiers_to_filter_no_modifiers() {
        setup();
        let filter = to_filter(&[], None, 2);
        assert_eq!(filter.priority, None);
        assert_eq!(filter.status, None);
        assert_eq!(filter.word, None);
        assert_eq!(filter.page, 2);
    }

    // ── parse_id ──────────────────────────────────────────────────────────────

    #[test]
    fn parse_id_valid() {
        setup();
        assert_eq!(parse_id("42").unwrap(), 42);
        assert_eq!(parse_id("1").unwrap(), 1);
    }

    #[test]
    fn parse_id_invalid() {
        setup();
        assert!(parse_id("abc").is_err());
        assert!(parse_id("").is_err());
        assert!(parse_id("1.5").is_err());
    }

    // ── parse_ids ─────────────────────────────────────────────────────────────

    #[test]
    fn parse_ids_valid() {
        setup();
        assert_eq!(parse_ids("1,2,3").unwrap(), vec![1, 2, 3]);
        assert_eq!(parse_ids("5").unwrap(), vec![5]);
    }

    #[test]
    fn parse_ids_with_spaces() {
        setup();
        assert_eq!(parse_ids("1, 2, 3").unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn parse_ids_invalid() {
        setup();
        assert!(parse_ids("1,abc,3").is_err());
    }

    // ── accept ────────────────────────────────────────────────────────────────

    #[test]
    fn accept_yes_variants() {
        setup();
        assert!(matches!(accept("y").unwrap(), Decision::Yes));
        assert!(matches!(accept("Y").unwrap(), Decision::Yes));
        assert!(matches!(accept("yes").unwrap(), Decision::Yes));
        assert!(matches!(accept("s").unwrap(), Decision::Yes));
        assert!(matches!(accept("si").unwrap(), Decision::Yes));
        assert!(matches!(accept("sí").unwrap(), Decision::Yes));
    }

    #[test]
    fn accept_no_variants() {
        setup();
        assert!(matches!(accept("n").unwrap(), Decision::No));
        assert!(matches!(accept("N").unwrap(), Decision::No));
        assert!(matches!(accept("no").unwrap(), Decision::No));
    }

    #[test]
    fn accept_invalid_input() {
        setup();
        assert!(accept("maybe").is_err());
        assert!(accept("").is_err());
        assert!(accept("x").is_err());
    }

    // ── accept_flag ───────────────────────────────────────────────────────────

    #[test]
    fn accept_flag_yes() {
        setup();
        assert!(matches!(accept_flag('y'), Some(Decision::Yes)));
        assert!(matches!(accept_flag('Y'), Some(Decision::Yes)));
        assert!(matches!(accept_flag('s'), Some(Decision::Yes)));
        assert!(matches!(accept_flag('S'), Some(Decision::Yes)));
    }

    #[test]
    fn accept_flag_no() {
        setup();
        assert!(matches!(accept_flag('n'), Some(Decision::No)));
        assert!(matches!(accept_flag('N'), Some(Decision::No)));
    }

    #[test]
    fn accept_flag_unknown() {
        setup();
        assert!(accept_flag('x').is_none());
        assert!(accept_flag('z').is_none());
    }
}
