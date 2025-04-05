extern crate chrono;
use std::collections::HashMap;

use chrono::{Local, NaiveDate};
use ratatui::widgets::ListState;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct Tasks {
    pub tasks: Vec<Task>,
    pub filter: Filter,
    pub state: ListState,
    pub tasks_with_children: HashMap<String, u16>,
    pub display_tasks: Vec<usize>,
}

impl Tasks {
    pub fn new(items: Vec<Task>) -> Tasks {
        Tasks {
            tasks: items,
            filter: Filter::Today,
            state: ListState::default(),
            tasks_with_children: HashMap::new(),
            display_tasks: Vec::new(),
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

    pub fn sort_tasks(&mut self) {
        // sort display tasks by task priority in descending order
        self.display_tasks.sort_by(|a, b| {
            let task_a = &self.tasks[*a];
            let task_b = &self.tasks[*b];
            task_b.priority.cmp(&task_a.priority) // Reverse the order
        });
    }

    pub fn filter_task_list(&mut self) {
        self.state = ListState::default();
        self.display_tasks = Vec::new();
        for (index, task) in self.tasks.iter().enumerate() {
            if let Some(_) = task.parent_id {
                continue;
            }
            match &self.filter {
                Filter::All => {
                    self.display_tasks.push(index);
                }
                Filter::Today => {
                    let today = Local::now().date_naive();
                    if let Some(due) = &task.due {
                        if due.date == today {
                            self.display_tasks.push(index);
                        }
                    }
                }
                Filter::ProjectId(project_id) => {
                    if task.project_id == *project_id {
                        self.display_tasks.push(index);
                    }
                }
                Filter::Overdue => {
                    let today = Local::now().date_naive();
                    if let Some(due) = &task.due {
                        if due.date < today {
                            self.display_tasks.push(index);
                        }
                    }
                }
            }
        }
    }

    pub fn next(&mut self) {
        if self.display_tasks.len() == 0 {
            self.state.select(None);
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.display_tasks.len() - 1 {
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
        if self.display_tasks.len() == 0 {
            self.state.select(None);
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.display_tasks.len() - 1
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub section_id: Option<String>,
    pub content: String,
    pub description: String,
    pub is_completed: bool,
    pub labels: Vec<String>,
    pub parent_id: Option<String>,
    pub order: i32,
    pub priority: u8,
    pub due: Option<Due>,
    pub url: String,
    pub comment_count: u16,
    #[serde(skip_serializing)]
    pub created_at: String,
    #[serde(skip_serializing)]
    pub creator_id: String,
    pub assignee_id: Option<String>,
    pub assigner_id: Option<String>,
    pub duration: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Due {
    pub string: String,
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
