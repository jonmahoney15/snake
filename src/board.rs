use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders}, Frame};

pub struct Board {
    x: u16,
    y: u16,
    cell_size: u16,
    grid_size: u16
}

impl Board {

    pub fn new(x: u16, y: u16, cell_size: u16, grid_size: u16) -> Board {
        Board {
            x, y, cell_size, grid_size
        }
    }

    pub fn render(&self, f: &mut Frame, score: u8) {

        let score_title = format!("Score: {}", score);

        let square = Rect {
            x: self.x,
            y: self.y,
            width: self.cell_size * (self.grid_size * 2) as u16,
            height: self.cell_size * self.grid_size as u16,
        };

        let block = Block::default()
            .title(score_title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red));

        f.render_widget(block, square);
    }
}
