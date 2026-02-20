use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::Frame;

use crate::{Menu, game::Game, score_board::Score};

#[derive(PartialEq, Eq, Debug)]
pub enum Screen {
    Menu,
    Playing,
    Score,
    Exit,
}

pub struct App {
    screen: Screen,
    menu: Menu,
    game: Game,
    speed: u64,
    running: bool,
}

impl App {
    pub fn default() -> App {
        App {
            screen: Screen::Menu,
            menu: Menu::default(),
            game: Game::default(),
            running: true,
            speed: 150,
        }
    }

    pub fn update(&mut self) {
        self.game.update();

        if self.screen == Screen::Playing && !self.game.is_running() {
            self.screen = Screen::Score;
        }

        let _ = self.handle_events();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Menu => self.menu.render(frame),
            Screen::Playing => self.game.render(frame),
            Screen::Score => Score::render(frame, self.game.score),
            Screen::Exit => self.exit(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn exit(&mut self) {
        self.running = false;
        self.screen = Screen::Exit;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Some(key_event) = self.poll_key_press(Duration::from_millis(self.speed))? {
            self.handle_input(key_event);
        }
        Ok(())
    }

    fn poll_key_press(&self, timeout: Duration) -> io::Result<Option<KeyEvent>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }

        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Ok(Some(key_event)),
            _ => Ok(None),
        }
    }

    fn handle_input(&mut self, key: KeyEvent) {
        match self.screen {
            Screen::Menu => {
                self.screen = self.menu.handle_key_event(key);

                if self.screen == Screen::Playing {
                    self.game = Game::default();
                    self.speed = self.menu.get_difficulty();
                }
            }
            Screen::Playing => self.game.handle_input(key),
            Screen::Exit => self.exit(),
            Screen::Score => {
                let mut score = Score;
                self.screen = score.handle_key_event(key);
            }
        }
    }
}
