# Claude Compact Runner

A terminal side-scroller game that plays during Claude Code's compaction process, inspired by Chrome's offline dino game but featuring the Claude Code mascot.

## Overview

When Claude Code compacts conversation context, instead of showing a static "compacting..." message, users see an inline terminal game where the Claude mascot runs and jumps over obstacles. The game ends automatically when compaction completes, or users can exit early with `q`/`Esc`.

## Goals

- **Delightful** - Transform a wait moment into a tiny joy
- **Inline** - Feels like part of the terminal, not a popup
- **Non-blocking** - Easy to dismiss, never gets in the way
- **On-brand** - Uses the Claude Code mascot character

## Technical Architecture

### Stack

- **Language**: Rust
- **Terminal UI**: `ratatui` + `crossterm`
- **File watching**: `notify` crate (or simple mtime polling)
- **Distribution**: Single binary, installed to `~/.local/bin/` or `~/.claude/bin/`

### File Structure

```
claude-compact-runner/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # Entry point, CLI args, orchestration
│   ├── game/
│   │   ├── mod.rs           # Game module exports
│   │   ├── state.rs         # Game state (player, obstacles, score)
│   │   ├── physics.rs       # Jump mechanics, collision detection
│   │   └── spawner.rs       # Obstacle spawning logic
│   ├── render/
│   │   ├── mod.rs           # Render module exports
│   │   ├── sprites.rs       # Claude mascot, obstacles as block art
│   │   ├── scene.rs         # Compose full game scene
│   │   └── ground.rs        # Scrolling ground line
│   ├── input/
│   │   ├── mod.rs           # Input handling
│   │   └── events.rs        # Keyboard event processing
│   └── watcher/
│       ├── mod.rs           # File watcher module
│       └── transcript.rs    # Watch transcript file for compaction end
├── assets/
│   └── sprites.txt          # ASCII/block art reference designs
└── install/
    ├── install.sh           # Installation script
    └── hook-config.json     # Example Claude Code hook configuration
```

### Claude Code Hook Integration

**Hook Configuration** (`.claude/settings.json`):

```json
{
  "hooks": {
    "PreCompact": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "claude-compact-runner --transcript \"$TRANSCRIPT_PATH\"",
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

**Hook Input** (received via stdin as JSON):

```json
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../session.jsonl",
  "hook_event_name": "PreCompact",
  "trigger": "auto"
}
```

### Exit Conditions

The game exits when ANY of these occur:

1. **Transcript file changes** - New content written after compaction completes
2. **User presses `q` or `Esc`** - Immediate graceful exit
3. **Timeout** - Safety net, exit after 5 minutes (configurable)
4. **Terminal resize to tiny** - Graceful exit if terminal too small to render

## Game Design

### Visual Layout

The game renders in a fixed-height region (6-8 terminal rows):

```
                                                           
       ▄█▀█▄                                               
       █▀█▀█          ▄█▄                 ▄█▄      ▄█▄     
       ▀███▀          ███      ▄█▄        ███▄     ███     
        █ █           ███      ███        ████     ███     
  ══════════════════════════════════════════════════════   
  compacting...                              score: 142    
                                                           
```

### Claude Mascot Sprite

Based on the reference image - a chunky, friendly character with:
- Peachy/salmon colored body (we'll use block characters, color via ANSI)
- Simple rectangular head with two dark square eyes
- Small rectangular "ears" or antennae on top
- Two small legs

**Sprite frames using Unicode block characters:**

```
Standing:
 ▄█▀█▄
 █▀█▀█
 ▀███▀
  █ █

Jumping (legs tucked):
 ▄█▀█▄
 █▀█▀█
 ▀███▀
  ▀▀▀

Running frame 1:
 ▄█▀█▄
 █▀█▀█
 ▀███▀
 █   █

Running frame 2:
 ▄█▀█▄
 █▀█▀█
 ▀███▀
   █ █
```

**Color**: Use ANSI 256-color or truecolor for the salmon/peach tone (`#D97757` or similar from the Claude palette)

### Obstacles

Simple geometric obstacles that fit the aesthetic:

**Small cactus/spike:**
```
 ▄█▄
 ███
 ███
```

**Tall cactus/spike:**
```
 ▄█▄
 ███
 ███▄
 ████
 ███
```

**Double obstacle:**
```
 ▄█▄   ▄█▄
 ███   ███
 ███   ███
```

**Color**: Use a contrasting color (darker, maybe the Claude dark gray `#2D2D2A`)

### Ground

Scrolling double-line ground using box-drawing characters:

```
══════════════════════════════════════════════════════
```

With occasional texture variation:
```
═══════╦══════════╦═══════════════╦══════════════════
```

### Game Mechanics

1. **Auto-run**: Claude mascot automatically moves right (world scrolls left)
2. **Jump**: `Space` triggers jump with simple parabolic arc
3. **Gravity**: ~0.5 units per tick, jump velocity ~-4 units
4. **Collision**: Simple bounding box, forgiving hitboxes (slightly smaller than visual)
5. **Scoring**: +1 per frame survived, or +10 per obstacle cleared
6. **Speed**: Gradually increases over time (cap at reasonable max)
7. **No death state**: On collision, brief flash but continue (this is for fun, not frustration)

