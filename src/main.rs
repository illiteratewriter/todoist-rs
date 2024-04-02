use crossterm::event::{self, KeyCode, KeyEventKind};
use projects::Projects;
use ratatui::widgets::ListState;
use sections::Sections;
use std::sync::Arc;
use tasks::{Filter, Tasks};
use tokio::sync::Mutex;
use tui_textarea::TextArea;
use color_eyre::Result;

mod api_calls;
mod projects;
mod sections;
mod tasks;
mod tui;
mod error;
mod task_edit;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
}

#[derive(Debug, Default, PartialEq)]
pub enum CurrentFocus {
    #[default]
    Projects,
    Tasks,
    Help,
}

#[derive(Debug, Default)]
pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub exit: bool,
    pub projects: Projects,
    pub current_focus: CurrentFocus,
    pub tasks: Tasks,
    pub show_help: bool,
    pub sections: Sections,
    pub show_task_editor: bool,
    pub task_edit: task_edit::TaskEdit<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    error::install_hooks()?;
    let mut terminal = tui::init()?;
    let app = Arc::new(Mutex::new(App::new()));

    let app_clone = Arc::clone(&app);
    let client_clone = client.clone();
    let initialise_task = tokio::spawn(async move {
        // todo: make network calls parallel
        let project_resp = api_calls::fetch_projects(&client_clone).await.unwrap();
        let projects = Projects::new(project_resp);
        let task_resp = api_calls::fetch_tasks(&client_clone).await.unwrap();
        let tasks = Tasks::new(task_resp);
        let mut app = app_clone.lock().await;
        let sections_resp = api_calls::fetch_sections(&client_clone).await.unwrap();
        let sections = Sections::new(sections_resp);
        app.projects = projects;
        app.tasks = tasks;
        app.sections = sections;
        app.tasks.filter_task_list();
        app.tasks.find_tasks_with_children();
    });

    loop {
        let mut app = app.lock().await;
        terminal.draw(|frame| tui::ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.show_task_editor {
                        if key.code == KeyCode::Esc {
                            app.show_task_editor = !app.show_task_editor;
                        } else if key.code == KeyCode::Enter {
                            app.show_task_editor = !app.show_task_editor;
                            if let Some(selected) = app.tasks.state.selected() {
                                // app.show_task_editor = true;
                                let index = app.tasks.display_tasks[selected];

                                app.tasks.tasks[index].content = app.task_edit.content.lines().join("\n");
                                app.tasks.tasks[index].description = app.task_edit.description.lines().join("\n");
                                let task = app.tasks.tasks[index].clone();
                                let client_clone = client.clone();
                                tokio::spawn(async move{
                                    let _ = api_calls::update_task(&client_clone, task).await;
                                });
                            }
                        }
                        if key.code == KeyCode::Tab {
                            if app.task_edit.currently_editing == task_edit::CurrentlyEditing::Content {
                                app.task_edit.currently_editing = task_edit::CurrentlyEditing::Description
                            } else if app.task_edit.currently_editing == task_edit::CurrentlyEditing::Description{
                                app.task_edit.currently_editing = task_edit::CurrentlyEditing::ChildTasks
                            } else {
                                app.task_edit.currently_editing = task_edit::CurrentlyEditing::Content
                            }
                            continue;
                        }

                        if app.task_edit.currently_editing == task_edit::CurrentlyEditing::Content {
                            app.task_edit.content.input(key);
                        } else if app.task_edit.currently_editing == task_edit::CurrentlyEditing::Description {
                            app.task_edit.description.input(key);
                        } else {
                            if key.code == KeyCode::Char('j') {
                                app.task_edit.next();
                            } else if key.code == KeyCode::Char('k') {
                                app.task_edit.previous();
                            } else if key.code == KeyCode::Enter {
                                todo!("add open sub task on pressing enter");
                            }
                        }
                        continue;
                    }

                    if key.code == KeyCode::Char('h') {
                        app.show_help = !app.show_help;
                    } else if key.code == KeyCode::Char('q') {
                        break;
                    } else if key.code == KeyCode::Char('t') {
                        app.tasks.filter = Filter::Today;
                        app.tasks.filter_task_list();
                        app.projects.unselect();
                    } else if key.code == KeyCode::Char('o') {
                        app.tasks.filter = Filter::Overdue;
                        app.tasks.filter_task_list();
                        app.projects.unselect();
                    }

                    if app.show_help {
                        continue;
                    }

                    if key.code == KeyCode::Tab {
                        match app.current_focus {
                            CurrentFocus::Projects => app.current_focus = CurrentFocus::Tasks,
                            CurrentFocus::Tasks => app.current_focus = CurrentFocus::Projects,
                            _ => {}
                        }
                    }

                    if app.current_focus == CurrentFocus::Projects {
                        if key.code == KeyCode::Char('j') {
                            app.projects.next();
                        } else if key.code == KeyCode::Char('k') {
                            app.projects.previous();
                        } else if key.code == KeyCode::Enter {
                            if let Some(selected) = app.projects.state.selected() {
                                let selected_id = app.projects.projects[selected].id.clone();
                                app.tasks.filter =
                                    crate::tasks::Filter::ProjectId(selected_id.clone());
                                app.tasks.filter_task_list();
                                app.projects.selected_project = Some(selected_id);
                            }
                        } else if key.code == KeyCode::Char('x') {

                        }
                    } else if app.current_focus == CurrentFocus::Tasks {
                        if key.code == KeyCode::Char('j') {
                            app.tasks.next();
                        } else if key.code == KeyCode::Char('k') {
                            app.tasks.previous();
                        } else if key.code == KeyCode::Enter {
                            if let Some(selected) = app.tasks.state.selected() {
                                app.show_task_editor = true;
                                let index = app.tasks.display_tasks[selected];
                                let selected = &app.tasks.tasks[index];

                                let mut children = Vec::new();

                                for (index, task) in app.tasks.tasks.iter().enumerate() {
                                    if task.parent_id == Some(selected.id.clone()) {
                                        children.push(index);
                                    }
                                }

                                app.task_edit = task_edit::TaskEdit {
                                    content: TextArea::from(vec![selected.content.clone()]),
                                    description: TextArea::from(vec![selected.description.clone()]),
                                    currently_editing: task_edit::CurrentlyEditing::Content,
                                    children: children,
                                    children_list_state: ListState::default(),
                                }
                            }
                            // app.tasks.select().await;
                        }
                    }
                }
            }
        }
    }
    tui::restore()?;
    let _ = initialise_task.await;
    Ok(())
}
