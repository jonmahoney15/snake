mod app;
mod board;
mod food;
mod game;
mod menu;
mod score_board;
mod snake;

use app::App;
use menu::Menu;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut terminal = ratatui::init();

    let mut app = App::default();

    while app.is_running() {
        terminal.draw(|f| {
            app.render(f);
        })?;

        app.update();
    }

    ratatui::restore();

    Ok(())
}
