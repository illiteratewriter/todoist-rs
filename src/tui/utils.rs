use ratatui::{prelude::*, widgets::ListItem};
use textwrap::fill;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn generate_list_item<'a>(
    content: &String,
    is_completed: bool,
    children: u16,
    width: usize,
) -> ListItem<'a> {
    let formatted_text = format!(
        "[{}] {} {}",
        if is_completed { "✓" } else { " " },
        if children > 0 { "⤷" } else { " " },
        content
    );

    let wrapped_text = fill(&formatted_text, width);

    ListItem::new(Text::styled(
        wrapped_text,
        Style::default().fg(Color::Yellow),
    ))
}
