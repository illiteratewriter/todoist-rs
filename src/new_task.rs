struct NewTask {
    content: String,
    description: Option<String>,
    project_id: Option<String>,
    section_id: Option<String>,
    parent_id: Option<String>,
    order: Option<usize>,
    labels: Option<Vec<String>>,
    priority: Option<u8>,
    due_string: Option<String>,
    due_date: Option<String>,
    due_datetime: Option<String>,
    due_lang: Option<String>,
    assignee_id: Option<String>,
    duration: Option<usize>,
    duration_unit: Option<DurationUnit>,
}

enum DurationUnit {
    Minute,
    Day,
}
