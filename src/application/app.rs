use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use color_eyre::{eyre::Context, Result};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer, layout::Rect, prelude::CrosstermBackend, widgets::Widget, Frame, Terminal,
};

use crate::{
    application::{data::Data, history::History, main_menu::MainMenu, timer::Timer},
    infra::repositories::settings_repository::SettingsRepository,
    models::settings::Settings,
};

pub struct App {
    mode: Mode,
    screen_stack: Vec<Screen>,
    pub main_menu: MainMenu,
    pub timer: Timer,
    pub history: History,
    pub data: Data,
    pub settings: Settings,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    MainMenu,
    Timer,
    History,
    Data,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Running,
    Destroy,
    Quit,
}

pub struct RemoveFromStack(pub bool);
pub struct KeyPressResult(pub Screen, pub Mode, pub RemoveFromStack);

impl App {
    pub fn new() -> Result<App> {
        let mut screen_stack: Vec<Screen> = Vec::new();
        screen_stack.push(Screen::default());
        let settings = Self::get_settings()?;
        Ok(App {
            mode: Mode::default(),
            screen_stack,
            timer: Timer::new(settings)?,
            main_menu: MainMenu::new(settings),
            history: History::new(settings)?,
            data: Data::new(settings)?,
            settings,
        })
    }

    fn get_settings() -> Result<Settings> {
        let settings_repo = SettingsRepository::new()?;
        Ok(match settings_repo.get_settings() {
            Ok(settings) => settings,
            Err(_) => settings_repo.insert_settings(Settings::default())?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        let mut ratatui_terminal = ratatui::init();
        while self.is_running() {
            let area = self.update_screen_size(&terminal)?;
            ratatui_terminal
                .draw(|frame| self.draw(frame, area))
                .wrap_err("terminal.draw")?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn update_screen_size(
        &mut self,
        terminal: &Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<Rect> {
        let terminal_size = terminal.size()?;
        let width = terminal_size.width;
        let height = terminal_size.height;
        let area = Rect::new(0, 0, width, height);

        self.history.update_max_visible(area.height as usize);
        self.data.update_max_visible(area.width as usize);

        Ok(area)
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 60.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return self.handle_key_press(key)
            }
            _ => {}
        }
        return Ok(());
    }

    fn handle_key_press(&mut self, key: KeyEvent) -> Result<()> {
        let current_screen = self.screen_stack.last().unwrap_or(&Screen::None).to_owned();
        if current_screen == Screen::None {
            self.mode = Mode::Quit;
            return Ok(());
        }
        let key_press_result = KeyPressResult(current_screen, self.mode, RemoveFromStack(false));
        let result = match current_screen {
            Screen::MainMenu => self.main_menu.handle_key_press(key)?,
            Screen::Timer => self.timer.handle_key_press(key)?,
            Screen::History => self.history.handle_key_press(key)?,
            Screen::Data => self.data.handle_key_press(key)?,
            Screen::None => key_press_result,
        };
        if result.0 != current_screen {
            if result.2 .0 {
                self.screen_stack.pop();
            }
            let new_current_screen = self.screen_stack.last().unwrap_or(&Screen::None).to_owned();
            if new_current_screen != result.0 {
                match result.0 {
                    Screen::MainMenu => self.main_menu = MainMenu::new(self.settings),
                    Screen::Timer => self.timer = Timer::new(self.settings)?,
                    Screen::History => self.history = History::new(self.settings)?,
                    Screen::Data => self.data = Data::new(self.settings)?,
                    Screen::None => {}
                }
                self.screen_stack.push(result.0);
            }
        }
        self.mode = result.1;
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        if let Some(screen) = self.screen_stack.last() {
            match screen {
                _ => frame.render_widget(self, area),
            }
        }
    }

    fn is_running(&self) -> bool {
        self.mode == Mode::Running
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(screen) = self.screen_stack.last() {
            match screen {
                Screen::MainMenu => self.main_menu.render(area, buf),
                Screen::Timer => self.timer.render(area, buf),
                Screen::History => self.history.render(area, buf),
                Screen::Data => self.data.render(area, buf),
                Screen::None => {}
            }
        }
    }
}
