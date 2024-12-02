mod application;
mod infra;
mod models;

use application::app::App;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let _ = App::new()?.run();
    execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");

    ratatui::restore();
    Ok(())
}
