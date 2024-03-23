use api_calls::fetch_projects;
use crossterm::event::{self, KeyCode, KeyEventKind};
use projects::Project;
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
    pub projects: Vec<Project>
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub async fn initialise(&mut self) {
        let projects = fetch_projects().await.unwrap();
        self.projects = projects;
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
        terminal.draw(|frame| {
            tui::ui(frame, &mut app)
        })?;
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
    }
    tui::restore()?;    
    let _ = initialise_task.await;
    Ok(())
}