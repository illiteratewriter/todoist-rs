use crossterm::event::{self, KeyCode, KeyEventKind};
use projects::Projects;
use sections::Sections;
use std::sync::Arc;
use tasks::{Filter, Tasks};
use tokio::sync::Mutex;

mod api_calls;
mod projects;
mod sections;
mod tasks;
mod tui;

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
    pub tasks: Tasks<'a>,
    pub show_help: bool,
    pub sections: Sections,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App::default()
    }

    // pub async fn initialise(&mut self) {
    //     self.projects.initialise().await;
    // }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut terminal = tui::init()?;
    let app = Arc::new(Mutex::new(App::new()));

    let app_clone = Arc::clone(&app);
    let initialise_task = tokio::spawn(async move {
        // todo: make network calls parallel
        let project_resp = api_calls::fetch_projects().await.unwrap();
        let projects = Projects::new(project_resp);
        let task_resp = api_calls::fetch_tasks().await.unwrap();
        let tasks = Tasks::new(task_resp);
        let mut app = app_clone.lock().await;
        let sections_resp = api_calls::fetch_sections().await.unwrap();
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
                    if key.code == KeyCode::Char('h') {
                        app.show_help = !app.show_help;
                    } else if key.code == KeyCode::Char('q') {
                        break;
                    } else if key.code == KeyCode::Char('t') {
                        app.tasks.filter = Filter::Today;
                        app.tasks.filter_task_list();
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
                        }
                    } else if app.current_focus == CurrentFocus::Tasks {
                        if key.code == KeyCode::Char('j') {
                            app.tasks.next();
                        } else if key.code == KeyCode::Char('k') {
                            app.tasks.previous();
                        } else if key.code == KeyCode::Enter {
                            println!("FROM HERE ");
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
