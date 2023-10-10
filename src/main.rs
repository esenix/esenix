use std::{io::stdout, time::Duration, sync::mpsc};

use crossterm::{
    cursor::SetCursorStyle,
    terminal::{EnterAlternateScreen, enable_raw_mode}, event::KeyCode
};

use ratatui::{
    prelude::CrosstermBackend,
    Terminal,
    widgets::Block
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // TODO: CursorStyle
    crossterm::execute!(stdout(), SetCursorStyle::SteadyBar, EnterAlternateScreen)?;

    enable_raw_mode()?;
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        use crossterm::event::{self, Event};

        loop {
            if let Ok(true) = event::poll(Duration::from_millis(40)) {
                // TODO: unwrap
                if let Event::Key(key_event) = event::read().unwrap() {
                    tx.send(key_event).unwrap();
                }
            }
        }
    });

    loop {
        if let Ok(key_event) = rx.try_recv() {
            match key_event.code {
                KeyCode::Esc => break,
                _ => {}
            }
        }

        terminal.draw(|frame| {
            let block = Block::default()
                .title("hallo");

            frame.render_widget(block, frame.size());
        })?;
    }

    Ok(())
}
