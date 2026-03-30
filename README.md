# standup

A fast CLI tool that summarizes your git commits across multiple repos — built for daily standups, weekly reviews, and freelance invoices. Built with Rust.

```
backend-forge
  f2371a6 Local Development (16:45)
  6d6b4a7 Readme (16:42)
  745d8b6 Docker Hub (16:35)
  9 commits since 7d

── AI Summary ──────────────────────────
I spent the week containerizing the backend service with Docker, publishing
to Docker Hub, and setting up a production API gateway with a CI/CD pipeline.
────────────────────────────────────────
```

---

## Features

- Reads git history across multiple repos in one command
- Filters by author and time range
- Three output formats: plain, markdown, JSON
- Optional AI summary powered by any OpenAI-compatible API (OpenAI, Groq, Ollama)
- Persistent config — add repos once, run forever
- Works fully offline without the `--summarize` flag

---

## Installation

### Prerequisites

- Rust toolchain — install from [rustup.rs](https://rustup.rs)
- Windows: Visual Studio Build Tools with MSVC

### Build from source

```bash
git clone https://github.com/yourname/standup
cd standup
cargo build --release
```

The binary will be at `target/release/standup.exe` (Windows) or `target/release/standup` (macOS/Linux).

Optionally move it to a directory on your PATH:

```bash
# Windows (PowerShell)
Copy-Item target\release\standup.exe $env:USERPROFILE\.cargo\bin\

# macOS / Linux
cp target/release/standup ~/.cargo/bin/
```

---

## Quick Start

```bash
# 1. Initialize config
standup init

# 2. Add your repos
standup add C:\Projects\my-api
standup add C:\Projects\frontend --name "web-app"

# 3. Run your standup
standup --since yesterday
```

---

## Commands

### `standup init`

Interactive setup wizard. Creates `~/.config/standup/config.toml` and prompts for:

- Your name or email (used as default author filter)
- Default time range (e.g. `yesterday`, `7d`)
- API key for AI summarization (optional, can be added later)

```bash
standup init
```

```
Setting up standup...

Your name or email for filtering (press Enter to skip): Abhil
Default time range [yesterday]: 7d
OpenAI API key for --summarize (press Enter to skip):

✓ Config created at C:\Users\abhil\AppData\Roaming\standup\config.toml
Now add repos with `standup add <path>`
```

---

### `standup add <path>`

Add a git repository to your config. Validates that the path exists and is a real git repo before saving.

```bash
standup add C:\Projects\backend
standup add C:\Projects\frontend --name "web-app"
```

| Flag | Description |
|------|-------------|
| `--name` / `-n` | Display name shown in output. Defaults to the folder name. |

---

### `standup list`

Show all configured repos and your current settings.

```bash
standup list
```

```
Configured repos:

  ✓ backend-forge   C:\Projects\backend-forge
  ✓ web-app         C:\Projects\frontend
  ✗ old-project     C:\Projects\old-project   (path missing)

→ Default since: yesterday
→ Default author: Abhil
→ GroqAI API key: set
```

A `✓` means the path exists on disk. A `✗` means the path is missing — that repo will be skipped at runtime with a warning.

---

### `standup remove <name>`

Remove a repo from your config by its display name.

```bash
standup remove old-project
```

---

### `standup set-key <key>`

Save an API key to your config for use with `--summarize`. Works with any OpenAI-compatible provider.

```bash
# OPENAI
standup set-key sk-proj-...

# Groq (free tier available at console.groq.com)
standup set-key gsk_...
```

---

### `standup` (run the report)

Run a standup report across all configured repos, or a single repo with `--repo`.

```bash
standup
standup --since 7d
standup --since yesterday --author "Abhil"
standup --repo C:\Projects\backend --since 3d
standup --since 7d --format markdown
standup --since 7d --summarize
```

| Flag | Default | Description |
|------|---------|-------------|
| `--since` / `-s` | config default | Time range: `today`, `yesterday`, `Nd` (e.g. `7d`), or `YYYY-MM-DD` |
| `--author` / `-a` | config default | Filter by author name or email (partial match) |
| `--repo` / `-r` | — | Run against a single repo path instead of all config repos |
| `--format` / `-f` | `plain` | Output format: `plain`, `markdown`, `json` |
| `--summarize` | off | Generate an AI summary of all commits (requires API key) |

---

## Output Formats

### Plain (default)

Clean terminal output with colors.

```
backend-forge
  f2371a6 Local Development (16:45)
  6d6b4a7 Readme (16:42)
  745d8b6 Docker Hub (16:35)
  3 commits since 7d

web-app
  a1b2c3d fix login redirect (11:20)
  1 commit since 7d

─── 2 repos · 4 commits · since 7d ───
```

### Markdown

Ready to paste into Slack, Notion, GitHub comments, or email.

```bash
standup --since 7d --format markdown
```

```markdown
## backend-forge

- `f2371a6` Local Development _2025-03-24 16:45_
- `6d6b4a7` Readme _2025-03-24 16:42_

_3 commits since 7d_

## web-app

- `a1b2c3d` fix login redirect _2025-03-24 11:20_

_1 commit since 7d_
```

### JSON

Machine-readable output for piping into other tools.

```bash
standup --since 7d --format json
```

```json
{
  "repo": "backend-forge",
  "since": "7d",
  "commit_count": 3,
  "commits": [
    {
      "hash": "f2371a6",
      "message": "Local Development",
      "author": "Abhil",
      "timestamp": "2025-03-24T16:45:00"
    }
  ]
}
```

---

## AI Summarization

The `--summarize` flag collects all commits across repos, sends them to an LLM, and prints a natural language summary you can paste directly into your standup message.

```bash
standup --since yesterday --summarize
standup --since 7d --format markdown --summarize
```

```
── AI Summary ──────────────────────────
I spent the week containerizing the backend service with Docker and publishing
to Docker Hub. I also set up a production API gateway with a full CI/CD
pipeline, and improved the local development environment.
────────────────────────────────────────
```

### Supported Providers

Any OpenAI-compatible API works. Recommended options:

| Provider | Cost | Setup |
|----------|------|-------|
| [Groq](https://console.groq.com) | Free tier | Sign up, create key, `standup set-key gsk_...` |
| [OpenAI](https://platform.openai.com) | Paid credits | Add billing, create key, `standup set-key sk-...` |
| [Ollama](https://ollama.com) | Free, local | Install Ollama, `ollama pull llama3.2`, point URL to `localhost:11434` |

---

## Config File

Located at:

- **Windows**: `C:\Users\<you>\AppData\Roaming\standup\config.toml`
- **macOS**: `~/.config/standup/config.toml`
- **Linux**: `~/.config/standup/config.toml`

You can edit it directly at any time:

```toml
[settings]
author = "Abhil"
default_since = "yesterday"
groqai_api_key = "gsk_..."

[[repos]]
name = "backend-forge"
path = 'C:\Projects\backend-forge'

[[repos]]
name = "web-app"
path = 'C:\Projects\frontend'
```

---

## Time Range Reference

| Value | Meaning |
|-------|---------|
| `today` | From midnight today |
| `yesterday` | From midnight yesterday |
| `1d` | Last 1 day |
| `7d` | Last 7 days |
| `30d` | Last 30 days |
| `2025-03-01` | From a specific date |

---

## Examples

```bash
# Morning standup — what did I do yesterday?
standup --since yesterday

# Weekly review — last 7 days with AI summary
standup --since 7d --summarize

# Share with team — markdown format
standup --since 7d --format markdown

# Just one repo
standup --repo C:\Projects\api --since 3d

# Filter to your commits only
standup --since 7d --author "your name"

# Export for another tool
standup --since 30d --format json > commits.json

# Full standup message, ready to paste
standup --since yesterday --format markdown --summarize
```

---

## Project Structure

```
src/
├── main.rs        — entry point, report runner
├── cli.rs         — argument definitions (clap)
├── config.rs      — config file read/write
├── commands.rs    — subcommand handlers (init, add, list, remove, set-key)
├── git.rs         — git log reading (git2)
├── output.rs      — plain / markdown / JSON formatters
├── llm.rs         — LLM API call and prompt builder
└── error.rs       — unified error type
```

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | Argument parsing and subcommands |
| `git2` | Read git history without shelling out |
| `chrono` | Date/time parsing and formatting |
| `serde` + `toml` | Config file serialization |
| `serde_json` | JSON output format |
| `dirs` | Cross-platform config directory lookup |
| `colored` | Terminal color output |
| `reqwest` | HTTP client for LLM API calls |

---

