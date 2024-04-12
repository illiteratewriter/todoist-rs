use serde::{Serialize, Serializer};
use tui_textarea::TextArea;

#[derive(Debug, Default, Serialize)]
pub struct NewTask<'a> {
    #[serde(serialize_with = "serialize_text_area")]
    pub content: TextArea<'a>,
    #[serde(serialize_with = "serialize_text_area")]
    pub description: TextArea<'a>,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<usize>,
    pub labels: Option<Vec<String>>,
    pub priority: Option<u8>,
    #[serde(serialize_with = "serialize_text_area")]
    pub due_string: TextArea<'a>,
    pub due_date: Option<String>,
    pub due_datetime: Option<String>,
    pub due_lang: Option<String>,
    pub assignee_id: Option<String>,
    pub duration: Option<usize>,
    pub duration_unit: Option<DurationUnit>,
    #[serde(skip)]
    pub currently_editing: CurrentlyEditing,
}

#[derive(Debug, Serialize, Default, PartialEq)]
pub enum CurrentlyEditing {
    #[default]
    Content,
    Description,
    DueString,
}

fn serialize_text_area<S>(text: &TextArea, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", text.lines().join("\n")); // Convert the TextArea to a String
    serializer.serialize_str(&s)
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DurationUnit {
    #[default]
    Minute,
    Day,
}

impl<'a> NewTask<'a> {
    pub fn new(project_id: String, parent_id: Option<String>) -> Self {
        NewTask {
            project_id: Some(project_id),
            parent_id,
            ..Default::default()
        }
    }

    pub fn get_json(&self) -> serde_json::Value {
        let task_string = serde_json::to_string(self).unwrap();
        serde_json::from_str(&task_string).unwrap()
    }
}
