extern crate chrono;
use chrono::{Local, NaiveDate};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{ListItem, ListState},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct Tasks<'a> {
    pub tasks: Vec<Task>,
    pub filter: Filter,
    pub state: ListState,
    pub task_list: Vec<ListItem<'a>>,
}

fn generate_list_item<'a>(content: &String) -> ListItem<'a> {
    ListItem::new(Line::from(Span::styled(
        format!("{}", content),
        Style::default().fg(Color::Yellow),
    )))
}

impl<'a> Tasks<'a> {
    pub fn new(items: Vec<Task>) -> Tasks<'a> {
        Tasks {
            tasks: items,
            filter: Filter::Today,
            state: ListState::default(),
            task_list: Vec::<ListItem>::new(),
        }
    }

    pub fn filter_task_list(&mut self) {
        self.task_list = Vec::<ListItem>::new();
        self.state = ListState::default();
        for task in &self.tasks {
            match &self.filter {
                Filter::All => {
                    self.task_list.push(generate_list_item(&task.content));
                }
                Filter::Today => {
                    let today = Local::now().date_naive();
                    if let Some(due) = &task.due {
                        if due.date == today {
                            self.task_list.push(generate_list_item(&task.content));
                        }
                    }
                }
                Filter::ProjectId(project_id) => {
                    if task.project_id == *project_id {
                        self.task_list.push(generate_list_item(&task.content));
                    }
                }
            }
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.task_list.len() - 1 {
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
                    self.task_list.len() - 1
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
}

#[derive(Debug, Default)]
pub enum Filter {
    #[default]
    All,
    Today,
    ProjectId(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub section_id: Option<String>,
    pub content: String,
    pub description: String,
    pub is_completed: bool,
    pub labels: Vec<String>,
    pub parent_id: Option<String>,
    pub order: u8,
    pub priority: u8,
    pub due: Option<Due>,
    pub url: String,
    pub comment_count: u16,
    pub created_at: String,
    pub creator_id: String,
    pub assignee_id: Option<String>,
    pub assigner_id: Option<String>,
    pub duration: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Due {
    pub string: String,
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Duration {
    amount: u32,
    unit: String,
}

pub mod date_format {
    use super::*;

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format("%Y-%m-%d"));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}
