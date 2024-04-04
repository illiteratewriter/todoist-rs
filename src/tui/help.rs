use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, Row, Table,
    },
};

use crate::tui::utils;

pub fn help(f: &mut Frame) {
    let close_help = Title::from(Line::from(vec![
        " To close, press ".into(),
        "h".blue().bold(),
        " again ".into(),
    ]));
    let block = Block::default()
        .title(" Help ")
        .title(
            close_help
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL);

    let rows = [
        Row::new(vec!["t", "Today's tasks"]),
        Row::new(vec!["q", "Quit"]),
        Row::new(vec![
            "x",
            "Press x while a task is highlighted to mark as done",
        ]),
    ];
    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Shortcut", "Functionality"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>")
        .block(block);

    let area = utils::centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(table, area);
}
