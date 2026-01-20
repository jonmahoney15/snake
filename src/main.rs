mod board;
mod food;
mod game;
mod snake;

use game::Game;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut terminal = ratatui::init();

    let mut game = Game::new();

    while game.is_running() {
        terminal.draw(|f| {
            game.render(f);
        })?;

        game.update();
    }

    ratatui::restore();

    Ok(())
}
