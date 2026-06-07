use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::io;
use std::time::Duration;

/// returns the current terminal size
pub fn get_terminal_size() -> (u16, u16) {
    terminal::size().unwrap_or((80, 24))
}

/// Enters Raw Mode: enables raw input, hides cursor, clears screen.
pub fn enter_raw_mode() {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )
    .expect("Failed to initialize terminal");
}

// shows cursor, clears screen, and exits raw mode
pub fn leave_raw_mode() {
    let mut stdout = io::stdout();
    let _ = execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    );
    let _ = terminal::disable_raw_mode();
}

// polls for a quit event (q key or Ctrl+C) within the specified timeout
pub fn poll_for_quit(timeout: Duration) -> bool {
    if event::poll(timeout).unwrap_or(false) {
        if let Ok(Event::Key(key_event)) = event::read() {
            return matches!(
                key_event,
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } | KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }
            );
        }
    }
    false
}

// waits until a quit event is detected
pub fn wait_for_quit() {
    loop {
        if poll_for_quit(Duration::from_millis(100)) {
            break;
        }
    }
}
