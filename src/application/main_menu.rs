use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Widget},
};

use crate::models::settings::Settings;

use super::{
    app::{KeyPressResult, Mode, RemoveFromStack, Screen},
    main_menu_font::MainMenuFont,
    theme::Theme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainMenu {
    settings: Settings,
}

impl MainMenu {
    pub fn new(settings: Settings) -> Self {
        MainMenu { settings }
    }

    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                KeyPressResult(Screen::Data, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                KeyPressResult(Screen::None, Mode::Quit, RemoveFromStack(true))
            }
            _ => KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(false)),
        })
    }
    pub fn draw_keybinds(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        let main_menu_font = MainMenuFont::new(area.width, area.height, self.settings);
        let x = main_menu_font.logo.offset
            + (area.width as i16 / 4
                + (area.width as i16 / 2 - main_menu_font.logo.width as i16) / 2)
                as u16;
        let logo_area = Rect::new(
            x,
            area.height / 4 - main_menu_font.logo.height,
            main_menu_font.logo.width,
            main_menu_font.logo.height,
        );
        Text::raw(main_menu_font.logo.text)
            .style(theme.logo)
            .render(logo_area, buf);
        let largest_line =
            main_menu_font.keybinding.longest_description + main_menu_font.keybinding.width + 2;
        let x = (area.width as i16 / 4 + (area.width as i16 / 2 - largest_line as i16) / 2) as u16;
        let keybind_block_area = Rect::new(
            x,
            area.height / 4,
            largest_line + 4,
            main_menu_font.keybinding.total_height + 2,
        );
        Block::new()
            .borders(Borders::all())
            .style(theme.key_binding.block)
            .render(keybind_block_area, buf);
        self.draw_keybinds_lines(
            keybind_block_area,
            buf,
            main_menu_font.keybinding.key_desc_map,
            main_menu_font.keybinding.width,
            main_menu_font.keybinding.height,
            main_menu_font.keybinding.vertical_gap,
            theme,
        );
    }
    fn draw_keybinds_lines(
        &self,
        area: Rect,
        buf: &mut Buffer,
        keybind_strings: Vec<(String, String)>,
        keybind_width: u16,
        keybind_height: u16,
        gap_size: u16,
        theme: Theme,
    ) {
        let mut key_area = area.clone();
        key_area.x += 1;
        key_area.y += 1;
        key_area.width -= 2;
        key_area.height = keybind_height + gap_size;
        for (i, (key, desc)) in keybind_strings.iter().enumerate() {
            let mut key_area = key_area.clone();
            key_area.y += (keybind_height + gap_size) * i as u16;

            Text::raw("")
                .style(theme.key_binding.key)
                .render(key_area, buf);
            key_area.x += 1;
            key_area.width -= 1;
            Text::raw(key.to_string())
                .style(theme.key_binding.key)
                .render(key_area, buf);
            key_area.x += 1;
            key_area.width -= 1;
            Text::raw("")
                .style(theme.key_binding.key)
                .render(key_area, buf);
            key_area.x += keybind_width;
            key_area.width -= keybind_width;

            Text::raw("")
                .style(theme.key_binding.description)
                .render(key_area, buf);
            key_area.x += 2;
            key_area.width -= 2;
            Text::raw(desc.to_string())
                .style(theme.key_binding.description)
                .render(key_area, buf);
        }
    }
}

impl Widget for MainMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::new(self.settings.theme);

        self.draw_keybinds(area, buf, theme);
    }
}
