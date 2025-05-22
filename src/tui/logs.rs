use ratatui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw_logs_panel(f: &mut Frame, area: Rect, logs: &[String]) {
    let log_items: Vec<ListItem> = logs
        .iter()
        .rev()
        .take(area.height.into()) // Take as many logs as fit in the panel
        .map(|log| ListItem::new(Span::raw(log)))
        .collect();

    let log_list = List::new(log_items)
        .block(Block::default().title("Logs").borders(Borders::ALL));
    f.render_widget(log_list, area);
}
