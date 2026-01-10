use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
/// Actions that can result from input
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Jump,
    Duck,
    StopDuck,
    Quit,
    None,
}

/// Poll for keyboard input with a timeout
/// Returns None if no event, Some(action) otherwise
pub fn poll_input(timeout: Duration) -> std::io::Result<Option<GameAction>> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            match key.kind {
                KeyEventKind::Press => return Ok(Some(handle_key_press(key))),
                KeyEventKind::Release => return Ok(Some(handle_key_release(key))),
                _ => {}
            }
        }
    }
    Ok(None)
}

fn handle_key_press(key: KeyEvent) -> GameAction {
    match key.code {
        // Jump keys
        KeyCode::Char(' ') => GameAction::Jump,
        KeyCode::Up => GameAction::Jump,
        KeyCode::Char('w') => GameAction::Jump,
        KeyCode::Char('k') => GameAction::Jump,

        // Duck keys
        KeyCode::Down => GameAction::Duck,
        KeyCode::Char('s') => GameAction::Duck,
        KeyCode::Char('j') => GameAction::Duck,

        // Quit keys
        KeyCode::Char('q') => GameAction::Quit,
        KeyCode::Esc => GameAction::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => GameAction::Quit,

        _ => GameAction::None,
    }
}

fn handle_key_release(key: KeyEvent) -> GameAction {
    match key.code {
        // Stop ducking when duck key released
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => GameAction::StopDuck,
        _ => GameAction::None,
    }
}
