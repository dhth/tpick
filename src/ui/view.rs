use super::model::Model;
use crate::format::TIMESTAMP_FORMAT;
use chrono::Local;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::MoveUp;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;

pub fn render(output: &mut impl Write, model: &Model, redraw: bool) -> std::io::Result<()> {
    if redraw {
        queue!(output, MoveToColumn(0), MoveUp(1))?;
    }

    queue!(
        output,
        Print("j/k ±1 min, J/K ±5 min, enter to select, q to quit\r\n"),
        Print(model.value.with_timezone(&Local).format(TIMESTAMP_FORMAT),),
    )?;
    output.flush()
}

pub fn clear(output: &mut impl Write) -> std::io::Result<()> {
    queue!(
        output,
        MoveToColumn(0),
        MoveUp(1),
        Clear(ClearType::FromCursorDown),
    )?;
    output.flush()
}
