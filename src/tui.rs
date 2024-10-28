use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, HighlightSpacing, List, ListItem, Paragraph,
    },
};

mod help;
mod new_task;
mod task_editor;
mod utils;

use crate::{tasks::Filter, App, CurrentFocus};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> std::io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Todoist",
        Style::default().fg(Color::Indexed(47)),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for project in &app.projects.projects {
        let color = if let Some(selected_project) = &app.projects.selected_project {
            if project.id == *selected_project {
                Color::Indexed(214)
            } else {
                Color::Yellow
            }
        } else {
            Color::Yellow
        };

        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", project.name),
            Style::default().fg(color),
        ))));
    }

    let my_projects_block = Block::default()
        .title(Title::from(" My projects ".bold()).alignment(Alignment::Left))
        .borders(Borders::ALL)
        .fg(match app.current_focus {
            CurrentFocus::Projects => Color::Indexed(47),
            _ => Color::White,
        });

    let list = List::new(list_items)
        .block(my_projects_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(list, inner_layout[0], &mut app.projects.state);

    let task_title = match app.tasks.filter {
        Filter::All => " All ",
        Filter::Today => " Today ",
        Filter::ProjectId(_) => " Tasks ",
        Filter::Overdue => " Overdue ",
    };

    let instructions = Title::from(Line::from(vec![
        " For help, press ".into(),
        "h ".blue().bold(),
    ]));

    let tasks_block = Block::default()
        .title(Title::from(task_title.bold()).alignment(Alignment::Left))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .fg(match app.current_focus {
            CurrentFocus::Tasks => Color::Indexed(47),
            _ => Color::White,
        });

    let mut task_list_item = Vec::new();
    for i in &app.tasks.display_tasks {
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

    f.render_stateful_widget(task_list, inner_layout[1], &mut app.tasks.state);

    // help popup
    if app.show_help {
        help::help(f);
    }

    if app.show_task_editor {
        task_editor::editor(f, app)
    }

    if app.show_new_task {
        new_task::editor(f, app)
    }
}
