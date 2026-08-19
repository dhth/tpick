use super::message::Message;
use super::model::Model;
use chrono::Duration;

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Adjust(minutes) => model.value += Duration::minutes(minutes),
        Message::Submit => {
            model.selection = Some(model.value);
            model.running = false;
        }
        Message::Quit => model.running = false,
    }
}
