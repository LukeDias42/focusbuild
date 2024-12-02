use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    pub theme: Theme,
    pub font_size: FontSize,
    pub focus_break_proportion: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::default(),
            font_size: FontSize::default(),
            focus_break_proportion: 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    BloodRiver,
    GalaxyBlue,
    EvergreenForest,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::BloodRiver => Self::GalaxyBlue,
            Self::GalaxyBlue => Self::EvergreenForest,
            Self::EvergreenForest => Self::BloodRiver,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::BloodRiver => Self::EvergreenForest,
            Self::GalaxyBlue => Self::BloodRiver,
            Self::EvergreenForest => Self::GalaxyBlue,
        }
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(input: &str) -> Result<Theme, Self::Err> {
        match input {
            "BloodRiver" => Ok(Theme::BloodRiver),
            "GalaxyBlue" => Ok(Theme::GalaxyBlue),
            "EvergreenForest" => Ok(Theme::EvergreenForest),
            _ => Err(format!("{input} is not a valid theme")),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ToSql for Theme {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for Theme {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(value.as_str()?.parse().unwrap_or(Theme::default()))
    }
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FontSize {
    #[default]
    Auto,
    Large,
    Small,
    Tiny,
}

impl FontSize {
    pub fn next(self) -> Self {
        match self {
            Self::Auto => Self::Large,
            Self::Large => Self::Small,
            Self::Small => Self::Tiny,
            Self::Tiny => Self::Tiny,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Auto => Self::Tiny,
            Self::Large => Self::Auto,
            Self::Small => Self::Large,
            Self::Tiny => Self::Small,
        }
    }
}

impl FromStr for FontSize {
    type Err = String;

    fn from_str(input: &str) -> Result<FontSize, Self::Err> {
        match input {
            "Auto" => Ok(FontSize::Auto),
            "Large" => Ok(FontSize::Large),
            "Small" => Ok(FontSize::Small),
            "Tiny" => Ok(FontSize::Tiny),
            _ => Err(format!("{input} is not a valid font size")),
        }
    }
}

impl Display for FontSize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ToSql for FontSize {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for FontSize {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(value.as_str()?.parse().unwrap_or(FontSize::default()))
    }
}
