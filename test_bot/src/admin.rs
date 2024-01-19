// admin.rs

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
