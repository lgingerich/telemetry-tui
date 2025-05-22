use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_config_panel(f: &mut Frame, area: Rect) {
    let config_text = "Config Panel (Placeholder)";
    let config_panel = Paragraph::new(config_text)
        .block(Block::default().title("Configuration").borders(Borders::ALL));
    f.render_widget(config_panel, area);
}
