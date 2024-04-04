use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear
    },
};

use crate::{task_edit::CurrentlyEditing, tui::utils, App};

pub fn editor(f: &mut Frame, app: &mut App) {
    let area = utils::centered_rect(60, 40, f.size());

    f.render_widget(Clear, area);

    // let tasks_block = Block::default()
    //     .title(Title::from(" Sub tasks ".bold()).alignment(Alignment::Left))
    //     .borders(Borders::ALL)
    //     .fg(match app.task_edit.currently_editing {
    //         CurrentlyEditing::ChildTasks => Color::Green,
    //         _ => Color::White,
    //     });

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

    app.task_edit
        .content
        .set_block(Block::default().borders(Borders::ALL).title("Task").fg(
            match app.task_edit.currently_editing {
                CurrentlyEditing::Content => Color::Green,
                _ => Color::White,
            },
        ));

    app.task_edit.description.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .fg(match app.task_edit.currently_editing {
                CurrentlyEditing::Description => Color::Green,
                _ => Color::White,
            }),
    );

    let content = app.task_edit.content.widget();
    let description = app.task_edit.description.widget();

    f.render_widget(content, vertical_split[0]);
    f.render_widget(description, vertical_split[1]);

    let close_modal_desc = Title::from(Line::from(vec![
        " To save, press ".into(),
        "Enter".blue().bold(),
        " and to close, press ".into(),
        "Esc".blue().bold(),
    ]));

    let block = Block::default()
        .title(" Edit task ")
        .title(
            close_modal_desc
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL);

    f.render_widget(block, area);
}
