use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, HighlightSpacing, List},
};

use crate::{task_edit::CurrentlyEditing, tui::utils, App};

pub fn editor(f: &mut Frame, app: &mut App) {
    let area = utils::centered_rect(
        Constraint::Percentage(60),
        Constraint::Percentage(40),
        f.area(),
    );

    f.render_widget(Clear, area);

    let inner_area = area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    let vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(inner_area);

    let task_list_width = vertical_split[1].width as usize;

    let tasks_block = Block::default()
        .title(" Sub tasks ")
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
            &task.due,
            task.priority,
            task.is_completed,
            children,
            task_list_width - 4,
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

    app.task_edit
        .content
        .set_block(Block::default().borders(Borders::ALL).title(" Task ").fg(
            match app.task_edit.currently_editing {
                CurrentlyEditing::Content => Color::Indexed(47),
                _ => Color::White,
            },
        ));

    app.task_edit.description.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Description ")
            .fg(match app.task_edit.currently_editing {
                CurrentlyEditing::Description => Color::Indexed(47),
                _ => Color::White,
            }),
    );

    app.task_edit
        .due_string
        .set_block(Block::default().borders(Borders::ALL).title(" Due ").fg(
            match app.task_edit.currently_editing {
                CurrentlyEditing::DueString => Color::Indexed(47),
                _ => Color::White,
            },
        ));

    let content = app.task_edit.content.widget();
    let description = app.task_edit.description.widget();
    let due_string = app.task_edit.due_string.widget();

    f.render_widget(content, vertical_split[0]);
    f.render_widget(description, vertical_split[1]);
    f.render_widget(due_string, vertical_split[2]);

    let close_modal_desc = Line::from(vec![
        " To save, press ".into(),
        "Enter".blue().bold(),
        " and to close, press ".into(),
        "Esc".blue().bold(),
    ]);

    let block = Block::default()
        .title(" Edit task ")
        .title_bottom(close_modal_desc.centered())
        .borders(Borders::ALL);

    f.render_stateful_widget(
        task_list,
        vertical_split[3],
        &mut app.task_edit.children_list_state,
    );

    f.render_widget(block, area);
}
