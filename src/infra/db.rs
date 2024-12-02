use color_eyre::Result;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(initialize_query: &str) -> Result<Db> {
        let conn = Connection::open("db/focusbuild.db")?;

        conn.execute(initialize_query, ())?;

        Ok(Db { conn })
    }
}
