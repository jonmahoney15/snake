use std::{io, time::Duration};

use crate::{board::Board, snake::{Direction, Snake}};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;


pub struct Game {
    running: bool,
    snake: Snake,
    board: Board
}

impl Game {
    pub fn new() -> Self {
        let x = 50;
        let y = 0;

        let board = Board::new(x, y, 4, 8);

        Self {
            running: true,
            snake: Snake::new(x+10, y+10, Direction::RIGHT),
            board
        }
    }

    pub fn update(&mut self) {
        self.snake.update_position();
        let _ = self.handle_events();
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.snake.change_direction(Direction::LEFT),
            KeyCode::Right => self.snake.change_direction(Direction::RIGHT),
            KeyCode::Up => self.snake.change_direction(Direction::UP),
            KeyCode::Down => self.snake.change_direction(Direction::DOWN),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn render(&self, frame: &mut Frame) {
        self.board.render(frame);
        self.snake.render(frame);
    }
}
