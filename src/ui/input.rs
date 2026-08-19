use super::message::Message;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::Adjust(1)),
        KeyCode::Char('k') => Some(Message::Adjust(-1)),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Enter => Some(Message::Submit),
        _ => None,
    }
}
