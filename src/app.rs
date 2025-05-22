use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use tokio::sync::mpsc;

use crate::comm::{TelemetryEvent, TelemetrySource};

pub struct App {
    receiver: mpsc::Receiver<TelemetryEvent>,
    logs: Vec<String>,
    latest_state: Option<(f32, f32, f32)>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);

        let mut source = crate::comm::mock::MockSource::new();
        tokio::spawn(async move {
            let _ = source.start_stream(tx).await;
        });

        App {
            receiver: rx,
            logs: Vec::new(),
            latest_state: None,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(0),
                    ])
                    .split(f.area());

                let state_text = if let Some((x, y, theta)) = self.latest_state {
                    format!("Pose: x={:.2}, y={:.2}, θ={:.2}°", x, y, theta)
                } else {
                    "Waiting for data...".to_string()
                };

                let state_panel = Paragraph::new(state_text)
                    .block(Block::default().title("Robot State").borders(Borders::ALL));
                f.render_widget(state_panel, chunks[0]);

                let log_items: Vec<ListItem> = self.logs.iter()
                    .rev()
                    .take(10)
                    .map(|log| ListItem::new(Span::raw(log)))
                    .collect();

                let log_list = List::new(log_items)
                    .block(Block::default().title("Logs").borders(Borders::ALL));
                f.render_widget(log_list, chunks[1]);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }

            while let Ok(event) = self.receiver.try_recv() {
                match event {
                    TelemetryEvent::State { x, y, theta, .. } => {
                        self.latest_state = Some((x, y, theta));
                    }
                    TelemetryEvent::Log { level, message } => {
                        self.logs.push(format!("[{}] {}", level, message));
                        if self.logs.len() > 1000 {
                            self.logs.drain(0..self.logs.len() - 1000);
                        }
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}