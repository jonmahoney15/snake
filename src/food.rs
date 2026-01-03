use rand::Rng;
use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders}};

pub struct Food {
    pub x: u16,
    pub y: u16
}

impl Food {
    pub fn new(rand_x: u16, rand_y: u16) -> Food {
        Food { x: rand_x, y: rand_y }
    }

    pub fn update(&mut self) {
        let mut rng = rand::rng();

        let rand_x: u16 = rng.random_range(51..60);
        let rand_y: u16 = rng.random_range(1..10);

        self.x = rand_x;
        self.y = rand_y;
    }

    pub fn render(&self, frame: &mut ratatui::Frame<'_>) {
        let square_area = Rect {
            x: self.x,
            y: self.y,
            width: 4,
            height: 2,
        };

        let solid_square = Block::default()
            .style(Style::default().bg(Color::Green))
            .borders(Borders::NONE);

        frame.render_widget(solid_square, square_area);
    }
}
