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

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
    pub projects: Projects,
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
                    if key.code == KeyCode::Char('q') {
                        break;
                    } else if key.code == KeyCode::Char('a') {
                        app.projects.next();
                    }
                }
            }
        }
    }
    tui::restore()?;
    let _ = initialise_task.await;
    Ok(())
}
