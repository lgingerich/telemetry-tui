use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use tokio::sync::mpsc;

use crate::comm::{TelemetryEvent, TelemetrySource};
use crate::tui;

pub struct App {
    receiver: mpsc::Receiver<TelemetryEvent>,
    pub logs: Vec<String>,
    pub latest_state: Option<(f32, f32, f32, f32, f32, String)>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);

        let mut source = crate::comm::mock::MockSource::new();
        // let mut source = crate::comm::serial::SerialSource::new("/dev/ttyUSB0", 115200);
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
                tui::ui::draw_ui(f, self);
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
                    TelemetryEvent::State { x, y, theta, vx, vy, mode } => {
                        self.latest_state = Some((x, y, theta, vx, vy, mode));
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