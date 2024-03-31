use ratatui::{
  prelude::*,
  widgets::{
      block::{Position, Title}, Block, Borders, Clear, Paragraph, Row, Table
  },
};

use crate::{tui::utils, App};

pub fn editor(f: &mut Frame, app: &mut App) {
  let area = utils::centered_rect(60, 20, f.size());

  let inner_area = area.inner(&Margin {
    vertical: 1,
    horizontal: 2,
  });

  let vertical_split = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
      Constraint::Length(5),
      Constraint::Min(1)
    ])
    .split(inner_area);

  

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
  .constraints(vec![
      Constraint::Percentage(25),
      Constraint::Percentage(75),
  ])
  .split(vertical_split[1]);

f.render_widget(Clear, area); //this clears out the background

  f.render_widget(
    Paragraph::new("inner 0")
        .block(Block::new().borders(Borders::ALL)),
    inner_layout[0]);
f.render_widget(
    Paragraph::new("inner 1")
        .block(Block::new().borders(Borders::ALL)),
    inner_layout[1]);


  f.render_widget(block, area);
}
