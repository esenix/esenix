use std::{io::{stdout, Stdout}, time::Duration, sync::mpsc::{self, Receiver}, fmt::Display};

use crossterm::{
    cursor::SetCursorStyle,
    terminal::{EnterAlternateScreen, enable_raw_mode}, event::{KeyCode, KeyEvent}
};

use ratatui::{
    prelude::{CrosstermBackend, Rect},
    widgets::{Block, Paragraph}
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Backend = CrosstermBackend<Stdout>;
type Terminal = ratatui::Terminal<Backend>;
type Frame<'a> = ratatui::Frame<'a, Backend>;

struct EsenixContext {
    mode: EsenixMode,
    windows: Vec<Window>,
    buffers: Vec<Buffer>,
}

impl Default for EsenixContext {
    fn default() -> Self {
        Self {
            mode: EsenixMode::Normal,
            windows: Vec::new(),
            buffers: Vec::new(),
        }
    }
}

enum EsenixMode {
    Normal,
    Insert,
}

enum Buffer {
    Text {
        name: String,
        content: String,
    },
    Canvas
}

struct Window {
    buffer_idx: Option<usize>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            buffer_idx: None
        }
    }
}

trait TuiRenderable {
    fn render(&self, context: &EsenixContext, frame: &mut Frame, area: Rect);
}

impl TuiRenderable for Window {
    fn render(&self, context: &EsenixContext, frame: &mut Frame, area: Rect) {
        match self.buffer_idx {
            Some(idx) => {
                let buffer = context.buffers.get(idx).unwrap();
                if let Buffer::Text { name, content } = buffer {
                    let p = Paragraph::new(format!("{name} --> {content}"));
                    frame.render_widget(p, area);
                }
            }
            None => {
                context.buffers.len();
                let p = Paragraph::new(format!("Open Buffers: {}", context.buffers.len()));

                frame.render_widget(p, area);
            }
        }
    }
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
    let mut context = EsenixContext::default();

    //* push example buffer
    context.buffers.push(
        Buffer::Text {
            name: String::from("example"),
            content: String::from("hallo, welt")
        }
    );

    context.windows.push(Window::default());

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
                KeyCode::Tab => {
                    context.windows.first_mut()
                        .unwrap()
                        .buffer_idx = Some(0);
                }
                _ => {}
            }
        }

        terminal.draw(|frame| {
            context.windows.iter()
                .for_each(|window| window.render(&context, frame, frame.size()))
        })?;
    }

    Ok(())
}
