# Todoist Terminal Client Usage Guide (HOWTO)

This project is designed to be a keyboard-driven interface for managing your Todoist tasks directly from your terminal. Use the shortcuts below to navigate and manage your tasks efficiently.

## 🌐 Global Shortcuts
These shortcuts and filters can be used anywhere in the application.

- `h`: Open / close the help menu
- `q`: Quit the application
- `Tab`: Switch focus between the Projects and Tasks panes
- **Task Filters & Sorting:**
  - `t`: View Today's tasks
  - `o`: View Overdue tasks
  - `a`: View All tasks
  - `p`: Sort tasks by priority
  - `d`: Sort tasks by date

---

## 📝 Tasks Pane
Shortcuts available when the Tasks list is focused.

- `j` or `↓`: Move to the next task
- `k` or `↑`: Move to the previous task
- `Enter`: Open the Task Editor for the highlighted task
- `x`: Mark the highlighted task as done (Close task)
- `d`: Delete the highlighted task
- `n`: Add a new task to the currently selected project

---

## 📁 Projects Pane
Shortcuts available when the Projects list is focused.

- `j` or `↓`: Move to the next project
- `k` or `↑`: Move to the previous project
- `+` or `n`: Add a new task to the highlighted project

---

## ✏️ Task Editor
Used for editing task details or managing sub-tasks.

- `Tab`: Cycle through input fields (Content -> Description -> Due Date -> Child Tasks)
- `Shift + Tab`: Cycle to previous input field (@Syzseisus)
- `Alt + Enter` or `Ctrl + n`: Insert new line in Description (@Syzseisus)
- `Esc`: Cancel changes and close the editor
- `Enter`: Save changes to the server and close. (If a child task is highlighted, it opens that child task instead)
- `n` (when Child Tasks list is focused): Create a new sub-task for the current task
- Use `j`, `k`, `↓`, `↑` to navigate the child tasks list.

---

## ➕ New Task Prompt
Used when creating a new task.

- `Tab`: Cycle through input fields (Content -> Description -> Due Date)
- `Shift + Tab`: Cycle to previous input field (@Syzseisus)
- `Alt + Enter` or `Ctrl + n`: Insert new line in Description (@Syzseisus)
- `Esc`: Cancel task creation and close the prompt
- `Enter`: Submit and create the new task on the server

---

## Contributors
This guide was authored and updated by @syzseisus.
