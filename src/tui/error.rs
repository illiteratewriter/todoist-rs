use ratatui::{prelude::*, widgets::Borders};

use crate::App;

pub fn render_error_modal(f: &mut Frame, app: &mut App) {
    let error_message = match &app.error_message {
        Some(msg) => msg,
        None => return,
    };

    let area = f.area();

    // Create a slightly smaller area for the modal
    let modal_width = std::cmp::min(60, area.width.saturating_sub(10));
    let modal_height = std::cmp::min(10, area.height.saturating_sub(10));

    let modal_area = ratatui::layout::Rect::new(
        (area.width - modal_width) / 2,
        (area.height - modal_height) / 2,
        modal_width,
        modal_height,
    );

    let close_error = Line::from(vec![
        " Press ".into(),
        "Esc".blue().bold(),
        " to close ".into(),
    ]);

    let block = ratatui::widgets::Block::default()
        .title(" Error ")
        .title_bottom(close_error.centered())
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Red));

    // Create the paragraph with the error message
    let text = Text::from(vec![Line::from(vec![
        Span::raw("\n"),
        Span::styled(error_message, Style::default()),
    ])]);

    let paragraph = ratatui::widgets::Paragraph::new(text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(ratatui::widgets::Clear, modal_area);
    f.render_widget(paragraph, modal_area);
}
