# Clawd Runner

A Chrome dino-style endless runner for Claude Code's compaction wait screen. Built in Rust with ratatui.

```
▗█▀█▀█▖
 █▅█▅█
  ▀ ▀
```

## Install

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/ScavieFae/clawd-runner.git
cd clawd-runner
cargo build --release
```

Binary lands at `target/release/clawd-runner`.

## Usage

### Manual

```bash
./target/release/clawd-runner --demo
```

### As a Claude Code hook

Add to `~/.claude/settings.json`:

```json
{
  "hooks": {
    "PreCompact": [
      {
        "matcher": "auto",
        "hooks": [
          {
            "type": "command",
            "command": "osascript -e 'tell app \"Terminal\" to do script \"$HOME/claude-projects/clawd-runner/target/release/clawd-runner --demo\"'"
          }
        ]
      }
    ]
  }
}
```

Adjust the path if your clone lives somewhere other than `~/claude-projects/clawd-runner`.

## Controls

- **Space / Up / W / K**: Jump
- **Down / S / J**: Duck (to avoid flying obstacles)
- **Q / Escape**: Quit

## How it works

When used as a PreCompact hook, the game launches in a new Terminal window so you have something to do while Claude compacts context. It watches for compaction to complete and exits automatically, or you can quit manually.

See [DESIGN.md](DESIGN.md) for the full design document.
