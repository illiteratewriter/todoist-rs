use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, Paragraph,
    },
};

use crate::{tui::utils, App};

pub fn editor(f: &mut Frame, app: &mut App) {
    let area = utils::centered_rect(60, 40, f.size());

    f.render_widget(Clear, area); //this clears out the background

    let inner_area = area.inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    let vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(inner_area);

    app.task_edit
        .content
        .set_block(Block::default().borders(Borders::ALL).title("Task"));

    app.task_edit
        .description
        .set_block(Block::default().borders(Borders::ALL).title("Description"));

    // if event::poll(std::time::Duration::from_millis(16)).unwrap() {

    //   match crossterm::event::read().unwrap().into() {
    //       Input { key: Key::Esc, .. } => {
    //           panic!("wowo");
    //       }
    //       input => {
    //           // println!("SOMLEKJLASKDFJNLSKDNFJSDLKJDNFLSDKJNFLSDNJFS>JKLDSNFD {:?}", input);
    //           app.text_area.input(input);
    //       }
    //   }
    // }

    let content = app.task_edit.content.widget();
    let description = app.task_edit.description.widget();

    f.render_widget(content, vertical_split[0]);
    f.render_widget(description, vertical_split[1]);

    let close_modal_desc = Title::from(Line::from(vec![
        " To close, press ".into(),
        "Esc".blue().bold(),
        " ".into(),
    ]));

    let block = Block::default()
        .title(" Edit task ")
        .title(
            close_modal_desc
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(vertical_split[2]);

    f.render_widget(
        Paragraph::new("inner 0").block(Block::new().borders(Borders::ALL)),
        inner_layout[0],
    );
    f.render_widget(
        Paragraph::new("inner 1").block(Block::new().borders(Borders::ALL)),
        inner_layout[1],
    );

    f.render_widget(block, area);
}
