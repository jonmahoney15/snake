use crate::{
    board::Board,
    food::Food,
    snake::{Direction, Snake},
};

use crossterm::event::{KeyCode, KeyEvent};
use rand::Rng;
use ratatui::Frame;

pub struct Game {
    running: bool,
    snake: Snake,
    board: Board,
    food: Food,
    pub score: u8,
}

impl Game {
    pub fn default() -> Game {
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
            snake: Snake::new(x + 10, y + 10, Direction::Right),
            board,
            food,
            score: 0,
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
        self.snake_eats_food();
    }

    pub fn render(&self, frame: &mut Frame) {
        self.board.render(frame, self.score);
        self.food.render(frame);
        self.snake.render(frame);
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.snake.change_direction(Direction::Left),
            KeyCode::Right => self.snake.change_direction(Direction::Right),
            KeyCode::Up => self.snake.change_direction(Direction::Up),
            KeyCode::Down => self.snake.change_direction(Direction::Down),
            _ => {}
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.running
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
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_is_running_running_true() {
//         let game = Game::new(None);
//         assert_eq!(game.is_running(), true);
//     }
//
//     #[test]
//     fn test_is_running_exit_false() {
//         let mut game = Game::new(None);
//         game.exit();
//         assert_eq!(game.is_running(), false);
//     }
// }
