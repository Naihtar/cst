use crate::{infrastructure::cli::ui::colors::*, prelude::Settings};

/// Returns the formatted version string with author, repository, and license info.
/// Returns the formatted version string with author, repository, and license info.
pub fn format_version() -> String {
    format!(
        "{cyan}{title}{reset} v{version}{reset}\n\
         - {desc}{reset}\n\
         - {white}{label_author}{reset}: {authors}\n\
         - {white}{label_repo}{reset}: {repo}\n\
         - {white}{label_license}{reset}: {license}",
        cyan = CYAN,
        title = Settings::t("help.title"),
        version = env!("CARGO_PKG_VERSION"),
        desc = env!("CARGO_PKG_DESCRIPTION"),
        authors = env!("CARGO_PKG_AUTHORS"),
        repo = env!("CARGO_PKG_REPOSITORY"),
        license = env!("CARGO_PKG_LICENSE"),
        label_author = Settings::t("help.version_author"),
        label_repo = Settings::t("help.version_repository"),
        label_license = Settings::t("help.version_license"),
        white = WHITE,
        reset = RESET,
    )
}

/// Returns the formatted current config: DB path and active language.
pub fn format_config(settings: &Settings) -> String {
    [
        CYAN.to_string() + &Settings::t("help.cmd_config") + RESET,
        format!(
            "  {WHITE}{}{RESET}: {}",
            Settings::t("help.config_db"),
            settings.db_path
        ),
        format!(
            "  {WHITE}{}{RESET}: {}",
            Settings::t("help.config_lang"),
            settings.language.as_str()
        ),
    ]
    .join("\n")
}

/// Returns the full formatted help text with all commands, modifiers, and examples.
pub fn format_help() -> String {
    [
        CYAN.to_string() + &Settings::t("help.title") + RESET,
        YELLOW.to_string() + &Settings::t("help.commands") + RESET,
        format!(
            "  {:<40}{GREEN}cst -A<modifier> <information>{RESET}",
            Settings::t("help.cmd_add") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -L<modifier>{RESET}",
            Settings::t("help.cmd_list") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -P<modifier> @<page>{RESET}",
            Settings::t("help.cmd_paged") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -G <id>{RESET}",
            Settings::t("help.cmd_get") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -D <id>{RESET}",
            Settings::t("help.cmd_done") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -D 1,2,3{RESET}",
            Settings::t("help.cmd_done_many") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -U<modifier> <id>{RESET}",
            Settings::t("help.cmd_update") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -U<modifier> <id> <information>{RESET}",
            Settings::t("help.cmd_update_text") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -U<modifier> 1,2,3{RESET}",
            Settings::t("help.cmd_update_many") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -R <id>{RESET}",
            Settings::t("help.cmd_remove") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -R 1,2,3{RESET}",
            Settings::t("help.cmd_remove_many") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -F<modifier> <word> @<page>{RESET}",
            Settings::t("help.cmd_filter") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -X{RESET}",
            Settings::t("help.cmd_clear") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -Z{RESET}",
            Settings::t("help.cmd_undo") + ":"
        ),
        // ── Import ─────────────────────────────────────────────────────────────
        format!(
            "  {:<40}{GREEN}cst -I <path>{RESET}",
            Settings::t("help.cmd_import") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -Id <path>{RESET}",
            Settings::t("help.cmd_import_dry_run") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -Ir <path>{RESET}",
            Settings::t("help.cmd_import_restore") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -Ij <path>{RESET}",
            Settings::t("help.cmd_import_json") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -Ia <path>{RESET}",
            Settings::t("help.cmd_import_yaml") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -It <path>{RESET}",
            Settings::t("help.cmd_import_toml") + ":"
        ),
        // ── Export ─────────────────────────────────────────────────────────────
        format!(
            "  {:<40}{GREEN}cst -S{RESET}",
            Settings::t("help.cmd_export_csv") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -M{RESET}",
            Settings::t("help.cmd_export_markdown") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -E{RESET}",
            Settings::t("help.cmd_export_excel") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -J{RESET}",
            Settings::t("help.cmd_export_json") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -Y{RESET}",
            Settings::t("help.cmd_export_yaml") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -T{RESET}",
            Settings::t("help.cmd_export_toml") + ":"
        ),
        // ── Misc ───────────────────────────────────────────────────────────────
        format!(
            "  {:<40}{GREEN}cst -C{RESET}",
            Settings::t("help.config_info") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -C<modifier> <value>{RESET}",
            Settings::t("help.cmd_config") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -H{RESET}",
            Settings::t("help.cmd_help") + ":"
        ),
        format!(
            "  {:<40}{GREEN}cst -V{RESET}",
            Settings::t("help.cmd_version") + ":"
        ),
        // ── Modifiers ──────────────────────────────────────────────────────────
        YELLOW.to_string() + &Settings::t("help.config_modifiers") + RESET,
        format!("  {WHITE}l{RESET}: {}", Settings::t("help.config_lang")),
        format!("  {WHITE}d{RESET}: {}", Settings::t("help.config_db")),
        YELLOW.to_string() + &Settings::t("help.priority_modifiers") + RESET,
        format!("  {WHITE}l{RESET}: {}", Settings::t("help.priority_low")),
        format!(
            "  {YELLOW}m{RESET}: {}",
            Settings::t("help.priority_medium")
        ),
        format!("  {RED}h{RESET}: {}", Settings::t("help.priority_high")),
        format!(
            "  {BRIGHT_RED}u{RESET}: {}",
            Settings::t("help.priority_urgent")
        ),
        YELLOW.to_string() + &Settings::t("help.status_modifiers") + RESET,
        format!("  {WHITE}t{RESET}: {}", Settings::t("help.status_todo")),
        format!(
            "  {CYAN}w{RESET}: {}",
            Settings::t("help.status_in_progress")
        ),
        format!("  {RED}b{RESET}: {}", Settings::t("help.status_blocked")),
        format!("  {GREEN}d{RESET}: {}", Settings::t("help.status_done")),
        YELLOW.to_string() + &Settings::t("help.sort_modifiers") + RESET,
        format!("  {WHITE}p{RESET}: {}", Settings::t("help.sort_priority")),
        format!("  {WHITE}s{RESET}: {}", Settings::t("help.sort_status")),
        format!("  {WHITE}i{RESET}: {}", Settings::t("help.sort_id")),
        format!("  {WHITE}+{RESET}: {}", Settings::t("help.sort_asc")),
        format!("  {WHITE}-{RESET}: {}", Settings::t("help.sort_desc")),
        YELLOW.to_string() + &Settings::t("help.language_modifiers") + RESET,
        format!("  {WHITE}en{RESET}: {}", Settings::t("help.lang_en")),
        format!("  {WHITE}es{RESET}: {}", Settings::t("help.lang_es")),
    ]
    .join("\n")
}
