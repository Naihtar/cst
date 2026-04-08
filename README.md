# CST — Command-line Task Manager

[![Version](https://img.shields.io/badge/version-1.0.0-blue?style=flat-square)](https://github.com/Naihtar/cst)
[![Rust](https://img.shields.io/badge/rust-2024_edition-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-GPL--v3-green?style=flat-square)](LICENSE)

> A fast, lightweight task manager for the terminal — built in Rust.

---

 ![CST Demo](assets/gif/demo.gif) 

---

## Features

- **Full task lifecycle** — create, update, filter, and delete tasks from the terminal
- **Priority & status tracking** — four priority levels and four status states, configurable via single-character modifiers
- **Sorting & filtering** — sort by ID, priority, or status; filter by keyword with pagination support
- **Multi-format import/export** — CSV, JSON, YAML, TOML, Markdown, and Excel
- **Undo support** — revert the last destructive operation instantly
- **Multilingual** — English and Spanish; change language with a single command, applies from the next invocation
- **Zero external dependencies at runtime** — SQLite bundled, configuration stored locally

---

## Installation

**Linux**

### Step by step
```bash
git clone https://github.com/Naihtar/cst
cd cst
cargo install --path .
```

**macOS** — *(TODO: untested — no macOS environment available)*

**Windows** — *(TODO: untested — no Windows environment available)*

---

## Usage

```
cst -<COMMAND>[modifiers] [arguments]
```

Commands use **uppercase** letters. Modifiers are **lowercase** letters appended directly to the command flag.

---

## Commands

### Task Management

| Command | Description | Example |
|---------|-------------|---------|
| `-A<modifier> <text>` | Add a task | `cst -Ah "Fix login bug"` |
| `-L[modifier]` | List all tasks | `cst -Lp-` |
| `-P[modifier] @<page>` | List paginated tasks | `cst -P @2` |
| `-G <id>` | Get task by ID | `cst -G 5` |
| `-D <id\|ids>` | Mark task(s) as done | `cst -D 1,2,3` |
| `-U<modifier> <id\|ids> [text]` | Update task(s) | `cst -Uh 3 "New description"` |
| `-R <id\|ids>` | Remove task(s) | `cst -R 5` |
| `-F<modifier> <word> [@page]` | Filter tasks | `cst -Fh api @1` |
| `-X` | Clear all tasks | `cst -X` |
| `-Z` | Undo last operation | `cst -Z` |

### Import

| Command | Description |
|---------|-------------|
| `-I <path>` | Import from CSV (append) |
| `-Ij <path>` | Import from JSON |
| `-Ia <path>` | Import from YAML |
| `-It <path>` | Import from TOML |
| `-Id <path>` | Dry-run — preview without writing |
| `-Ir <path>` | Import and restore DB |

### Export

| Command | Description |
|---------|-------------|
| `-S [path]` | Export to CSV |
| `-J [path]` | Export to JSON |
| `-Y [path]` | Export to YAML |
| `-T [path]` | Export to TOML |
| `-M [path]` | Export to Markdown |
| `-E [path]` | Export to Excel |

### Configuration & Info

| Command | Description |
|---------|-------------|
| `-C` | Show current configuration |
| `-Cl <lang>` | Set language (`en` / `es`) |
| `-Cd <path>` | Set database path |
| `-H` | Show help |
| `-V` | Show version |

---

## Modifiers

Modifiers are appended directly to the command flag, with no separator.

### Priority

| Modifier | Priority |
|----------|----------|
| `l` | Low *(default)* |
| `m` | Medium |
| `h` | High |
| `u` | Urgent |

### Status

| Modifier | Status |
|----------|--------|
| `t` | Todo *(default)* |
| `w` | In Progress |
| `b` | Blocked |
| `d` | Done |

### Sort

| Modifier | Description |
|----------|-------------|
| `i` | Sort by ID |
| `p` | Sort by Priority |
| `s` | Sort by Status |
| `+` | Ascending *(default)* |
| `-` | Descending |

### Confirmation

| Modifier | Description |
|----------|-------------|
| `y` | Skip confirmation prompt |

---

## Examples

```bash
# Add a high-priority task
cst -Ah "Implement authentication"

# Add an urgent task currently in progress
cst -Auw "Fix production bug"

# List tasks sorted by priority, descending
cst -Lp-

# Filter blocked tasks containing "api"
cst -Fb api

# Update tasks 1, 2, and 3 to urgent and blocked
cst -Uub 1,2,3

# Remove task 7 without confirmation
cst -Ry 7

# Preview a CSV import before committing
cst -Id tasks.csv

# Import JSON and restore the database
cst -Ijr tasks.json

# Export to Excel
cst -E ~/Documents/tasks.xlsx

# Change language to Spanish
cst -Cl es

# Undo last operation
cst -Z
```

---

## Configuration

CST stores its configuration in the OS config directory:

| OS | Path |
|----|------|
| Linux | `~/.config/cst/.env` |
| macOS | (TODO: untested — no macOS environment available)|
| Windows | (TODO: untested — no Windows environment available) |

```env
DB_PATH=/your/custom/path/to/tasks.db
# Interface language (en, es)
LANGUAGE=en
```
---

## Supported Formats

| Format | Import | Export |
|--------|:------:|:------:|
| CSV | ✅ | ✅ |
| JSON | ✅ | ✅ |
| YAML | ✅ | ✅ |
| TOML | ✅ | ✅ |
| Markdown | ❌ | ✅ |
| Excel | ❌ | ✅ |

---

## Built With

| Crate | Purpose |
|-------|---------|
| `rusqlite` | SQLite database (bundled) |
| `serde` | Serialization / deserialization |
| `serde_json` | JSON support |
| `serde_yaml` | YAML support |
| `toml` | TOML support |
| `csv` | CSV parsing and writing |
| `rust_xlsxwriter` | Excel export |
| `rust-embed` | Locale files embedded in binary |
| `dirs` | OS-specific directory resolution |
| `terminal_size` | Dynamic terminal width |
| `thiserror` | Structured error handling |
| `dotenvy` | `.env` configuration loading |

---

## Roadmap

- [ ] **Style configuration** — custom colors and formatting, with compatibility for Windows terminals

---

## AI Assistance

This project was developed with AI assistance ([Claude](https://claude.ai)) in the following areas:

- **Localization** — translation of UI messages into English and Spanish
- **Testing** — design and implementation of the test suite
- **Documentation** — `rustdoc` comments and README
- **Code review** — Rust conventions, clean architecture guidance, and API design

---

## Uninstallation

**Linux**
```bash
cargo uninstall cst
rm -rf ~/.config/cst/
```

**macOS** — *(TODO: untested — no macOS environment available)*

**Windows** — *(TODO: untested — no Windows environment available)*

---

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).  
You are free to use, modify, and distribute this software, provided that any derivative work is also distributed under the same license.
