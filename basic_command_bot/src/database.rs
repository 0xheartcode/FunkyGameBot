use rusqlite::{Connection, Result};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

// Create a type alias for the pool for convenience
pub type DbPool = Pool<SqliteConnectionManager>;

// Function to create and initialize the database pool
pub fn init_db_pool() -> DbPool {
    let manager = SqliteConnectionManager::file("my_database.db");
    let pool = Pool::new(manager).expect("Failed to create the database pool");

    // Initialize the database schema
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS my_table (
            id INTEGER PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");

    pool
}

// Function to write a value to the SQLite database
pub async fn write_to_db(pool: &DbPool, value: &str) -> Result<()> {
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute("INSERT INTO my_table (value) VALUES (?1)", [value])?;
    Ok(())
}

// Function to read a value from the SQLite database
pub async fn read_from_db(pool: &DbPool) -> Result<String> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut stmt = conn.prepare("SELECT value FROM my_table ORDER BY id DESC LIMIT 1")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Ok("No data found".to_string())
    }
}

