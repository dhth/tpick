use super::message::Message;
use super::model::Model;

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Adjust(amount) => model.value = model.value.wrapping_add_signed(amount),
        Message::Submit => {
            model.selection = Some(model.value);
            model.running = false;
        }
        Message::Quit => model.running = false,
    }
}
