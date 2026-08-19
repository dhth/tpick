use super::message::Message;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::Adjust(1)),
        KeyCode::Char('k') => Some(Message::Adjust(-1)),
        KeyCode::Char('J') => Some(Message::Adjust(5)),
        KeyCode::Char('K') => Some(Message::Adjust(-5)),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Enter => Some(Message::Submit),
        _ => None,
    }
}
