use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};
use key_handler::{handle_new_tasks, handle_projects, handle_task_editor, handle_tasks};
use new_task::NewTask;
use projects::Projects;
use sections::Sections;
use std::sync::{
    mpsc::{self, Receiver, Sender, TryRecvError},
    Arc,
};
use tasks::{Filter, Task, Tasks};
use tokio::sync::Mutex;

mod api_calls;
mod error;
mod key_handler;
mod new_task;
mod projects;
mod sections;
mod task_edit;
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
    pub tasks: Tasks,
    pub show_help: bool,
    pub sections: Sections,
    pub show_task_editor: bool,
    pub task_edit: task_edit::TaskEdit<'a>,
    pub show_new_task: bool,
    pub new_task: NewTask<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx): (Sender<Task>, Receiver<Task>) = mpsc::channel();
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
                        handle_task_editor(&mut app, key, client.clone());
                        continue;
                    }

                    if app.show_new_task {
                        handle_new_tasks(&mut app, key, client.clone(), tx.clone());
                        continue;
                    }

                    if app.show_new_task {
                        if key.code == KeyCode::Esc {
                            app.show_new_task = false;
                        } else if key.code == KeyCode::Enter {
                        }
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
                        handle_projects(&mut app, key)
                    } else if app.current_focus == CurrentFocus::Tasks {
                        handle_tasks(&mut app, key, client.clone())
                    }
                }
            }
        }

        match rx.try_recv() {
            Ok(received) => {
                app.tasks.tasks.push(received);
                app.tasks.filter_task_list();
            }
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => break,
        }
    }
    tui::restore()?;
    let _ = initialise_task.await;
    Ok(())
}
