- [X] Preparations
---
- [X] Set the bots profile picture
- [X] Create all essential files
	- [X] Set SECRETS
	- [X] Copy workflow.yml
	- [X] Create folder structure
	- [X] Makefile
	
---
- [X] Basic Bot with Rust
- [X] Basic Bot with Rust and SQLlite
- [X] No Docker, Rust + SQLlite inconvenient at the moment.

- [ ] Basic Bot with Rust and SQLlite in CI/CD
- [ ] CI/CD connected to my remote cloud server. InProgress
---
- Implement the game logic, step by step.
- Monday


---
- Further: use MongoDB instead of SQLlite
- Docker:
https://www.reddit.com/r/rust/comments/126xeyx/exploring_the_problem_of_faster_cargo_docker/
https://github.com/LukeMathWalker/cargo-chef

- Further: notes on how to async block database with tokio
Step 1: Create a Utility Function
You can create a utility function that takes a closure and executes it using tokio::task::spawn_blocking. This function should be generic to handle various types of operations:

```rust
use std::sync::Arc;
use tokio::task;
use rusqlite::Error as RusqliteError;
use crate::database::DbPool;

// Utility function to run blocking database operations asynchronously
async fn run_blocking_db_operation<F, R>(pool: Arc<DbPool>, operation: F) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
where
    F: FnOnce(Arc<DbPool>) -> Result<R, RusqliteError> + Send + 'static,
    R: Send + 'static,
{
    task::spawn_blocking(move || operation(pool)).await?
}

```

In this utility function:

F is the type of the closure you pass to the function.
R is the return type of the closure.
The closure takes an Arc<DbPool> as its argument and returns a Result<R, RusqliteError>.
The function itself returns a Result<R, Box<dyn std::error::Error + Send + Sync>> to handle different types of errors.
## Step 2: Use the Utility Function in is_admin
Now, you can use this utility function in your is_admin function:
```rust
pub async fn is_admin(pool: Arc<DbPool>, username: String) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    run_blocking_db_operation(pool, move |p| {
        let conn = p.get().expect("Failed to get connection from pool");
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM administrators WHERE username = ?1")?;
        let count: i64 = stmt.query_row(&[&username], |row| row.get(0))?;
        Ok(count > 0)
    }).await
}

```

In this version of is_admin:

The username parameter is taken by value since it needs to be moved into the async block.
The run_blocking_db_operation utility function is used to execute the database operation.
Step 3: Reuse for Other Functions
You can now easily use run_blocking_db_operation for other similar operations in your code. For example, if you have another function that performs a database write operation, you can use this utility function in a similar way.

This approach centralizes the handling of async and sync interoperation, reducing code duplication and simplifying the integration of synchronous database operations in an async context.
