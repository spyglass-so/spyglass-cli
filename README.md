# Spyglass CLI

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-CLI%20generated%20by%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2Fspyglass-so%2Fspyglass-cli)
[![npm shield](https://img.shields.io/npm/v/@spyglasshq/cli)](https://www.npmjs.com/package/@spyglasshq/cli)

Command-line interface for the Spyglass API.

## Table of contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Quick start](#quick-start)
- [Usage](#usage)
- [Documentation](#documentation)
- [Advanced](#advanced)
  - [Common flags](#common-flags)
  - [Environment variables](#environment-variables)
  - [Output formats](#output-formats)
  - [Shell completion](#shell-completion)
- [Attribution](#attribution)

## Installation

### Shell (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spyglass-so/spyglass-cli/releases/latest/download/spyglass-cli-installer.sh | sh
```

### PowerShell (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/spyglass-so/spyglass-cli/releases/latest/download/spyglass-cli-installer.ps1 | iex"
```

### npm

```bash
npm install -g @spyglasshq/cli
```

Or run directly without installing:

```bash
npx @spyglasshq/cli --help
```

### Build from source

If you prefer to build from source, install the [Rust toolchain](https://rustup.rs/) and run:

```bash
cargo build --release
./target/release/spyglass --help
```

## Authentication

Set the following environment variable(s) before using the CLI:

```bash
export SPYGLASS_API_KEY="<your token>"
```

A `.env` file in the working directory is also supported — the CLI auto-loads credentials and output preferences from it on startup. Transport settings (`*_BASE_URL`, `*_PROXY`, `*_NO_PROXY`, `*_INSECURE`, `*_CA_BUNDLE`, `HTTP(S)_PROXY`, `NO_PROXY`, `SSL_CERT_FILE`) are ignored when they come from `.env` and must be exported in the shell.

## Quick start

List available commands:

```bash
spyglass --help
```

Call an API endpoint:

```bash
spyglass <resource> <method>
```

Resources can be nested (e.g. `spyglass <resource> <sub-resource> <method>`). Run `spyglass <resource> --help` to see the sub-resources and methods of a resource.

## Usage

Every API resource appears as a subcommand (e.g. `spyglass <resource> <method>`). Resources can be nested several levels deep (e.g. `spyglass <resource> <sub-resource> <method>`). Run `spyglass <resource> --help` to see available sub-resources and methods.

Provide request parameters as flags, or (for operations with a request body) as JSON:

```bash
spyglass <resource> <method> --json '{"key": "value"}'
```

## Documentation

See [reference.md](./reference.md) for the full command reference.

## Advanced

### Common flags

These flags are accepted anywhere on the command line:

| Flag | Description |
|------|-------------|
| `--format <json\|table\|yaml\|csv\|jsonl\|raw\|http>` | Output format (default `table` on a terminal, `json` when piped) |
| `--human` | Keep the interactive (`table`) rendering when piping |
| `--base-url <URL>` | Override the API base URL |

These flags are available on every operation:

| Flag | Description |
|------|-------------|
| `--dry-run` | Validate the request locally and print the HTTP request without sending it |
| `--params <JSON>` | Merge extra parameters as JSON (overrides individual flags) |
| `--no-extract` | Print the full response body instead of only the part the command returns by default |
| `--retries <N>` | Override the retry count (falls back to `SPYGLASS_RETRIES`, then the active profile) |
| `--no-retry` | Disable automatic retries, including retries on network errors |
| `-q, --quiet` | Suppress stdout output on success (errors still go to stderr) |

Operations with a request body also accept:

| Flag | Description |
|------|-------------|
| `--json <JSON\|->` | Supply the request body as JSON (or `-` to read stdin) |

Operations that return binary data also accept:

| Flag | Description |
|------|-------------|
| `-o, --output <PATH>` | Write the binary response to a file (`-` streams to stdout) |

Paginated commands also accept:

| Flag | Description |
|------|-------------|
| `--page-all` | Auto-paginate and stream results as NDJSON |
| `--page-limit <N>` | Max pages to fetch when auto-paginating (default `10`) |
| `--page-delay <MS>` | Delay between page fetches in milliseconds (default `100`) |
| `--no-pager` | Disable the pager even on interactive terminals |

Streaming commands also accept:

| Flag | Description |
|------|-------------|
| `--no-stream` | Buffer the streaming response and print it as a single value once complete |

Run `spyglass <resource> <method> --help` to see exactly which flags an operation accepts.

### Environment variables

| Variable | Description |
|----------|-------------|
| `SPYGLASS_BASE_URL` | Override the API base URL |
| `SPYGLASS_OUTPUT` | Default output format (same values as `--format`) |
| `SPYGLASS_RETRIES` | Default retry count (same as `--retries`) |
| `SPYGLASS_PROFILE` | Name of the stored credential profile to use |
| `SPYGLASS_CA_BUNDLE` | Path to PEM file with extra trust roots (or `SSL_CERT_FILE`) |
| `SPYGLASS_INSECURE=1` | Skip TLS verification (debugging only) |
| `SPYGLASS_PROXY` | HTTP(S) proxy URL |
| `SPYGLASS_NO_PROXY` | Comma-separated hosts that bypass `SPYGLASS_PROXY` |
| `SPYGLASS_TIMEOUT_SECS` | Total request timeout in seconds (default: none) |
| `SPYGLASS_CONNECT_TIMEOUT_SECS` | Connection-establishment timeout in seconds |
| `SPYGLASS_USER_AGENT_SUFFIX` | Product token appended to the `User-Agent` header |

Standard environment variables (`HTTPS_PROXY` / `HTTP_PROXY` / `NO_PROXY` / `SSL_CERT_FILE`) are also honored.

### Output formats

Use the global `--format` flag to control output. Supported values: `json`, `table`, `yaml`, `csv`, `jsonl`, `raw`, `http`.

Without `--format`, output (including errors) is `table` when stdout is a terminal and `json` when it is piped or redirected — so scripts and agents get JSON by default. Pass `--human` to keep the interactive rendering when piping to a pager, and `--format json` to pin JSON in a terminal.

```bash
# Pipe JSON output through jq
spyglass <resource> <method> --format json | jq

# Keep the human rendering even when piped
spyglass <resource> <method> --human | less

# Machine-readable catalog of every operation (same as --schema)
spyglass --help --format json | jq '.operations | length'
```

### Shell completion

Generate shell completion scripts:

```bash
spyglass completion <bash|zsh|fish|powershell>
```

## Attribution

Built on [fern-cli-sdk](https://github.com/fern-api/fern), Copyright Fern, licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).

