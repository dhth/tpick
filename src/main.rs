mod format;
mod ui;

use chrono::Local;

fn main() -> anyhow::Result<()> {
    if let Some(value) = ui::pick()? {
        println!(
            "{}",
            value.with_timezone(&Local).format(format::TIMESTAMP_FORMAT)
        );
    }

    Ok(())
}
