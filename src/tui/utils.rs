use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use ratatui::{prelude::*, widgets::ListItem};
use textwrap::fill;

use crate::tasks::Due;

pub fn centered_rect(horizontal: Constraint, vertical: Constraint, r: Rect) -> Rect {
    let popup_layout = match vertical {
        Constraint::Percentage(percent_y) => Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r),
        Constraint::Length(height) => {
            let vertical_margin = (r.height.saturating_sub(height)) / 2;
            Layout::vertical([
                Constraint::Length(vertical_margin),
                Constraint::Length(height),
                Constraint::Length(vertical_margin),
            ])
            .split(r)
        }
        _ => panic!("Unsupported vertical constraint"),
    };

    match horizontal {
        Constraint::Percentage(percent_x) => Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1],
        Constraint::Length(width) => {
            let horizontal_margin = (r.width.saturating_sub(width)) / 2;
            Layout::horizontal([
                Constraint::Length(horizontal_margin),
                Constraint::Length(width),
                Constraint::Length(horizontal_margin),
            ])
            .split(popup_layout[1])[1]
        }
        _ => panic!("Unsupported horizontal constraint"),
    }
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

    let formatted_due = if let Some(due) = due {
        if let Some(datetime_str) = &due.datetime {
            if let Ok(datetime) = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S") {
                format!(
                    "(due: {} at {:02}:{:02})",
                    format_date(datetime.date()),
                    datetime.hour(),
                    datetime.minute()
                )
            } else {
                format!("(due: {})", format_date(due.date))
            }
        } else {
            format!("(due: {})", format_date(due.date))
        }
    } else {
        String::new()
    };

    let formatted_text = format!(
        "[{}] {} {} {} - {}",
        if is_completed { "✓" } else { " " },
        if children > 0 { "⤷" } else { " " },
        content,
        formatted_due,
        color
    );

    let wrapped_text = fill(&formatted_text, width);

    ListItem::new(Text::styled(
        wrapped_text,
        Style::default().fg(Color::White),
    ))
}

fn format_date(date: NaiveDate) -> String {
    let month = match date.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };
    format!("{:02} {}, {}", date.day(), month, date.year())
}