### Controls

| Key | Action |
|-----|--------|
| `Space` | Jump |
| `q` | Quit immediately |
| `Esc` | Quit immediately |
| `↑` | Jump (alternative) |

### Status Bar

Bottom of game area shows:
- Left: "compacting..." (animated dots: . → .. → ... → .)
- Right: "score: XXX"
- Optional: "[q] quit" hint, fades after first few seconds

## Implementation Details

### Main Loop

```rust
// Pseudocode
fn main() {
    let args = parse_cli_args();
    let transcript_path = args.transcript.or_else(|| read_from_stdin());
    
    let mut terminal = setup_terminal()?;
    let mut game = GameState::new();
    let watcher = TranscriptWatcher::new(transcript_path)?;
    
    loop {
        // Check exit conditions
        if watcher.file_changed()? || game.should_quit {
            break;
        }
        
        // Handle input (non-blocking)
        if let Some(key) = poll_input()? {
            game.handle_input(key);
        }
        
        // Update game state
        game.tick();
        
        // Render
        terminal.draw(|f| render_game(f, &game))?;
        
        // Frame rate limiting (~30fps)
        sleep(Duration::from_millis(33));
    }
    
    restore_terminal(terminal)?;
    // Exit code 0 - success, don't block Claude
}
```

### File Watcher Strategy

Simple approach - just watch mtime:

```rust
struct TranscriptWatcher {
    path: PathBuf,
    initial_size: u64,
    initial_mtime: SystemTime,
}

impl TranscriptWatcher {
    fn file_changed(&self) -> Result<bool> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len() > self.initial_size || 
           metadata.modified()? > self.initial_mtime)
    }
}
```

### Terminal Region

Rather than clearing the whole screen, the game should:
1. Note current cursor position
2. Render in a fixed region below
3. On exit, clear just that region and restore cursor

This keeps it feeling "inline" rather than full-screen.

### Color Palette

From Claude/Anthropic brand (approximate):
- Mascot body: `#D97757` (salmon/peach)
- Mascot eyes: `#2D2D2A` (dark gray, nearly black)
- Obstacles: `#2D2D2A` or `#5A5A52`
- Ground: `#8A8A7A` (medium gray)
- Background: terminal default (transparent)
- Score text: terminal default foreground

## Installation & Distribution

### One-liner Install

```bash
curl -sSL https://raw.githubusercontent.com/YOU/claude-compact-runner/main/install.sh | bash
```

### Install Script Actions

1. Detect OS/arch
2. Download pre-built binary from releases (or cargo install from source)
3. Place in `~/.local/bin/claude-compact-runner`
4. Ensure `~/.local/bin` is in PATH (warn if not)
5. Offer to add hook configuration to `~/.claude/settings.json` (ask first, don't clobber)

### Manual Install

```bash
# From source
cargo install claude-compact-runner

# Or download binary
wget https://github.com/YOU/claude-compact-runner/releases/latest/download/claude-compact-runner-$(uname -s)-$(uname -m) -O ~/.local/bin/claude-compact-runner
chmod +x ~/.local/bin/claude-compact-runner
```

## CLI Interface

```
claude-compact-runner [OPTIONS]

OPTIONS:
    -t, --transcript <PATH>    Path to transcript file to watch
    -d, --duration <SECS>      Max duration before auto-exit [default: 300]
    -s, --speed <MULT>         Initial speed multiplier [default: 1.0]
    --no-color                 Disable colors
    --demo                     Run in demo mode (no file watching, manual exit only)
    -h, --help                 Print help
    -V, --version              Print version

If --transcript is not provided, reads PreCompact hook JSON from stdin.
```

## Testing Plan

1. **Unit tests**: Game physics, collision detection, spawner logic
2. **Integration test**: Full game loop with mock file watcher
3. **Manual testing**: 
   - Run `claude-compact-runner --demo` standalone
   - Test with actual Claude Code compaction
4. **Edge cases**:
   - Tiny terminal (< 40 cols or < 8 rows)
   - Very fast exit (file changes immediately)
   - Very long run (5+ minutes)
   - Rapid key presses

## Future Enhancements (v2+)

- [ ] High score persistence (`~/.claude/runner-highscore`)
- [ ] Sound effects (optional, terminal bell or background audio)
- [ ] Multiple mascot skins
- [ ] Day/night cycle based on system time
- [ ] Flying obstacles (require duck mechanic)
- [ ] Multiplayer high scores across Claude users

## Open Questions

1. **Exact transcript change detection**: Need to verify what Claude Code writes to transcript after compaction - is it immediately obvious, or do we need to parse for specific content?

2. **Hook timeout behavior**: If the hook times out, does Claude Code kill the process? Need to handle SIGTERM gracefully.

3. **Windows support**: ratatui/crossterm should work, but needs testing. File watching differs on Windows.

---

## Quick Start for Implementation

```bash
# Create project
cargo new claude-compact-runner
cd claude-compact-runner

# Add dependencies to Cargo.toml
cargo add ratatui crossterm notify clap serde serde_json

# Create directory structure
mkdir -p src/{game,render,input,watcher}

# Start with demo mode working, then add file watching
```

Good luck, Claude Code! 🦖
