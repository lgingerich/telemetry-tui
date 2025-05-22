use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_sensor_status_panel(f: &mut Frame, area: Rect) {
    let sensor_text = "Sensor Status: OK (Placeholder)"; // Placeholder
    let sensor_panel = Paragraph::new(sensor_text)
        .block(Block::default().title("Sensor Status").borders(Borders::ALL));
    f.render_widget(sensor_panel, area);
} 