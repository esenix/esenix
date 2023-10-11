use std::{
    io::{stdout, Stdout},
    time::Duration,
    sync::mpsc::{self, Receiver}
};

use crossterm::{
    cursor::SetCursorStyle,
    terminal::{EnterAlternateScreen, enable_raw_mode}, event::{KeyCode, KeyEvent}
};

use ratatui::{
    prelude::{CrosstermBackend, Rect, Layout, Direction, Constraint},
    widgets::{Paragraph, Block}, style::{Style, Color}
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type Backend = CrosstermBackend<Stdout>;
type Terminal = ratatui::Terminal<Backend>;
type Frame<'a> = ratatui::Frame<'a, Backend>;

struct EsenixContext {
    mode: EsenixMode,
    active_window_idx: Option<usize>,
    windows: Vec<Window>,
    buffers: Vec<Buffer>,
}

/**
 * this is used to render the status bar
 */
impl TuiRenderable for EsenixContext {
    fn render(&self, _: Option<&EsenixContext>, frame: &mut Frame, area: Rect) {
        let vert_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        {
            let top_bar_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]).split(vert_layout[0]);

            let active_window_hint_str = self.active_window_idx
                .map(|idx| format!("Window: {idx}"))
                .unwrap_or_else(|| String::from("No window"));

            let active_window_hint_paragraph = Paragraph::new(active_window_hint_str);


            let active_window_buffer_hint_str = self.active_window_idx
                .and_then(|idx| self.windows.get(idx)) // TODO: unwrap
                .and_then(|window| window.buffer_idx)
                .and_then(|buf_idx| self.buffers.get(buf_idx).zip(Some(buf_idx)))
                .map(|(buf, idx)| format!("Attached Buffer: <{idx}::{}>", buf.name))
                .unwrap_or_else(|| String::from("No buffer attached"));

            let active_window_buffer_hint_paragraph = Paragraph::new(active_window_buffer_hint_str);

            let open_buffers_hint_paragraph = Paragraph::new(format!("Open Buffers: {}", self.buffers.len()));

            // background
            frame.render_widget(
                Block::default()
                    .style(
                        Style::default()
                            .bg(Color::DarkGray)
                    ),
                vert_layout[0]
            );

            frame.render_widget(active_window_hint_paragraph, top_bar_layout[0]);
            frame.render_widget(active_window_buffer_hint_paragraph, top_bar_layout[1]);
            frame.render_widget(open_buffers_hint_paragraph, top_bar_layout[2]);
        }

        {
            let bottom_bar_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]).split(vert_layout[1]);

            let esenix_mode_hint_paragraph = Paragraph::new(format!("-- {} --", self.mode.as_str()));

            frame.render_widget(esenix_mode_hint_paragraph, bottom_bar_layout[0]);
        }
    }
}

impl Default for EsenixContext {
    fn default() -> Self {
        Self {
            mode: EsenixMode::Normal,
            windows: Vec::new(),
            buffers: Vec::new(),
            active_window_idx: None
        }
    }
}

enum EsenixMode {
    Normal,
    Insert,
}

impl EsenixMode {
    fn as_str(&self) -> &'static str {
        match self {
            EsenixMode::Normal => "Normal",
            EsenixMode::Insert => "Insert"
        }
    }
}

struct Buffer {
    name: String,
    content: String,
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
    fn render(&self, context: Option<&EsenixContext>, frame: &mut Frame, area: Rect);
}

impl TuiRenderable for Window {
    fn render(&self, context: Option<&EsenixContext>, frame: &mut Frame, area: Rect) {
        let context = context.unwrap();

        match self.buffer_idx {
            Some(idx) => {
                let buffer = context.buffers.get(idx).unwrap();
                let p = Paragraph::new(format!("{}", buffer.name));
                frame.render_widget(p, area);
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
        Buffer {
            name: String::from("example"),
            content: String::from("hallo, welt"),
        }
    );

    context.windows.push(Window::default());
    context.active_window_idx = Some(0);

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
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(98),
                    Constraint::Min(2)
                ])
                .split(frame.size());

            context.windows.iter()
                .for_each(|window| window.render(Some(&context), frame, main_layout[0]));

            context.render(None, frame, main_layout[1]);
        })?;
    }

    Ok(())
}
