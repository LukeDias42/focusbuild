use ratatui::style::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub key_binding: KeyBinding,
    pub logo: Style,
    pub timer: Timer,
    pub history: History,
    pub data: Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
    pub surrounding: Style,
    pub block: Style,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    pub active: Style,
    pub inactive: Style,
    pub title: Style,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data {
    pub break_bar: Style,
    pub focus_bar: Style,
    pub block: Style,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct History {
    pub style: Style,
    pub border: Style,
}

impl Theme {
    pub fn new(theme_settings: crate::models::settings::Theme) -> Self {
        match theme_settings {
            crate::models::settings::Theme::BloodRiver => BLOOD_RIVER,
            crate::models::settings::Theme::GalaxyBlue => GALAXY_BLUE,
            crate::models::settings::Theme::EvergreenForest => EVERGREEN_FOREST,
        }
    }
}

const BLOOD_RIVER: Theme = Theme {
    logo: Style::new().fg(EARTH_RED),
    key_binding: KeyBinding {
        key: Style::new().fg(EARTH_RED).bg(CHARCOAL_BROWN_DARK),
        description: Style::new().fg(EARTH_RED).bg(Color::Reset),
        surrounding: Style::new().fg(EARTH_RED),
        block: Style::new().fg(EARTH_RED),
    },
    timer: Timer {
        active: Style::new().bg(DARK_MOSS_GREEN).fg(EARTH_RED),
        inactive: Style::new().fg(EARTH_RED),
        title: Style::new().fg(EARTH_RED),
    },
    history: History {
        style: Style::new().fg(EARTH_RED),
        border: Style::new().fg(EARTH_RED),
    },
    data: Data {
        break_bar: Style::new().fg(LIGHT_RED),
        focus_bar: Style::new().fg(FOCUS_GREEN),
        block: Style::new().fg(Color::White),
    },
};

const GALAXY_BLUE: Theme = Theme {
    logo: Style::new().fg(STELLAR_CYAN), // Bright cyan for cosmic feel
    key_binding: KeyBinding {
        key: Style::new().fg(STELLAR_CYAN).bg(DARK_NEBULA),
        description: Style::new().fg(STELLAR_CYAN).bg(Color::Reset),
        surrounding: Style::new().fg(STELLAR_CYAN),
        block: Style::new().fg(STELLAR_CYAN),
    },
    timer: Timer {
        active: Style::new().bg(GALAXY_BLUE_COLOR).fg(STELLAR_CYAN),
        inactive: Style::new().fg(COSMIC_GRAY),
        title: Style::new().fg(STELLAR_CYAN),
    },
    history: History {
        style: Style::new().fg(STELLAR_CYAN),
        border: Style::new().fg(DARK_NEBULA),
    },
    data: Data {
        break_bar: Style::new().fg(NEBULA_PINK),
        focus_bar: Style::new().fg(STARLIGHT_YELLOW),
        block: Style::new().fg(Color::White),
    },
};

const EVERGREEN_FOREST: Theme = Theme {
    logo: Style::new().fg(FOREST_GREEN), // Evergreen logo
    key_binding: KeyBinding {
        key: Style::new().fg(CHARCOAL_BROWN).bg(DARK_MOSS_GREEN),
        description: Style::new().fg(FOREST_GREEN).bg(Color::Reset),
        surrounding: Style::new().fg(FOREST_GREEN),
        block: Style::new().fg(FOREST_GREEN),
    },
    timer: Timer {
        active: Style::new().bg(DARK_MOSS_GREEN).fg(FOREST_GREEN),
        inactive: Style::new().fg(CHARCOAL_BROWN),
        title: Style::new().fg(FOREST_GREEN),
    },
    history: History {
        style: Style::new().fg(FOREST_GREEN),
        border: Style::new().fg(DARK_MOSS_GREEN),
    },
    data: Data {
        break_bar: Style::new().fg(CHARCOAL_BROWN_LIGHT),
        focus_bar: Style::new().fg(FOREST_GREEN),
        block: Style::new().fg(Color::White),
    },
};

const FOREST_GREEN: Color = Color::Rgb(34, 139, 34);
const CHARCOAL_BROWN: Color = Color::Rgb(89, 57, 47);
const CHARCOAL_BROWN_DARK: Color = Color::Rgb(40, 28, 22);
const CHARCOAL_BROWN_LIGHT: Color = Color::Rgb(140, 95, 75);
const EARTH_RED: Color = Color::Rgb(0xAD, 0x4B, 0x44);
const LIGHT_RED: Color = Color::Rgb(0xFF, 0x77, 0x77);
const FOCUS_GREEN: Color = Color::Rgb(0x17, 0xad, 0x52);
const DARK_MOSS_GREEN: Color = Color::Rgb(0x2D, 0x33, 0x30);
const GALAXY_BLUE_COLOR: Color = Color::Rgb(25, 25, 112);
const STELLAR_CYAN: Color = Color::Rgb(0, 139, 139);
const NEBULA_PINK: Color = Color::Rgb(219, 112, 147);
const COSMIC_GRAY: Color = Color::Rgb(105, 105, 105);
const STARLIGHT_YELLOW: Color = Color::Rgb(255, 223, 0);
const DARK_NEBULA: Color = Color::Rgb(44, 62, 80);
