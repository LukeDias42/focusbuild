use std::collections::HashMap;

use chrono::{Days, Local, NaiveTime};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Padding, Widget},
};

use crate::{
    infra::repositories::focus_session_repository::FocusSessionRepository,
    models::{focus_session::FocusSession, settings::Settings},
};

use super::{
    app::{KeyPressResult, Mode, RemoveFromStack, Screen},
    theme::Theme,
};

pub struct Data {
    times_per_day: HashMap<i64, TotalTime>,
    scroll_offset: usize,
    pub max_visible: usize,
    total_days: usize,
    settings: Settings,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct TotalTime {
    focus_seconds: u64,
    break_seconds: u64,
}
impl TotalTime {
    fn new(focus_seconds: u64, break_seconds: u64) -> Self {
        Self {
            focus_seconds,
            break_seconds,
        }
    }
}

impl Data {
    const BAR_GAP: u16 = 1;
    const BAR_WIDTH: u16 = 5;
    const GROUP_GAP: u16 = 2;

    pub fn new(settings: Settings) -> Result<Self> {
        let focus_sessions = FocusSessionRepository::new()?.select_many()?;
        let (times_per_day, total_days) = Data::group_focus_sessions_by_date(focus_sessions);
        let timer = Self {
            times_per_day,
            scroll_offset: 0,
            max_visible: 1,
            total_days,
            settings,
        };
        Ok(timer)
    }

    fn group_focus_sessions_by_date(
        focus_sessions: Vec<FocusSession>,
    ) -> (HashMap<i64, TotalTime>, usize) {
        let mut times_per_day: HashMap<i64, TotalTime> = HashMap::new();
        let now = Local::now()
            .with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
            .unwrap();
        let mut oldest = 0;
        for focus_session in focus_sessions {
            let days_ago = now.signed_duration_since(focus_session.start).num_days();
            if days_ago > oldest {
                oldest = days_ago;
            }
            times_per_day
                .entry(days_ago)
                .and_modify(|t| {
                    t.focus_seconds += focus_session.focus_seconds;
                    t.break_seconds += focus_session.break_seconds
                })
                .or_insert(TotalTime::new(
                    focus_session.focus_seconds,
                    focus_session.break_seconds,
                ));
        }
        (times_per_day, oldest as usize + 1)
    }

    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                self.scroll_offset -= if self.scroll_offset == 0 { 0 } else { 1 };
                KeyPressResult(Screen::Data, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                self.scroll_offset += if self.max_visible > self.total_days
                    || self.scroll_offset == self.total_days - self.max_visible
                {
                    0
                } else {
                    1
                };
                KeyPressResult(Screen::Data, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::Data, Mode::Running, RemoveFromStack(false)),
        })
    }
    pub fn draw_days_graph(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        let mut barchart = BarChart::default()
            .block(
                Block::new()
                    .title("Statistics")
                    .borders(Borders::ALL)
                    .padding(Padding::new(1, 1, 1, 0))
                    .style(theme.data.block),
            )
            .bar_gap(Self::BAR_GAP)
            .bar_width(Self::BAR_WIDTH)
            .group_gap(Self::GROUP_GAP)
            .label_style(theme.logo);
        let start = self.scroll_offset as i64;
        let end = start + self.max_visible as i64;
        for bar_group in (start..end)
            .rev()
            .map(|days_ago| {
                let total_time = match self.times_per_day.get(&days_ago) {
                    Some(t) => t,
                    None => &TotalTime::new(0, 0),
                };
                let focus_bar = Bar::default()
                    .label("focus".into())
                    .value(total_time.focus_seconds)
                    .text_value((total_time.focus_seconds / 60).to_string())
                    .style(theme.data.focus_bar);
                let break_bar = Bar::default()
                    .label("break".into())
                    .value(total_time.break_seconds)
                    .text_value((total_time.break_seconds / 60).to_string())
                    .style(theme.data.break_bar);
                (days_ago, vec![focus_bar, break_bar])
            })
            .map(|(days_ago, bars)| {
                let day = Local::now()
                    .checked_sub_days(Days::new(days_ago as u64))
                    .unwrap()
                    .format("%m/%d");
                BarGroup::default()
                    .label(Line::from(format!("{day}")))
                    .bars(&bars)
            })
        {
            barchart = barchart.data(bar_group);
        }
        barchart.render(area, buf);
    }

    pub fn draw_keybinds(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        let keys = [
            ("Quit", "Q"),
            ("Left", "H/←"),
            ("Right", "L/→"),
            ("Sessions", "S"),
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

    pub fn update_max_visible(&mut self, width: usize) {
        self.max_visible = ((width - 2)
            / (2 * Self::BAR_WIDTH + 2 * Self::BAR_GAP + Self::GROUP_GAP) as usize)
            as usize;
        if self.total_days < self.max_visible {
            self.scroll_offset = 0;
        } else if self.scroll_offset > self.total_days - self.max_visible {
            self.scroll_offset = self.total_days - self.max_visible;
        };
    }
}
impl Widget for &Data {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::new(self.settings.theme);

        let graph_area = Rect::new(area.x, area.y + 1, area.width, area.height - 6);
        let keybinds_area = Rect::new(
            graph_area.x,
            graph_area.y + graph_area.height,
            graph_area.width,
            1,
        );
        self.draw_days_graph(graph_area, buf, theme);
        self.draw_keybinds(keybinds_area, buf, theme);
    }
}
