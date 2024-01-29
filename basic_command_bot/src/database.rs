/// database.rs

//use rusqlite::{Connection, Result};
use rusqlite::{Result};
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

    //conn.execute("DROP TABLE IF EXISTS administrators", []).expect("Failed to drop table"); // Reset switch
    conn.execute(
        "CREATE TABLE IF NOT EXISTS administrators (
            username TEXT PRIMARY KEY
        )",
        [],
    ).expect("Failed to create administrators table");

    // Add default administrators if they don't exist
    let default_admins = vec!["juno0x153", "novo2424"];
    for admin in default_admins {
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM administrators WHERE username = ?1",
            &[admin],
            |row| row.get(0),
        ).unwrap_or(0);

        if exists == 0 {
            conn.execute(
                "INSERT INTO administrators (username) VALUES (?1)",
                &[admin],
            ).expect("Failed to insert default administrator");
        }
    }
    conn.execute("DROP TABLE IF EXISTS seasons", []).expect("Failed to drop table"); // Reset switch
    conn.execute(
    "CREATE TABLE IF NOT EXISTS seasons (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        is_active BOOLEAN NOT NULL,
        max_players INTEGER NOT NULL,
        start_date TEXT,
        stop_date TEXT,
        status TEXT NOT NULL DEFAULT 'initial'
        )",
        [],
    ).expect("Failed to create modified seasons table");

    //conn.execute("DROP TABLE IF EXISTS channel_settings", []).expect("Failed to drop table"); // Reset switch
    conn.execute(
        "CREATE TABLE IF NOT EXISTS channel_settings (
            id INTEGER PRIMARY KEY,
            broadcast_channel_id TEXT,
            group_channel_id TEXT
        )",
        [],
    ).expect("Failed to create channel_settings table");

    // insert initial row. Not sure why it should be created but we will see.
    conn.execute(
        "INSERT INTO channel_settings (id, broadcast_channel_id, group_channel_id) VALUES (1, NULL, NULL) ON CONFLICT(id) DO NOTHING",
        [],
    ).expect("Failed to insert initial row into channel_settings");
    
    // Create the MasterRoundTable
    conn.execute("DROP TABLE IF EXISTS MasterRoundTable", []).expect("Failed to drop MasterRoundTable"); // Reset switch
    conn.execute(
        "CREATE TABLE IF NOT EXISTS MasterRoundTable (
            id INTEGER PRIMARY KEY,
            season_id INTEGER,
            round_number INTEGER NOT NULL,
            start_time TEXT,
            end_time TEXT,
            FOREIGN KEY(season_id) REFERENCES Seasons(id)
        )",
        [],
    ).expect("Failed to create MasterRoundTable");

    // Create the RoundDetailsTable
    conn.execute("DROP TABLE IF EXISTS RoundDetailsTable", []).expect("Failed to drop RoundDetailsTable"); // Reset switch
    conn.execute(
        "CREATE TABLE IF NOT EXISTS RoundDetailsTable (
            id INTEGER PRIMARY KEY,
            round_id INTEGER,
            player_username TEXT,
            player_id INTEGER,
            player_hand TEXT,
            opponent_username TEXT,
            opponent INTEGER,
            opponent_hand TEXT,
            timestamp TEXT,
            game_status TEXT,
            FOREIGN KEY(round_id) REFERENCES MasterRoundTable(id)
        )",
        [],
    ).expect("Failed to create RoundDetailsTable");


    conn.execute("DROP TABLE IF EXISTS PlayerDetailsTable", []).expect("Failed to drop PlayerDetailsTable"); // Reset switch
    // Create the PlayerDetailsTable
    conn.execute(
        "CREATE TABLE IF NOT EXISTS PlayerDetailsTable (
            id INTEGER PRIMARY KEY,
            season_id INTEGER,
            player_id INTEGER,
            player_username TEXT,
            player_wallet TEXT,
            score INTEGER,
            FOREIGN KEY(season_id) REFERENCES Seasons(id)
        )",
        [],
    ).expect("Failed to create PlayerDetailsTable");

    conn.execute("DROP TABLE IF EXISTS MasterCandidateTable", []).expect("Failed to drop MasterCandidateTable"); // Reset switch
    // Create the MasterCandidateTable
    conn.execute(
        "CREATE TABLE IF NOT EXISTS MasterCandidateTable (
            id INTEGER PRIMARY KEY,
            season_id INTEGER,
            player_id INTEGER,
            player_username TEXT,
            player_wallet TEXT,
            player_status TEXT,
            FOREIGN KEY(season_id) REFERENCES Seasons(id)
        )",
        [],
    ).expect("Failed to create MasterCandidateTable");

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
