use super::model::Model;
use super::{input, update, view};
use chrono::{DateTime, Utc};
use crossterm::event::{self, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Write;

pub fn pick() -> anyhow::Result<Option<DateTime<Utc>>> {
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    let result = run_loop(&mut stdout);
    let clear_result = view::clear(&mut stdout);
    let disable_raw_mode_result = disable_raw_mode();

    let selection = result?;
    clear_result?;
    disable_raw_mode_result?;

    Ok(selection)
}

fn run_loop(output: &mut impl Write) -> anyhow::Result<Option<DateTime<Utc>>> {
    let mut model = Model {
        value: Utc::now(),
        running: true,
        selection: None,
    };

    view::render(output, &model, false)?;

    while model.running {
        if let Event::Key(key) = event::read()?
            && let Some(message) = input::handle_key(key)
        {
            update::update(&mut model, message);
            if model.running {
                view::render(output, &model, true)?;
            }
        }
    }

    Ok(model.selection)
}
