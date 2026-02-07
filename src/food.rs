use rand::Rng;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub struct Food {
    pub x: u16,
    pub y: u16,
    x_min: u16,
    x_max: u16,
    y_min: u16,
    y_max: u16,
}

impl Food {
    pub fn new(rand_x: u16, rand_y: u16, x_min: u16, x_max: u16, y_min: u16, y_max: u16) -> Food {
        Food {
            x: rand_x,
            y: rand_y,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::rng();

        let rand_x: u16 = rng.random_range(self.x_min..self.x_max);
        let rand_y: u16 = rng.random_range(self.y_min..self.y_max);

        self.x = rand_x;
        self.y = rand_y;
    }

    pub fn render(&self, frame: &mut ratatui::Frame) {
        let square_area = Rect {
            x: self.x,
            y: self.y,
            width: 1,
            height: 1,
        };

        let solid_square = Block::default()
            .style(Style::default().bg(Color::Blue))
            .borders(Borders::NONE);

        frame.render_widget(solid_square, square_area);
    }
}
