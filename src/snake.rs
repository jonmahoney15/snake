use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    pub x: u16,
    pub y: u16,
    direction: Direction,
    body: Vec<(u16, u16)>,
}

impl Snake {
    pub fn new(x: u16, y: u16, direction: Direction) -> Snake {
        Snake {
            x,
            y,
            direction,
            body: vec![],
        }
    }

    pub fn update_position(&mut self) {
        if !self.body.is_empty() {
            self.body.insert(0, (self.x, self.y));
            self.body.pop();
        }

        match self.direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left;
                }
            }
            Direction::Right => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right;
                }
            }
            Direction::Up => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up;
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down;
                }
            }
        }
    }

    pub fn collides_with_body(&self) -> bool {
        let next_head = self.next_head_position();

        self.body.contains(&next_head)
    }

    pub fn render(&self, frame: &mut Frame) {
        let square_area = Rect {
            x: self.x,
            y: self.y,
            width: 1,
            height: 1,
        };

        let solid_square = Block::default()
            .style(Style::default().bg(Color::Green))
            .borders(Borders::NONE);

        frame.render_widget(solid_square, square_area);

        for segment in &self.body {
            let square_area = Rect {
                x: segment.0,
                y: segment.1,
                width: 1,
                height: 1,
            };

            let solid_square = Block::default()
                .style(Style::default().bg(Color::Green))
                .borders(Borders::NONE);

            frame.render_widget(solid_square, square_area);
        }
    }

    pub fn add_to_body(&mut self) {
        if let Some(&last) = self.body.last() {
            self.body.push(last);
        } else {
            self.body.push((self.x, self.y));
        }
    }

    fn next_head_position(&self) -> (u16, u16) {
        match self.direction {
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_direction_up_snake_headed_left_goes_up() {
        let mut snake = Snake::new(10, 10, Direction::Left);

        snake.change_direction(Direction::Up);

        assert_eq!(snake.direction, Direction::Up);
    }

    #[test]
    fn test_change_direction_up_snake_headed_down_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::Down);

        snake.change_direction(Direction::Up);

        assert_eq!(snake.direction, Direction::Down);
    }

    #[test]
    fn test_change_direction_down_snake_headed_left_goes_down() {
        let mut snake = Snake::new(10, 10, Direction::Left);

        snake.change_direction(Direction::Down);

        assert_eq!(snake.direction, Direction::Down);
    }

    #[test]
    fn test_change_direction_down_snake_headed_up_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::Up);

        snake.change_direction(Direction::Down);

        assert_eq!(snake.direction, Direction::Up);
    }

    #[test]
    fn test_change_direction_left_snake_headed_down_goes_left() {
        let mut snake = Snake::new(10, 10, Direction::Down);

        snake.change_direction(Direction::Left);

        assert_eq!(snake.direction, Direction::Left);
    }

    #[test]
    fn test_change_direction_left_snake_headed_right_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::Right);

        snake.change_direction(Direction::Left);

        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn test_change_direction_right_snake_headed_down_goes_right() {
        let mut snake = Snake::new(10, 10, Direction::Down);

        snake.change_direction(Direction::Right);

        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn test_change_direction_right_snake_headed_left_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::Left);

        snake.change_direction(Direction::Right);

        assert_eq!(snake.direction, Direction::Left);
    }
}
