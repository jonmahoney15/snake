use std::{io, time::Duration};

use crate::{board::Board, food::Food, snake::{Direction, Snake}};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::Rng;
use ratatui::Frame;


pub struct Game {
    running: bool,
    snake: Snake,
    board: Board,
    food: Food,
    fps: u64,
    score: u8
}

impl Game {

    pub fn new() -> Self {

        let mut rng = rand::rng();

        let x = 50;
        let y = 0;

        let board = Board::new(x, y, 4, 8);
        let (x_min, x_max) = board.get_inner_x_pair();
        let (y_min, y_max) = board.get_inner_y_pair();

        let rand_x: u16 = rng.random_range(x_min..x_max);
        let rand_y: u16 = rng.random_range(y_min..y_max);

        let food = Food::new(rand_x, rand_y, x_min, x_max, y_min, y_max);

        Self {
            running: true,
            snake: Snake::new(x+10, y+10, Direction::RIGHT),
            board,
            food,
            fps: 150,
            score:0
        }
    }

    pub fn update(&mut self) {

        if self.snake.collides_with_body() {
            self.running = false;
            return;
        }

        if self.board.is_out_of_bounds(self.snake.x, self.snake.y) {
            self.running = false;
            return;
        }

        self.snake.update_position();
        let _ = self.handle_events();
        self.snake_eats_food();
    }

    pub fn render(&self, frame: &mut Frame) {
        self.board.render(frame, self.score);
        self.food.render(frame);
        self.snake.render(frame);
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Some(key_event) = self.poll_key_press(Duration::from_millis(self.fps))? {
            self.handle_key_event(key_event);
        }
        Ok(())
    }

    fn poll_key_press(&self, timeout: Duration) -> io::Result<Option<KeyEvent>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Ok(Some(key_event)),
            _ => Ok(None)
        }
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

    fn snake_eats_food(&mut self) {
        if self.snake.x == self.food.x && self.snake.y == self.food.y {
            self.score += 1;
            self.snake.add_to_body();
            self.food.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_running_running_true() {
        let game = Game::new();
        assert_eq!(game.is_running(), true);
    }

    #[test]
    fn test_is_running_exit_false() {
        let mut game = Game::new();
        game.exit();
        assert_eq!(game.is_running(), false);
    }
}
