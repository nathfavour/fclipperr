# fclipperr: Universal Clipboard Utility

A cross-platform Rust command-line utility that reliably copies file contents to the system clipboard, working seamlessly in both local and remote (SSH) environments.

## Features

- **Simple Interface**: Just `fclipperr <filename>`
- **Environment Detection**: Automatically detects local vs SSH sessions
- **Local Clipboard Support**: Native clipboard integration for Windows, Linux, and macOS
- **Remote SSH Support**: Uses OSC 52 escape sequences for clipboard access over SSH
- **Tmux/Screen Compatible**: Works inside terminal multiplexers
- **Single Static Binary**: Statically compiled for easy deployment

## Installation

### From Source

```bash
cargo build --release
```

The binary will be available at `target/release/fclipperr`.

### Install via Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

Copy a file's contents to clipboard:

```bash
fclipperr my_output.txt
```

### Help

Display help information:

```bash
fclipperr --help
```

## How It Works

### Local Environment

When running locally, `fclipperr` uses native OS clipboard APIs via the `arboard` crate to copy content directly to your system clipboard.

### Remote/SSH Environment

When running over SSH, `fclipperr` detects the remote environment by checking for SSH-related environment variables (`SSH_TTY`, `SSH_CONNECTION`, `SSH_CLIENT`). It then uses the OSC 52 terminal escape sequence to transmit the content to your local machine's clipboard:

```
\x1b]52;c;{base64_encoded_content}\x07
```

This works with modern terminal emulators that support OSC 52, including:
- iTerm2 (macOS)
- Terminal.app (macOS, with configuration)
- Windows Terminal
- Alacritty
- kitty
- tmux (with `set-clipboard on`)

## Exit Codes

- `0`: Success - content copied to clipboard
- `1`: Failure - file not found, permission denied, or clipboard error

## Requirements

### Terminal Emulator Support (for SSH)

For remote clipboard functionality, your terminal emulator must support OSC 52 escape sequences. Most modern terminals do, but you may need to enable it in your terminal's settings.

### Tmux Configuration

If using tmux, ensure clipboard support is enabled:

```bash
set -g set-clipboard on
```

## Project Structure

```
fclipperr/
├── Cargo.toml          # Package definition and dependencies
├── src/
│   ├── main.rs         # Entry point, argument parsing, environment check
│   └── copy_handler.rs # Core logic module (local and remote copy)
└── README.md
```

## Dependencies

- `arboard` - Cross-platform clipboard library
- `base64` - Base64 encoding for OSC 52 sequences

## License

MIT OR Apache-2.0
