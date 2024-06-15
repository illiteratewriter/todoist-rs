use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear,
    },
};

use crate::{new_task::CurrentlyEditing, tui::utils, App};

pub fn editor(f: &mut Frame, app: &mut App) {
    let area = utils::centered_rect(60, 40, f.size());

    f.render_widget(Clear, area);

    let inner_area = area.inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    let vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(inner_area);

    app.new_task
        .content
        .set_block(Block::default().borders(Borders::ALL).title("Task").fg(
            match app.new_task.currently_editing {
                CurrentlyEditing::Content => Color::Indexed(47),
                _ => Color::White,
            },
        ));

    app.new_task.description.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .fg(match app.new_task.currently_editing {
                CurrentlyEditing::Description => Color::Indexed(47),
                _ => Color::White,
            }),
    );

    app.new_task.due_string.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Due String")
            .fg(match app.new_task.currently_editing {
                CurrentlyEditing::DueString => Color::Indexed(47),
                _ => Color::White,
            }),
    );

    let content = app.new_task.content.widget();
    let description = app.new_task.description.widget();
    let due_string = app.new_task.due_string.widget();

    f.render_widget(content, vertical_split[0]);
    f.render_widget(description, vertical_split[1]);
    f.render_widget(due_string, vertical_split[2]);

    let close_modal_desc = Title::from(Line::from(vec![
        " To save, press ".into(),
        "Enter".blue().bold(),
        " and to close, press ".into(),
        "Esc".blue().bold(),
    ]));

    let block = Block::default()
        .title(" New task ")
        .title(
            close_modal_desc
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL);

    f.render_widget(block, area);
}
