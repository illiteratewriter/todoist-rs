use ratatui::widgets::ListState;
use tui_textarea::TextArea;

#[derive(Debug, Default, Clone)]
pub struct TaskEdit<'a> {
    pub content: TextArea<'a>,
    pub description: TextArea<'a>,
    pub currently_editing: CurrentlyEditing,
    pub children: Vec<usize>,
    pub children_list_state: ListState,
}

impl<'a> TaskEdit<'a> {
    pub fn next(&mut self) {
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
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum CurrentlyEditing {
    #[default]
    Content,
    Description,
    ChildTasks
}
