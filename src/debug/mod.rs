mod fps;
mod logs;

pub use fps::*;
use tracing_subscriber::EnvFilter;

use std::{
    borrow::Cow,
    io::{self, Stdout},
    time::Duration,
};

use ansi_to_tui::ansi_to_text;
use bevy::{prelude::*, utils::Instant};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders, List, ListItem},
    Terminal,
};

use fps::FrameCounter;
use logs::SharedLogs;

pub struct DebugTerminalPlugin {
    env_filter: Cow<'static, str>,
}

impl DebugTerminalPlugin {
    pub fn new<S: Into<Cow<'static, str>>>(env_filter: S) -> Self {
        Self {
            env_filter: env_filter.into(),
        }
    }
}

impl Plugin for DebugTerminalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Initialize logs
        let shared_logs = SharedLogs::with_capacity(32);
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new(&self.env_filter))
            .with_writer(shared_logs.clone())
            .init();

        // Initialize terminal
        let mut stdout = io::stdout();
        enable_raw_mode().expect("failed to enable raw mode");
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .expect("failed to enter alternative screen");
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("failed to initialize terminal");

        app.init_resource::<FrameCounter>()
            .insert_resource(shared_logs)
            .insert_resource(DebugTerminal::new(terminal))
            .add_system(record_fps.system())
            .add_system(terminal_render.system());
    }
}

pub struct DebugTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl DebugTerminal {
    const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

    fn render<'a, I, S>(&mut self, fps_data: &[(&str, u64)], logs: I)
    where
        S: Into<tui::text::Text<'a>> + 'a,
        I: IntoIterator<Item = S>,
    {
        self.terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size());

                // FPS Barchart
                let barchart = BarChart::default()
                    .block(Block::default().title("FPS").borders(Borders::ALL))
                    .data(fps_data)
                    .bar_width(9)
                    .bar_style(Style::default().fg(Color::Yellow))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
                f.render_widget(barchart, chunks[0]);

                // Logs
                let block = Block::default().borders(Borders::ALL).title("Events");
                let items: Vec<_> = logs.into_iter().map(ListItem::new).collect();
                let list = List::new(items)
                    .block(block)
                    .start_corner(Corner::BottomLeft);
                f.render_widget(list, chunks[1]);
            })
            .expect("failed to render terminal");
    }
}

impl DebugTerminal {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> Self {
        Self { terminal }
    }
}

pub struct LastRender(Instant);

impl Default for LastRender {
    fn default() -> Self {
        Self(Instant::now())
    }
}

pub fn terminal_render(
    time: Res<Time>,
    mut last_render: Local<LastRender>,
    mut terminal: ResMut<DebugTerminal>,
    frames: Res<FrameCounter>,
    logs: ResMut<SharedLogs>,
) {
    let delta = time
        .last_update()
        .and_then(|now| now.checked_duration_since(last_render.0));
    if let Some(delta) = delta {
        if delta > DebugTerminal::UPDATE_INTERVAL {
            let logs = logs.0.lock().expect("poisoned");
            let recent_logs = logs
                .iter()
                .rev()
                .take(10)
                .map(|x| ansi_to_text(x.as_bytes().to_vec()))
                .flatten();
            terminal.render(frames.history(), recent_logs);
            *last_render = LastRender(Instant::now());
        }
    }
}
