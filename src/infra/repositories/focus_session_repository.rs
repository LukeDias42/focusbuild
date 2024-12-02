use chrono::{DateTime, FixedOffset, Local};
use color_eyre::Result;
use rusqlite::params;

use crate::{infra::db::Db, models::focus_session::FocusSession};

#[derive(Debug)]
pub struct FocusSessionRepository {
    db: Db,
}

impl FocusSessionRepository {
    pub fn new() -> Result<FocusSessionRepository> {
        let db = Db::new(
            "create table if not exists focus_session (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             start TEXT NOT NULL,
             focus_seconds INTEGER NOT NULL,
             break_seconds INTEGER NOT NULL
         )",
        )?;

        Ok(FocusSessionRepository { db })
    }

    pub fn insert_focus_session(
        &self,
        start: DateTime<Local>,
        focus_seconds: u64,
        break_seconds: u64,
    ) -> Result<FocusSession> {
        self.db.conn.execute(
            "INSERT INTO focus_session (start, focus_seconds, break_seconds) values (?1, ?2, ?3)",
            params![start.to_string(), focus_seconds, break_seconds],
        )?;
        let id = self.db.conn.last_insert_rowid();
        Ok(FocusSession {
            id,
            start,
            focus_seconds,
            break_seconds,
        })
    }
    pub fn select_many(&self) -> Result<Vec<FocusSession>> {
        let mut stmt = self.db.conn.prepare(
            "
            SELECT id, start, focus_seconds, break_seconds
            FROM focus_session;
            ",
        )?;

        let focus_sessions_iter = stmt.query_map([], |row| {
            let start: String = row.get(1)?;
            let start_fixed_offset: DateTime<FixedOffset> =
                start.parse::<DateTime<FixedOffset>>().map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;

            let start_local = start_fixed_offset.with_timezone(&Local);
            Ok(FocusSession {
                id: row.get(0)?,
                start: start_local,
                focus_seconds: row.get(2)?,
                break_seconds: row.get(3)?,
            })
        })?;

        // Collect all FocusSession instances into a Vec
        let mut focus_sessions = Vec::new();
        for session in focus_sessions_iter {
            focus_sessions.push(session?);
        }

        Ok(focus_sessions)
    }
}
