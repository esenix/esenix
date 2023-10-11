use std::{io::{stdout, Stdout}, time::Duration, sync::mpsc::{self, Receiver}};

use crossterm::{
    cursor::SetCursorStyle,
    terminal::{EnterAlternateScreen, enable_raw_mode}, event::{KeyCode, KeyEvent}
};

use ratatui::{
    prelude::CrosstermBackend,
    widgets::Block
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Backend = CrosstermBackend<Stdout>;
type Terminal = ratatui::Terminal<Backend>;
struct EsenixContext {
    mode: EsenixMode,
}
}

enum EsenixMode {
    Normal,
    Insert,
}

fn start_crossterm_event_polling_thread() -> Receiver<KeyEvent> {
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

    return rx;
}

fn main() -> Result<()> {
    let backend = Backend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // TODO: CursorStyle
    crossterm::execute!(stdout(), SetCursorStyle::SteadyBar, EnterAlternateScreen)?;

    enable_raw_mode()?;
    terminal.clear()?;

    let key_event_receiver = start_crossterm_event_polling_thread();

    loop {
        if let Ok(key_event) = key_event_receiver.try_recv() {
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
