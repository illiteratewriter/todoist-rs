use crossterm::event::{self, KeyCode, KeyEventKind};
use projects::Projects;
use std::sync::Arc;
use tokio::sync::Mutex;

mod api_calls;
mod projects;
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
    Tasks
}

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
    pub projects: Projects,
    pub current_focus: CurrentFocus
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub async fn initialise(&mut self) {
        self.projects.initialise().await;
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut terminal = tui::init()?;
    let app = Arc::new(Mutex::new(App::new()));

    let app_clone = Arc::clone(&app);
    let initialise_task = tokio::spawn(async move {
        let mut app = app_clone.lock().await;
        app.initialise().await;
    });

    loop {
        let mut app = app.lock().await;
        terminal.draw(|frame| tui::ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('t') {
                        println!("T was pressed");
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    } else if key.code == KeyCode::Tab {
                        match app.current_focus {
                            CurrentFocus::Projects => app.current_focus = CurrentFocus::Tasks,
                            CurrentFocus::Tasks => app.current_focus = CurrentFocus::Projects
                        }
                    }
                    
                    if app.current_focus == CurrentFocus::Projects {
                        if key.code == KeyCode::Char('j') {
                            app.projects.next();
                        } else if key.code == KeyCode::Char('k') {
                            app.projects.previous();
                        } else if key.code == KeyCode::Enter {
                            println!("FROM HERE ");
                            app.projects.select().await;
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
