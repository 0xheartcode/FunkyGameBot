// admin.rs

use teloxide::{prelude::*};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

// Lazy static to store admin usernames
lazy_static! {
    static ref ADMIN_USERNAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

// Function to add a user to the admin list
pub fn add_admin(username: String) {
    ADMIN_USERNAMES.lock().unwrap().insert(username);
}

// Function to remove a user from the admin list
pub fn remove_admin(username: &str) {
    ADMIN_USERNAMES.lock().unwrap().remove(username);
}

// Function to list admin users
pub fn list_admins() -> Vec<String> {
    ADMIN_USERNAMES.lock().unwrap().iter().cloned().collect()
}

pub fn is_admin(username: &str) -> bool {
    ADMIN_USERNAMES.lock().unwrap().contains(username)
}

pub fn is_authorized_sender(msg: &Message) -> bool {
    if let Some(true_sender_username) = msg.from().and_then(|user| user.username.as_ref()) {
        true_sender_username == "juno0x153" || true_sender_username == "novo2424" || is_admin(true_sender_username)
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
