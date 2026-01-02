mod snake;

use snake::{Direction, Snake};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::Rect,
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders},
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = Game::new();
    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();

    while game.running {
        terminal.draw(|f| {
            render_border(f);
            game.snake.render(f);
        })?;

        if last_tick.elapsed() >= tick_rate {
            game.update();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn render_border(f: &mut Frame) {
    let cell_size = 4;
    let grid_size = 8;

    let square = Rect {
        x: 50,
        y: 0,
        width: cell_size*(grid_size*2) as u16,
        height: cell_size*grid_size as u16,
    };
    let block = Block::default()
        .title("Snake")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));
    f.render_widget(block, square);
}

struct Game {
    running: bool,
    snake: Snake,
}

impl Game {
    fn new() -> Self {
        Self {
            running: true,
            snake: Snake::new(10, 10, Direction::RIGHT),
        }
    }

    fn update(&mut self) {
        self.snake.update_position();
        let _ = self.handle_events();
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(50))? {
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
}
