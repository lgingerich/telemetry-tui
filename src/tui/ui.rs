use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

use super::{
    config::draw_config_panel, logs::draw_logs_panel,
    robot_state::draw_robot_state_panel, sensor_status::draw_sensor_status_panel,
};

pub fn draw_ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);
    
    draw_robot_state_panel(f, top_chunks[0], &app.latest_state);
    draw_sensor_status_panel(f, top_chunks[1]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70), // Logs panel
                Constraint::Percentage(30), // Config panel
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    draw_logs_panel(f, bottom_chunks[0], &app.logs);
    draw_config_panel(f, bottom_chunks[1]);
} 