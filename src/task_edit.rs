use ratatui::{
    style::{Modifier, Style},
    widgets::ListState,
};
use tui_textarea::TextArea;

#[derive(Debug, Default, Clone)]
pub struct TaskEdit<'a> {
    pub content: TextArea<'a>,
    pub description: TextArea<'a>,
    pub due_string: TextArea<'a>,
    pub currently_editing: CurrentlyEditing,
    pub children: Vec<usize>,
    pub children_list_state: ListState,
    pub current_task_index: usize,
}

impl<'a> TaskEdit<'a> {
    pub fn new(
        content: String,
        description: String,
        due_string: String,
        children: Vec<usize>,
        current_task_index: usize,
        currently_editing: CurrentlyEditing,
    ) -> Self {
        let mut task_edit = TaskEdit {
            content: TextArea::from(vec![content]),
            description: TextArea::from(vec![description]),
            due_string: TextArea::from(vec![due_string]),
            currently_editing,
            children,
            children_list_state: ListState::default(),
            current_task_index,
        };

        // Automatically update cursor styles
        task_edit.update_cursor_styles();

        task_edit
    }

    pub fn next(&mut self) {
        if self.children.len() == 0 {
            self.children_list_state.select(None);
            return;
        }
        let i = match self.children_list_state.selected() {
            Some(i) => {
                if i >= self.children.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.children_list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.children.len() == 0 {
            self.children_list_state.select(None);
            return;
        }
        let i = match self.children_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.children.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.children_list_state.select(Some(i));
    }

    pub fn update_cursor_styles(&mut self) {
        let active_style = Style::default().add_modifier(Modifier::REVERSED);
        let default_style = Style::default();
    
        self.content.set_cursor_style(default_style);
        self.description.set_cursor_style(default_style);
        self.due_string.set_cursor_style(default_style);
    
        match self.currently_editing {
            CurrentlyEditing::Content => self.content.set_cursor_style(active_style),
            CurrentlyEditing::Description => self.description.set_cursor_style(active_style),
            CurrentlyEditing::DueString => self.due_string.set_cursor_style(active_style),
            CurrentlyEditing::ChildTasks => {} 
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum CurrentlyEditing {
    #[default]
    Content,
    Description,
    DueString,
    ChildTasks,
}
