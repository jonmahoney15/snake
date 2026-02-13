use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::Screen;

pub struct Score;

impl Score {
    pub fn render(frame: &mut Frame, score: u8) {
        let area = frame.area();

        let text = vec![
            Line::from(Span::styled(
                "GAME OVER",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::raw(format!("Score: {}", score))),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to return to menu",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph =
            Paragraph::new(text).block(Block::default().title("Snake").borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Screen {
        match key_event.code {
            KeyCode::Enter => return Screen::Exit,
            _ => return Screen::Score,
        }
    }
}
