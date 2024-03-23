use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

use crate::api_calls::{self};

#[derive(Debug, Default)]
pub struct Projects {
    pub projects: Vec<Project>,
    pub state: ListState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub comment_count: u8,
    pub order: u8,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub is_team_inbox: bool,
    pub view_style: ListType,
    pub url: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    Board,
    List,
}

impl Projects {
    pub fn new() -> Projects {
        Projects {
            projects: vec![],
            state: ListState::default(),
        }
    }

    pub async fn initialise(&mut self) {
        self.projects = api_calls::fetch_projects().await.unwrap();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.projects.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.projects.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }

    pub async fn select(&mut self) {
        println!("AND FROM HERE");
        let i = self.state.selected().unwrap();
        // self.projects[i].get_sections();
    }

    pub fn get_sections(&mut self) {
        println!("{:?}", self.state)
    }
}
