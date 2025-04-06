use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Row, Table},
};

use crate::tui::utils;

pub fn help(f: &mut Frame) {
    let close_help = Line::from(vec![
        " To close, press ".into(),
        "h".blue().bold(),
        " again ".into(),
    ]);
    let block = Block::default()
        .title(" Help ")
        .title_bottom(close_help.centered())
        .borders(Borders::ALL);

    let rows = [
        Row::new(vec!["t", "Today's tasks"]),
        Row::new(vec!["q", "Quit"]),
        Row::new(vec![
            "x",
            "Press x while a task is highlighted to mark as done",
        ]),
        Row::new(vec![
            "d",
            "Press d while a task is highlighted to delete the task",
        ]),
        Row::new(vec![
            "+",
            "Press + while on a project to add task to the project",
        ]),
        Row::new(vec!["o", "Overdue tasks"]),
        Row::new(vec!["a", "All tasks"]),
        Row::new(vec!["p", "Sort by priority"]),
        Row::new(vec!["d", "Sort by date"]),
        Row::new(vec!["Tab", "Switch between projects and tasks"]),
    ];
    let row_count = rows.len();
    let total_height = row_count + 5;

    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Shortcut", "Functionality"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .block(block);

    let area = utils::centered_rect(
        Constraint::Percentage(60),
        Constraint::Length(total_height as u16),
        f.area(),
    );
    f.render_widget(Clear, area);
    f.render_widget(table, area);
}
