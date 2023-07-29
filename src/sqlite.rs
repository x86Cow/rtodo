use rusqlite::{Connection, Result};

pub fn create_db() -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE table IF NOT EXISTS todos {
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        }",
        [],
        )?;
    Ok(())
}
