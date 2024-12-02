use chrono::Duration;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    infra::repositories::focus_session_repository::FocusSessionRepository,
    models::{focus_session::FocusSession, settings::Settings},
};

use super::{
    app::{KeyPressResult, Mode, RemoveFromStack, Screen},
    theme::Theme,
};

pub struct History {
    focus_sessions: Vec<FocusSession>,
    scroll_offset: usize,
    pub max_visible: usize,
    settings: Settings,
}

const ENTRY_WIDTH: u16 = 68;

impl History {
    pub fn new(settings: Settings) -> Result<Self> {
        let focus_sessions = FocusSessionRepository::new()?.select_many()?;
        let timer = Self {
            focus_sessions,
            scroll_offset: 0,
            max_visible: 1,
            settings,
        };
        Ok(timer)
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                KeyPressResult(Screen::Data, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                self.scroll_offset -= if self.scroll_offset == 0 { 0 } else { 1 };
                KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                self.scroll_offset +=
                    if self.scroll_offset == self.focus_sessions.len() - self.max_visible {
                        0
                    } else {
                        1
                    };
                KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false)),
        })
    }
    pub fn draw_entries(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        Block::new()
            .title("history")
            .borders(Borders::ALL)
            .style(theme.history.border)
            .render(area, buf);

        let visible_sessions = self
            .focus_sessions
            .iter()
            .rev()
            .skip(self.scroll_offset)
            .take(self.max_visible);

        let mut y_offset = 0;
        for focus_session in visible_sessions {
            let date = focus_session.start.format("%Y-%m-%d %H:%M");
            let focus_minutes = focus_session.focus_seconds / 60;
            let break_minutes = focus_session.break_seconds / 60;
            let end_date = (focus_session.start
                + Duration::seconds(focus_session.focus_seconds as i64)
                + Duration::seconds(focus_session.break_seconds as i64))
            .format("%H:%M");
            let id = focus_session.id;
            let focus_percent =
                (focus_minutes as f64 / (break_minutes as f64 + focus_minutes as f64) * 100_f64)
                    .round();

            let content = Paragraph::new(format!(
                "{date}-{end_date},\nFocused for: {focus_minutes} min  In Break for: {break_minutes} min  Time in Focus: {focus_percent}%"
            ))
            .block(
                Block::default()
                    .title(format!("Session {id}"))
                    .borders(Borders::ALL),
            ).style(theme.history.style);

            let entry_area = Rect::new(area.x + 1, area.y + y_offset + 1, area.width - 2, 4);

            content.render(entry_area, buf);

            y_offset += 4;
        }
    }

    pub fn draw_keybinds(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        let keys = [
            ("Quit", "Q"),
            ("Up", "K/↑"),
            ("Down", "J/↓"),
            ("Graph", "G"),
        ];

        let spans: Vec<Span> = keys
            .iter()
            .flat_map(|(desc, key)| {
                let key = Span::styled(format!(" {key} "), theme.key_binding.key);
                let desc = Span::styled(format!(" {desc} "), theme.key_binding.description);
                [key, desc]
            })
            .collect();
        Line::from(spans)
            .centered()
            .style(theme.key_binding.surrounding)
            .render(area, buf);
    }

    pub fn update_max_visible(&mut self, height: usize) {
        self.max_visible = ((height) / 4) as usize - 2;
        if self.max_visible > self.focus_sessions.len() {
            self.scroll_offset = 0
        } else if self.scroll_offset > self.focus_sessions.len() - self.max_visible {
            self.scroll_offset = self.focus_sessions.len() - self.max_visible;
        };
    }
}
impl Widget for &History {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::new(self.settings.theme);

        let history_area = Rect::new(
            (area.width - ENTRY_WIDTH + 2) / 2,
            area.y + 1,
            ENTRY_WIDTH + 2,
            area.height - 6,
        );
        let keybinds_area = Rect::new(
            history_area.x,
            history_area.y + history_area.height,
            history_area.width,
            3,
        );
        self.draw_entries(history_area, buf, theme);
        self.draw_keybinds(keybinds_area, buf, theme);
    }
}
