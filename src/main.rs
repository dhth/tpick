mod ui;

fn main() -> anyhow::Result<()> {
    if let Some(value) = ui::pick()? {
        println!("{value}");
    }

    Ok(())
}
