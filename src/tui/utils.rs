use ratatui::{prelude::*, widgets::ListItem};
use textwrap::fill;

use crate::tasks::Due;

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
    due: &Option<Due>,
    priority: u8,
    is_completed: bool,
    children: u16,
    width: usize,
) -> ListItem<'a> {

    let color = match priority {
        1 => "P4",
        2 => "P3",
        3 => "P2",
        4 => "P1",
        _ => "",
    };


    let formatted_text = format!(
        "[{}] {} {} {} - {}",
        if is_completed { "✓" } else { " " },
        if children > 0 { "⤷" } else { " " },
        content,
        if let Some(due) = due {
            format!(" (due: {})", due.string)
        } else {
            String::new()
        },
        color
    );

    let wrapped_text = fill(&formatted_text, width);

    // show colors according to priority from 1 to 4

    ListItem::new(Text::styled(
        wrapped_text,
        Style::default().fg(Color::White),
    ))
}
