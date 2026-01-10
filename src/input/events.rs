use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
/// Actions that can result from input
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Jump,
    ToggleDuck,
    Quit,
    None,
}

/// Poll for keyboard input with a timeout
/// Returns None if no event, Some(action) otherwise
pub fn poll_input(timeout: Duration) -> std::io::Result<Option<GameAction>> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            // Only respond to key press, not release (release events unreliable cross-platform)
            if key.kind == KeyEventKind::Press {
                return Ok(Some(handle_key(key)));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: KeyEvent) -> GameAction {
    match key.code {
        // Jump keys
        KeyCode::Char(' ') => GameAction::Jump,
        KeyCode::Up => GameAction::Jump,
        KeyCode::Char('w') => GameAction::Jump,
        KeyCode::Char('k') => GameAction::Jump,

        // Duck keys (toggle)
        KeyCode::Down => GameAction::ToggleDuck,
        KeyCode::Char('s') => GameAction::ToggleDuck,
        KeyCode::Char('j') => GameAction::ToggleDuck,

        // Quit keys
        KeyCode::Char('q') => GameAction::Quit,
        KeyCode::Esc => GameAction::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => GameAction::Quit,

        _ => GameAction::None,
    }
}
