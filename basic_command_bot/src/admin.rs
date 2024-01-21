/// admin.rs

use teloxide::{prelude::*};
use crate::database::{DbPool};
use rusqlite::Error as RusqliteError;
//mod gamefunctions;
//use crate::gamefunctions::season::{start_new_season, stop_current_season, current_active_season};

// Function to add an administrator to the database
pub fn add_admin(pool: &DbPool, username: &str) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute("INSERT INTO administrators (username) VALUES (?1)", [username])?;
    Ok(())
}

// Function to remove an administrator from the database
pub fn remove_admin(pool: &DbPool, username: &str) -> Result<(), RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute("DELETE FROM administrators WHERE username = ?1", [username])?;
    Ok(())
}

// Function to list all administrators from the database
pub fn list_admins(pool: &DbPool) -> Result<Vec<String>, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut stmt = conn.prepare("SELECT username FROM administrators")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    
    let mut admins = Vec::new();
    for admin in rows {
        admins.push(admin?);
    }
    Ok(admins)
}

// Function to check if a username is an administrator
pub fn is_admin(pool: &DbPool, username: &str) -> Result<bool, RusqliteError> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM administrators WHERE username = ?1")?;
    let count: i64 = stmt.query_row([username], |row| row.get(0))?;
    
    Ok(count > 0)
}

pub fn is_authorized_sender(msg: &Message, pool: &DbPool) -> bool {
    if let Some(true_sender_username) = msg.from().and_then(|user| user.username.as_ref()) {
        true_sender_username == "juno0x153" || true_sender_username == "novo2424" || is_admin(pool, true_sender_username).unwrap_or(false)
    } else {
        false
    }
}


pub fn is_authorized_dev(msg: &Message) -> bool {
    if let Some(true_sender_username) = msg.from().and_then(|user| user.username.as_ref()) {
        true_sender_username == "juno0x153" 
    } else {
        false
    }
}




