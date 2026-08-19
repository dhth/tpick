use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    println!("Press keys. Press q to quit.\r");

    loop {
        if let Event::Key(key) = event::read()? {
            println!("received: {:?}\r", key);

            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
