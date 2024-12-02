use color_eyre::Result;
use rusqlite::params;

use crate::{
    infra::db::Db,
    models::settings::{FontSize, Settings, Theme},
};

#[derive(Debug)]
pub struct SettingsRepository {
    db: Db,
}

impl SettingsRepository {
    pub fn new() -> Result<SettingsRepository> {
        let db = Db::new(
            "create table if not exists settings (
             id INTEGER PRIMARY KEY,
             theme TEXT NOT NULL,
             font_size TEXT NOT NULL,
             focus_break_proportion INTEGER NOT NULL
         )",
        )?;

        Ok(SettingsRepository { db })
    }

    pub fn insert_settings(&self, settings: Settings) -> Result<Settings> {
        self.db.conn.execute(
            "INSERT INTO settings (id, theme, font_size, focus_break_proportion) values (1, ?1, ?2, ?3)",
            params![settings.theme,  settings.font_size, settings.focus_break_proportion],
        )?;
        Ok(settings)
    }

    pub fn get_settings(&self) -> Result<Settings> {
        let mut stmt = self.db.conn.prepare(
            "
            SELECT theme, font_size, focus_break_proportion
            FROM settings
            WHERE id=1;
            ",
        )?;

        let settings = stmt.query_row([], |row| {
            let theme: Theme = row.get(0)?;
            let font_size: FontSize = row.get(1)?;
            let proportion: i64 = row.get(2)?;
            Ok(Settings {
                theme,
                font_size,
                focus_break_proportion: proportion as u16,
            })
        })?;

        Ok(settings)
    }
}
