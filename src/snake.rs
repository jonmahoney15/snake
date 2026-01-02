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
}

impl Snake {
    pub fn new(x: u16, y: u16, direction: Direction) -> Snake {
        Snake { x, y, direction }
    }

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::LEFT => self.x = self.x - 1,
            Direction::RIGHT => self.x = self.x + 1,
            Direction::UP => self.y = self.y - 1,
            Direction::DOWN => self.y = self.y + 1,
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

    pub fn render(&self, frame: &mut Frame) {
            let square_area = Rect {
                x: self.x,
                y: self.y,
                width: 4,
                height: 4,
            };

            let solid_square = Block::default()
                .style(Style::default().bg(Color::Green))
                .borders(Borders::NONE);

            frame.render_widget(solid_square, square_area);
    }
}
