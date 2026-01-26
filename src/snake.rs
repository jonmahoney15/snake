use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders}, Frame};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub struct Snake {
    pub x: u16,
    pub y: u16,
    direction: Direction,
    body: Vec<(u16, u16)>
}

impl Snake {
    pub fn new(x: u16, y: u16, direction: Direction) -> Snake {
        Snake { x, y, direction, body: vec![] }
    }

    pub fn update_position(&mut self) {

        if !self.body.is_empty() {
            self.body.insert(0, (self.x, self.y));
            self.body.pop();
        }

        match self.direction {
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {

        match direction {
            Direction::LEFT => {
                if self.direction != Direction::RIGHT {
                    self.direction = Direction::LEFT;
                }
            }
            Direction::RIGHT => {
                if self.direction != Direction::LEFT {
                    self.direction = Direction::RIGHT;
                }
            },
            Direction::UP => {
                if self.direction != Direction::DOWN {
                    self.direction = Direction::UP;
                }
            },
            Direction::DOWN => {
                if self.direction != Direction::UP {
                    self.direction = Direction::DOWN;
                }
            },

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
            Direction::LEFT  => (self.x - 1, self.y),
            Direction::RIGHT => (self.x + 1, self.y),
            Direction::UP    => (self.x, self.y - 1),
            Direction::DOWN  => (self.x, self.y + 1),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_direction_up_snake_headed_left_goes_up() {
        let mut snake = Snake::new(10, 10, Direction::LEFT);

        snake.change_direction(Direction::UP);

        assert_eq!(snake.direction, Direction::UP);
    }

    #[test]
    fn test_change_direction_up_snake_headed_down_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::DOWN);

        snake.change_direction(Direction::UP);

        assert_eq!(snake.direction, Direction::DOWN);
    }

    #[test]
    fn test_change_direction_down_snake_headed_left_goes_down() {
        let mut snake = Snake::new(10, 10, Direction::LEFT);

        snake.change_direction(Direction::DOWN);

        assert_eq!(snake.direction, Direction::DOWN);
    }

    #[test]
    fn test_change_direction_down_snake_headed_up_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::UP);

        snake.change_direction(Direction::DOWN);

        assert_eq!(snake.direction, Direction::UP);
    }

    #[test]
    fn test_change_direction_left_snake_headed_down_goes_left() {
        let mut snake = Snake::new(10, 10, Direction::DOWN);

        snake.change_direction(Direction::LEFT);

        assert_eq!(snake.direction, Direction::LEFT);
    }

    #[test]
    fn test_change_direction_left_snake_headed_right_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::RIGHT);

        snake.change_direction(Direction::LEFT);

        assert_eq!(snake.direction, Direction::RIGHT);
    }

    #[test]
    fn test_change_direction_right_snake_headed_down_goes_right() {
        let mut snake = Snake::new(10, 10, Direction::DOWN);

        snake.change_direction(Direction::RIGHT);

        assert_eq!(snake.direction, Direction::RIGHT);
    }

    #[test]
    fn test_change_direction_right_snake_headed_left_doesnt_change() {
        let mut snake = Snake::new(10, 10, Direction::LEFT);

        snake.change_direction(Direction::RIGHT);

        assert_eq!(snake.direction, Direction::LEFT);
    }
}
