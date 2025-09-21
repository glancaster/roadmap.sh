# Expense Tracker CLI

Track your expenses locally through the cli

Based on https://roadmap.sh/projects/expense-tracker


## Building the Project

```bash
cargo build --release
```
Or to install to PATH
```bash
cargo install --path .
```

## Available Commands Overview

```
Usage: expense_tracker <COMMAND>

Commands:
  add      Add an Expense with description and amount
  list     View all Expenses
  update   Update an Expense from ID
  delete   Delete an Expense from ID
  summary  Summary for all Expenses. Optional Filter for Month
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---

###  Future Features
-  Category support
-  Export functionality
-  Budget tracking
-  Category filtering
-  CSV export
-  Date range queries
-  Expense analytics

---

## Technical Details

- **Language**: Rust
- **CLI Framework**: clap (with derive macros)
- **Database**: SQLite via rusqlite
- **Datetime**: chrono crate
- **Logging**: env_logger + log crates




