use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Padding},
};

use crate::app::Screen;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,
    Hard,
}

pub struct Menu {
    selected: usize,
    items: Vec<&'static str>,
}

impl Menu {
    pub fn default() -> Menu {
        Menu {
            selected: 0,
            items: vec!["EASY", "HARD", "EXIT"],
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::default()
            .title("Select Difficulty")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(2));

        let mut state = ListState::default();
        state.select(Some(self.selected));
        let items: Vec<ListItem> = self.items.iter().map(|i| ListItem::new(*i)).collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut state);
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Screen {
        match key_event.code {
            KeyCode::Up => {
                self.prev();
                return Screen::Menu;
            }
            KeyCode::Down => {
                self.next();
                return Screen::Menu;
            }
            KeyCode::Esc => return Screen::Exit,
            KeyCode::Enter => return Screen::Playing,
            _ => return Screen::Menu,
        }
    }
}
