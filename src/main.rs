use api_calls::fetchProjects;
use crossterm::event::{self, KeyCode, KeyEventKind};
use projects::Project;
use ratatui::{style::Stylize, widgets::Paragraph};

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
        let projects = fetchProjects().await.unwrap();
        self.projects = projects;
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut terminal = tui::init()?;
    let mut app = App::new();

    // let initialise_task = tokio::spawn(async move {
    //     app.initialise().await;
    // });

    loop {
        terminal.draw(|frame| tui::ui(frame))?;
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    tui::restore()?;    
    // api_calls::test().await.unwrap();
    // let _ = initialise_task.await;
    Ok(())
}
