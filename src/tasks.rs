extern crate chrono;
use std::collections::HashMap;

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
    pub tasks_with_children: HashMap<String, u16>,
}

fn generate_list_item<'a>(content: &String, is_completed: bool, children: u16) -> ListItem<'a> {
    ListItem::new(Line::from(Span::styled(
        format!(
            "[{}] {} {}",
            if is_completed { "✓" } else { " " },
            if children > 0 { "⤷" } else { " " },
            content
        ),
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
            tasks_with_children: HashMap::new(),
        }
    }

    pub fn find_tasks_with_children(&mut self) {
        for task in &self.tasks {
            if let Some(parent_id) = &task.parent_id {
                *self
                    .tasks_with_children
                    .entry(parent_id.clone())
                    .or_insert(0) += 1;
            }
        }
    }

    pub fn filter_task_list(&mut self) {
        self.task_list = Vec::<ListItem>::new();
        self.state = ListState::default();
        for task in &self.tasks {
            let children: u16 = *self.tasks_with_children.get(&task.id).unwrap_or(&0);

            match &self.filter {
                Filter::All => {
                    self.task_list.push(generate_list_item(
                        &task.content,
                        task.is_completed,
                        children,
                    ));
                }
                Filter::Today => {
                    let today = Local::now().date_naive();
                    if let Some(due) = &task.due {
                        if due.date == today {
                            self.task_list.push(generate_list_item(
                                &task.content,
                                task.is_completed,
                                children,
                            ));
                        }
                    }
                }
                Filter::ProjectId(project_id) => {
                    if task.project_id == *project_id {
                        self.task_list.push(generate_list_item(
                            &task.content,
                            task.is_completed,
                            children,
                        ));
                    }
                }
                Filter::Overdue => {
                    let today = Local::now().date_naive();
                    if let Some(due) = &task.due {
                        if due.date < today {
                            self.task_list.push(generate_list_item(
                                &task.content,
                                task.is_completed,
                                children,
                            ));
                        }
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
    Overdue,
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
