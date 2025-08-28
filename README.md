# Fine Directory Curator (fdc)

🗂️ A fast, elegant Rust CLI for keeping your macOS & Linux folders tidy (Windows support planned).  
It sorts first-level items by **year** and **type**, avoids overwrite collisions, preserves top-level structure, and lets you **preview** before doing anything destructive.

> TL;DR: `cargo install fine-directory-curator` → `fdc --dry-run` → review → `fdc`  
> No drama, no duplicates, no "where-did-that-download-go?" energy.

---

## Table of Contents
- [Why fdc?](#why-fdc)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [CLI Reference](#cli-reference)
  - [Commands](#commands)
  - [Global Flags](#global-flags)
  - [Examples](#examples)
- [Configuration](#configuration)
  - [File layout & rules](#file-layout--rules)
  - [Extension categories](#extension-categories)
- [Behavior & Guarantees](#behavior--guarantees)
- [Operating System Support](#operating-system-support)
- [Development](#development)
- [Troubleshooting & FAQ](#troubleshooting--faq)
- [Roadmap](#roadmap)
- [License](#license)

---

## Why fdc?

Because the messy reality is: **Downloads** fills up, collisions happen, and recursive tools are risky.  
**fdc** is opinionated on purpose:
- **Shallow & safe**: Only the **first level** of the source directory is organized. Directories are moved whole; no deep traversal surprises.
- **Year-first**: Time is the best primary key for "where does this belong?"
- **Non-overwriting**: Uses `name (1).ext`, `name (2).ext`, … instead of clobbering.
- **Preview-first**: `--dry-run` to see exactly what will change.
- **Fast & portable**: Written in Rust, runs great on macOS and Linux.

---

## Installation

### From crates.io (recommended)

```bash
cargo install fine-directory-curator
# binary will be available on PATH as:
fdc
```

> Note: Package name is `fine-directory-curator`, executable is `fdc`.

### Build from source

```bash
# in repo root
cargo build --release
# binary:
./target/release/fdc
```

---

## Quick Start

```bash
# See help
fdc --help

# Use defaults (source: ~/Downloads, target: ~/Documents/Matrixs)
fdc

# Dry run (no changes)
fdc --dry-run

# Custom source & target
fdc -s ~/Downloads -t ~/Documents/Matrixs

# Set source path in config only (no execution)
fdc -s ~/Downloads

# Verbose logging (stackable)
fdc -v
fdc -vv
```

---

## CLI Reference

### Commands

| Command | Alias | Purpose | Notes |
|---|---|---|---|
| `fdc` | — | Run organizer with current config (or defaults) | Use with flags to customize run |
| `fdc init-config` | — | Create a default config file | No overwrite; prompts if exists |
| `fdc config` | — | Print resolved configuration | Useful for debugging |
| `fdc set-source <PATH>` | — | Set source directory in config | Updates config file only |
| `fdc set-target <PATH>` | — | Set target root in config | Updates config file only |
| `fdc --help` | `-h` | Show help | — |
| `fdc --version` | `-V` | Show version | SemVer |

### Global Flags

| Flag | Alias | Type | Default | Effect | Example |
|---|---|---|---|---|---|
| `--dry-run` | — | bool | `false` | Plan only; print actions; do not modify FS | `fdc --dry-run` |
| `--source <DIR>` | `-s` | path | `~/Downloads` | Override source directory | `fdc -s ~/Desktop` |
| `--target <DIR>` | `-t` | path | `~/Documents/Matrixs` | Override target root | `fdc -t ~/Archive` |
| `--verbose` | `-v` | count | `0` | Increase log detail (stackable) | `fdc -vv` |
| `--help` | `-h` | — | — | Show usage | `fdc -h` |
| `--version` | `-V` | — | — | Show version | `fdc -V` |

> **Smart Flag Behavior**: When only `-s` is provided (without `-t`, `--dry-run`, or `-v`), fdc will only update the config file and not execute any file operations.

### Examples

| Goal | Command |
|---|---|
| Preview what tomorrow's cleanup would do | `fdc --dry-run` |
| Move only your current Downloads into your structured vault | `fdc` |
| Use a custom target root | `fdc -t ~/Documents/Matrixs` |
| Run from a different source folder | `fdc -s ~/Desktop` |
| Set source path in config only | `fdc -s ~/Desktop` |
| Set source path in config only (explicit) | `fdc set-source ~/Desktop` |
| Set target in config only | `fdc set-target ~/Archive` |
| Turn on detailed logs | `fdc -vv` |
| Initialize (or re-check) your config file | `fdc init-config` ; `fdc config` |

---

## Configuration

On first run, fdc writes a default config to:

- **macOS**: `~/Library/Application Support/fine-directory-curator/config.toml`  
- **Linux**: `~/.config/fine-directory-curator/config.toml`

> Prefer `fdc init-config` if you want to create it proactively.

### Sample `config.toml`
```toml
source_dir = "~/Downloads"
target_dir = "~/Documents/Matrixs"

[sort_rule]
# Sorting priority; left to right
order = ["year", "category"]

# Map specific extensions to custom categories (optional)
[extension_overrides]
# xmind = "mindmaps"
# heic  = "images"
```

### File layout & rules

- **Year** comes first, derived by:
  1) file **creation time**;  
  2) fallback to **modification time**;  
  3) fallback to **current year** if neither is available.
- **Category** comes second (see below).
- **Structure example**:
  ```
  <target_dir>/
    2025/
      images/
      documents/
      videos/
      ...
  ```

### Extension categories
fdc maps extensions (case-insensitive) to these buckets:

- `images`, `pdfs`, `videos`, `audio`, `archives`,  
  `documents`, `spreadsheets`, `presentations`,  
  `code`, `design`, `mindmaps`, `executables`,  
  `installers`, `fonts`, `others`, `directory`

> Directories are **not** traversed deeply; they are treated as a single item and moved under `directory/`.

---

## Behavior & Guarantees

- **Non-overwrite strategy**: if a target path exists, fdc writes `name (1).ext`, `name (2).ext`, … until it finds a free slot.
- **Shallow operation**: only first-level entries in `source_dir` are processed.
- **Cross-volume safe**: moves across filesystems are done as **copy + delete**.
- **Idempotent-ish**: running again won't duplicate files in-place thanks to non-overwrite naming.
- **Safety first**: Always start with `--dry-run` on new setups.

---

## Operating System Support

| OS | Status | Notes |
|---|---|---|
| **macOS** | ✅ Supported | Tested on Apple Silicon & Intel |
| **Linux** | ✅ Supported | XDG config path used by default |
| **Windows** | 🛤️ Planned | Path semantics & metadata handling to be added |

> macOS users: if organizing outside your home folder, you may need to grant Terminal (or your shell) **Full Disk Access**.

---

## Development

A lean workflow that keeps quality high and iteration fast.

### Prerequisites
- Rust 1.75+ (via `rustup`)
- `cargo fmt`, `clippy`, and `cargo test`

### Common tasks
```bash
# Format
cargo fmt

# Lint strictly
cargo clippy -- -D warnings

# Unit & integration tests
cargo test

# Release build
cargo build --release
```

### Suggested developer UX (optional but recommended)
- Use a `justfile` or `Makefile` for one-liners:
  ```makefile
  build: ; cargo build --release
  check: ; cargo fmt --check && cargo clippy -- -D warnings && cargo test
  release: check build
  ```
- Add a pre-commit hook:
  ```bash
  # .git/hooks/pre-commit
  cargo fmt -- --check &&
  cargo clippy -- -D warnings &&
  cargo test
  ```
- Versioning & publishing: use `cargo-release` for tagging, changelogs, and `cargo publish` hygiene.

---

## Troubleshooting & FAQ

**Q: Nothing moved. What gives?**  
A: Run with `-vv` to inspect decisions. Ensure the source directory contains items at the top level. fdc does not recurse into subfolders.

**Q: Why “year” first?**  
A: Time is universal across file types and workflows. It keeps archives navigable even when categories blur.

**Q: Can I customize categories?**  
A: Yes, with `[extension_overrides]` in `config.toml`. Unknown extensions go to `others/`.

**Q: Is `--dry-run` truly non-destructive?**  
A: Yes. It only prints the plan; no filesystem writes occur.

**Q: Handling duplicates?**  
A: fdc never overwrites. It picks the next available `name (N).ext`.

**Q: Will fdc change file metadata?**  
A: Moves generally preserve metadata; cross-device copy + delete may differ per FS. Original timestamps are used for routing; they are not rewritten.

---

## Roadmap

- ✅ macOS & Linux parity  
- ⏭️ Windows support (path, timestamps, alternate data streams)  
- ⏭️ Configurable naming templates (e.g., `YYYY/category` vs `category/YYYY`)  
- ⏭️ Include/ignore patterns (e.g., `.fdcignore`)  
- ⏭️ Summary report output (`--report json|markdown`)  
- ⏭️ Dry-run diff to file (`--plan <path>`)  

> Have opinions? Open an issue or PR—let’s keep the filesystem civilized together.

---

## License

[MIT](http://opensource.org/licenses/MIT) © 2025–present, [逍遥自在轩](https://www.niceshare.site/)。