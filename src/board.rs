use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub struct Board {
    x: u16,
    y: u16,
    cell_size: u16,
    grid_size: u16,
}

impl Board {
    pub fn new(x: u16, y: u16, cell_size: u16, grid_size: u16) -> Board {
        Board {
            x,
            y,
            cell_size,
            grid_size,
        }
    }

    pub fn render(&self, f: &mut Frame, score: u8) {
        let score_title = format!("Score: {}", score);

        let square = Rect {
            x: self.x,
            y: self.y,
            width: self.cell_size * (self.grid_size * 2),
            height: self.cell_size * self.grid_size,
        };

        let border = Block::default()
            .title(score_title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red));

        f.render_widget(border, square);
    }

    pub fn get_inner_x_pair(&self) -> (u16, u16) {
        let area = Rect {
            x: self.x,
            y: self.y,
            width: self.cell_size * (self.grid_size * 2),
            height: self.cell_size * self.grid_size,
        };

        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        (inner.x, inner.x + inner.width)
    }

    pub fn get_inner_y_pair(&self) -> (u16, u16) {
        let area = Rect {
            x: self.x,
            y: self.y,
            width: self.cell_size * (self.grid_size * 2),
            height: self.cell_size * self.grid_size,
        };

        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        (inner.y, inner.y + inner.height)
    }

    pub fn is_out_of_bounds(&self, x: u16, y: u16) -> bool {
        let inner = self.inner_rect();

        x < inner.x || x >= inner.x + inner.width || y < inner.y || y >= inner.y + inner.height
    }

    fn inner_rect(&self) -> Rect {
        let area = Rect {
            x: self.x,
            y: self.y,
            width: self.cell_size * (self.grid_size * 2),
            height: self.cell_size * self.grid_size,
        };

        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        })
    }
}
