use std::io::stdout;

use ratatui::{prelude::CrosstermBackend, Terminal};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    Ok(())
}
