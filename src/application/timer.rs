use chrono::{DateTime, Local};
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use std::time::Instant;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Widget},
};

use crate::{
    infra::repositories::focus_session_repository::FocusSessionRepository,
    models::settings::Settings,
};

use super::{
    app::{KeyPressResult, Mode, RemoveFromStack, Screen},
    theme::Theme,
    timer_font::{Digits, TimerFont},
};

pub struct Timer {
    stopwatch: Instant,
    focus_time: Vec<u64>,
    break_time: Vec<u64>,
    start_date: DateTime<Local>,
    is_focusing: bool,
    focus_session_repository: FocusSessionRepository,
    settings: Settings,
}

impl Timer {
    pub fn new(settings: Settings) -> Result<Self> {
        let timer = Self {
            stopwatch: Instant::now(),
            focus_time: Vec::new(),
            break_time: Vec::new(),
            start_date: Local::now(),
            is_focusing: true,
            focus_session_repository: FocusSessionRepository::new()?,
            settings,
        };
        Ok(timer)
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.end_focus_session()?;
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Char('b') => {
                if self.is_focusing {
                    self.focus_time.push(self.stopwatch.elapsed().as_secs());
                    self.is_focusing = false;
                    self.stopwatch = Instant::now();
                }
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('f') => {
                if !self.is_focusing {
                    self.break_time.push(self.stopwatch.elapsed().as_secs());
                    self.is_focusing = true;
                    self.stopwatch = Instant::now();
                }
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false)),
        })
    }

    fn get_number_digits(mut number: i64) -> Vec<usize> {
        let mut digits = Vec::new();
        while number != 0 {
            digits.push((number % 10) as usize);
            number /= 10;
        }
        digits.reverse();
        digits
    }

    pub fn timer_digits_size_vector(
        mut elapsed_seconds: i64,
        digits: &Digits,
    ) -> Vec<(String, u16)> {
        let mut timer_digits = Vec::new();
        if elapsed_seconds < 0 {
            timer_digits.push((digits.minus_sign.clone(), digits.minus_sign_width));
            elapsed_seconds *= -1;
        }

        let hours = elapsed_seconds / 3600;
        if hours < 10 {
            timer_digits.push((digits.digit_map[0].clone(), digits.digit_width));
            if hours == 0 {
                timer_digits.push((digits.digit_map[0].clone(), digits.digit_width));
            }
        }
        for number in Timer::get_number_digits(hours) {
            timer_digits.push((digits.digit_map[number].clone(), digits.digit_width));
        }
        timer_digits.push((digits.colon.clone(), digits.colon_width));

        let minutes = (elapsed_seconds / 60) - hours * 60;

        if minutes < 10 {
            timer_digits.push((digits.digit_map[0].clone(), digits.digit_width));
            if minutes == 0 {
                timer_digits.push((digits.digit_map[0].clone(), digits.digit_width));
            }
        }
        for number in Timer::get_number_digits(minutes) {
            timer_digits.push((digits.digit_map[number].clone(), digits.digit_width));
        }
        return timer_digits;
    }

    fn get_focus_time(&self) -> u64 {
        let previous_focus_time = self
            .focus_time
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();
        previous_focus_time
            + if self.is_focusing {
                self.stopwatch.elapsed().as_secs()
            } else {
                0
            }
    }
    fn get_break_time(&self) -> u64 {
        let previous_break_time = self
            .break_time
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .unwrap_or_default();
        previous_break_time
            + if self.is_focusing {
                0
            } else {
                self.stopwatch.elapsed().as_secs()
            }
    }

    fn end_focus_session(&self) -> Result<()> {
        let focus_seconds = self.get_focus_time();
        if focus_seconds < 60 {
            return Ok(());
        }

        self.focus_session_repository.insert_focus_session(
            self.start_date,
            focus_seconds,
            self.get_break_time(),
        )?;
        Ok(())
    }

    pub fn draw_timers(&self, area: Rect, buf: &mut Buffer, timer_font: TimerFont, theme: Theme) {
        Block::new()
            .borders(Borders::ALL)
            .padding(Padding::symmetric(2, 3))
            .style(theme.timer.title)
            .render(area, buf);

        let focus_block_area = Rect::new(
            area.x + 1,
            area.y + 1,
            timer_font.digits.stopwatch_width + 2,
            timer_font.digits.height + 2,
        );
        self.draw_timer(
            focus_block_area,
            buf,
            if self.is_focusing {
                theme.timer.active
            } else {
                theme.timer.inactive
            },
            self.get_focus_time() as i64,
            "Focus",
            &timer_font,
        );

        let break_block_area = Rect::new(
            focus_block_area.x + focus_block_area.width,
            focus_block_area.y,
            focus_block_area.width,
            focus_block_area.height,
        );

        self.draw_timer(
            break_block_area,
            buf,
            if self.is_focusing {
                theme.timer.inactive
            } else {
                theme.timer.active
            },
            self.get_break_time() as i64,
            "Break",
            &timer_font,
        );

        let balance_block_area = Rect::new(
            break_block_area.x + break_block_area.width,
            break_block_area.y,
            timer_font.digits.balance_width + 2,
            timer_font.digits.height + 2,
        );
        let balance_seconds = self.get_focus_time() as i64 / 3 - self.get_break_time() as i64;
        self.draw_timer(
            balance_block_area,
            buf,
            theme.timer.inactive,
            balance_seconds,
            "Balance",
            &timer_font,
        );
    }
    fn draw_timer(
        &self,
        area: Rect,
        buf: &mut Buffer,
        style: Style,
        seconds: i64,
        title: &str,
        timer_font: &TimerFont,
    ) {
        Block::new()
            .borders(Borders::ALL)
            .title(title)
            .style(style)
            .render(area, buf);
        let focus_digits = Timer::timer_digits_size_vector(seconds, &timer_font.digits);
        let mut current_x = area.x + 1 + timer_font.digits.gap;
        for (digit, size) in focus_digits {
            let focus_digit_area = Rect::new(
                current_x,
                area.y + 1,
                size.clone(),
                timer_font.digits.height,
            );
            Text::raw(digit).style(style).render(focus_digit_area, buf);
            current_x += size + timer_font.digits.gap;
        }
    }
    pub fn draw_keybinds(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        let keys = [("Quit", "Q"), ("Focus", "F"), ("Break", "B")];

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
}

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::new(self.settings.theme);

        let timer_font = TimerFont::new(area.width, area.height, self.is_focusing, self.settings);
        let timer_block_total_height = timer_font.digits.height + 4;
        let total_text_height = timer_block_total_height + 1 + timer_font.title.height;
        let x = ((area.width as i16 - timer_font.title.width as i16) / 2 + timer_font.title.offset)
            as u16;
        let y = ((area.height as i16 - total_text_height as i16) / 2) as u16;
        let title_area = Rect::new(x, y, timer_font.title.width, timer_font.title.height);
        Text::raw(timer_font.title.text.clone())
            .style(theme.timer.title)
            .render(title_area, buf);

        let timer_block_total_width =
            (timer_font.digits.stopwatch_width + 2) * 2 + (timer_font.digits.balance_width + 2) + 2;

        let timers_area = Rect::new(
            (area.width - timer_block_total_width) / 2,
            title_area.y + title_area.height,
            timer_block_total_width,
            timer_block_total_height,
        );
        let keybinds_area = Rect::new(
            timers_area.x,
            timers_area.y + timers_area.height,
            timers_area.width,
            3,
        );
        self.draw_timers(timers_area, buf, timer_font, theme);
        self.draw_keybinds(keybinds_area, buf, theme);
    }
}
