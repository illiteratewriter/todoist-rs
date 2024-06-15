use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, HighlightSpacing, List,
    },
};

use crate::{task_edit::CurrentlyEditing, tui::utils, App};

pub fn editor(f: &mut Frame, app: &mut App) {
    let area = utils::centered_rect(60, 40, f.size());

    f.render_widget(Clear, area);

    let tasks_block = Block::default()
        .title(Title::from(" Sub tasks ".bold()).alignment(Alignment::Left))
        .borders(Borders::ALL)
        .fg(match app.task_edit.currently_editing {
            CurrentlyEditing::ChildTasks => Color::Indexed(47),
            _ => Color::White,
        });

    let mut task_list_item = Vec::new();
    for i in &app.task_edit.children {
        let task = &app.tasks.tasks[*i];
        let children: u16 = *app.tasks.tasks_with_children.get(&task.id).unwrap_or(&0);
        task_list_item.push(utils::generate_list_item(
            &task.content,
            task.is_completed,
            children,
        ))
    }

    let task_list = List::new(task_list_item)
        .block(tasks_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Cyan),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

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
                CurrentlyEditing::Content => Color::Indexed(47),
                _ => Color::White,
            },
        ));

    app.task_edit.description.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .fg(match app.task_edit.currently_editing {
                CurrentlyEditing::Description => Color::Indexed(47),
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

    f.render_stateful_widget(
        task_list,
        vertical_split[2],
        &mut app.task_edit.children_list_state,
    );

    f.render_widget(block, area);
}
