mod game;
mod input;
mod render;
mod watcher;

use clap::Parser;
use crossterm::{
    cursor,
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::Deserialize;
use std::io::{self, Read};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use game::GameState;
use input::{poll_input, GameAction};
use render::GameScene;
use watcher::TranscriptWatcher;

/// Claude Compact Runner - A game to play while Claude Code compacts
#[derive(Parser, Debug)]
#[command(name = "claude-compact-runner")]
#[command(about = "A terminal side-scroller game for Claude Code compaction")]
#[command(version)]
struct Args {
    /// Path to transcript file to watch
    #[arg(short, long)]
    transcript: Option<PathBuf>,

    /// Max duration before auto-exit (seconds)
    #[arg(short, long, default_value = "300")]
    duration: u64,

    /// Initial speed multiplier
    #[arg(short, long, default_value = "1.0")]
    speed: f32,

    /// Disable colors
    #[arg(long)]
    no_color: bool,

    /// Run in demo mode (no file watching, manual exit only)
    #[arg(long)]
    demo: bool,
}

/// Hook input JSON from Claude Code
#[derive(Deserialize, Debug)]
struct HookInput {
    #[serde(default)]
    transcript_path: Option<String>,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    hook_event_name: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Determine transcript path
    let transcript_path = if args.demo {
        None
    } else if let Some(path) = args.transcript {
        Some(path)
    } else {
        // Try to read from stdin (hook input)
        read_transcript_from_stdin()
    };

    // Set up terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get terminal size
    let size = terminal.size()?;

    // Initialize game state
    let mut game = GameState::new().with_terminal_width(size.width);
    game.speed = args.speed;

    // Set up file watcher
    let watcher: Option<TranscriptWatcher> = transcript_path
        .as_ref()
        .and_then(|p| TranscriptWatcher::new(p.clone()).ok());

    let start_time = Instant::now();
    let max_duration = Duration::from_secs(args.duration);
    let frame_duration = Duration::from_millis(33); // ~30fps

    // Main game loop
    loop {
        let frame_start = Instant::now();

        // Check exit conditions
        if game.should_quit {
            break;
        }

        // Check timeout
        if start_time.elapsed() >= max_duration {
            break;
        }

        // Check file watcher
        if let Some(ref w) = watcher {
            if w.file_changed().unwrap_or(false) {
                break;
            }
        }

        // Handle input (non-blocking with short timeout)
        let input_timeout = Duration::from_millis(5);
        if let Some(action) = poll_input(input_timeout)? {
            match action {
                GameAction::Jump => game.jump(),
                GameAction::Quit => game.should_quit = true,
                GameAction::None => {}
            }
        }

        // Update game state
        game.tick();
        game.maybe_spawn_obstacle();

        // Check terminal size and update
        let new_size = terminal.size()?;
        if new_size.width < 40 || new_size.height < 6 {
            // Terminal too small, exit gracefully
            break;
        }
        game.terminal_width = new_size.width;

        // Render
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(GameScene::new(&game), area);
        })?;

        // Frame rate limiting
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        cursor::Show
    )?;

    Ok(())
}

fn read_transcript_from_stdin() -> Option<PathBuf> {
    // Check if stdin has data (non-blocking check)
    // For now, just try to read and parse JSON
    let mut input = String::new();

    // Set a short timeout for reading stdin
    // If nothing is available, return None
    match io::stdin().read_to_string(&mut input) {
        Ok(_) if !input.is_empty() => {
            if let Ok(hook_input) = serde_json::from_str::<HookInput>(&input) {
                hook_input.transcript_path.map(PathBuf::from)
            } else {
                None
            }
        }
        _ => None,
    }
}
