use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_robot_state_panel(
    f: &mut Frame,
    area: Rect,
    latest_state: &Option<(f32, f32, f32, f32, f32, String)>,
) {
    let state_text = if let Some((x, y, theta, vx, vy, mode)) = latest_state {
        format!(
            "Pose: x={:.2}, y={:.2}, θ={:.2}°\nVelocity: vx={:.2}, vy={:.2}\nMode: {}",
            x, y, theta, vx, vy, mode
        )
    } else {
        "Waiting for data...".to_string()
    };

    let state_panel = Paragraph::new(state_text)
        .block(Block::default().title("Robot State").borders(Borders::ALL));
    f.render_widget(state_panel, area);
} 