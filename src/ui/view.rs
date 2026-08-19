use super::model::Model;
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
        Print("j/k to adjust, enter to select, q to quit\r\n"),
        Print(format!("{:<3}", model.value)),
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
