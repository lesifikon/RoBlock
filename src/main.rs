use ratatui;

pub mod app;
pub mod ui;

use crate::app::app;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}
