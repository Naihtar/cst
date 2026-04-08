# CST — Command-line Simple Task

[![Version](https://img.shields.io/github/v/tag/Naihtar/cst?label=version&style=flat-square)](https://github.com/Naihtar/cst/releases)
[![CI](https://github.com/Naihtar/cst/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/Naihtar/cst/actions)
[![Rust Edition](https://img.shields.io/badge/rust-2024-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey?style=flat-square)](https://github.com/Naihtar/cst)
[![License](https://img.shields.io/badge/license-GPL--v3-blue?style=flat-square)](LICENSE)

---

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

## OS Support

Thanks to our CI/CD pipeline, every commit is automatically tested and verified on:

| OS | Status |
|:---|:---:|
| **Linux** | ✅ |
| **macOS** | ✅ |
| **Windows** | ✅ |

---

## Installation

Since CST is written in Rust, you can install it easily using `cargo`:

### From source
```bash
git clone https://github.com/Naihtar/cst
cd cst
cargo install --path .
```

---

## Uninstallation

To completely remove **CST** from your system, follow these steps:

**1. Remove the binary** 
```bash
cargo uninstall cst
```

**2. Clean up data and configuration** 
> Since paths vary depending on the Operating System, run `cst -C` before uninstalling to see your current paths. Then, manually remove the displayed directories.

Typically, these are located at:
* **Linux:** `~/.config/cst/`
* **macOS:** `~/Library/Application Support/cst/`
* **Windows:** `%APPDATA%\<Author>\cst\`

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

CST uses a `.env` file to manage its settings. Since paths are resolved dynamically based on your Operating System, you can check your specific configuration and database locations at any time by running: 
```bash
cst -C
```
**Available Settings:**

* **DB_PATH:** Path to the SQLite database file.
* **LANGUAGE:** Interface language (`en` / `es`).
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

- [ ] **Style configuration** — custom colors and formatting, with compatibility for Windows terminals?

---

## AI Assistance

This project was developed with AI assistance ([Claude](https://claude.ai)) in the following areas:

- **Localization** — translation of UI messages into English and Spanish
- **Testing** — design and implementation of the test suite
- **Documentation** — `rustdoc` comments and README
- **Code review** — Rust conventions, clean architecture guidance, and API design

---

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).  
You are free to use, modify, and distribute this software, provided that any derivative work is also distributed under the same license.
