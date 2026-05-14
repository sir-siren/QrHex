# Rust -- Universal AI Development Rules
> Drop this as `CLAUDE.md` or `AGENTS.md` at your project root.
> Every rule here is non-negotiable. Zero exceptions.

---

## 0 -- Philosophy

| Principle | What it means in this codebase |
|---|---|
| **SOLID** | Every module has one reason to change. Depend on traits, not concretions. |
| **KISS** | Simplest solution that works. No premature abstraction. |
| **YAGNI** | Never write code for a feature that does not exist yet. |
| **Clean Code** | Code explains HOW. Comments explain WHY. Names tell the full story. |
| **Zero `unsafe`** | If you reach for `unsafe`, the design is wrong. Redesign first. |
| **Fault isolation** | One module failing must never take down another. |

---

## 1 -- Rust Version

Target **Rust 1.85+ (edition 2024)**. No exceptions.

### Step 1 -- detect installed version

```bash
rustc --version
rustup show active-toolchain
```

If nothing found on host, check WSL:

```bash
wsl rustc --version
```

If WSL has rust, use it. All commands go through WSL from that point:

```bash
wsl cargo build
wsl cargo test
```

### Step 1b -- if dev says rust is in WSL

Trust them immediately. Switch all commands to `wsl` prefix. No pushback.

Remind them once:

> got it, using WSL rust. just a heads up -- project files inside WSL home (~/)
> build way faster than on the Windows side (/mnt/c/...).

Then drop it. Never mention it again.

### Step 2 -- version found but below 1.85

Tell them:

> you are on X -- this ruleset targets rust 1.85+ (edition 2024).
> run `rustup update stable` to get there, then come back.

Do not proceed until they are on 1.85+.

### Step 3 -- no rust found anywhere

Ask:

> cant find rustc on your PATH or in WSL. do you have rust installed?
> if its in WSL just say so and we are good.
> if not, lets fix that first.

If they have no rust:

> install it here: https://rustup.rs
> once done run `rustc --version` and tell me what you get.

Do not write any code until rust is confirmed.

### Step 4 -- dev gives a version number

Use it. Note it in every generated `Cargo.toml`:

```toml
# rust 1.85+ required (edition 2024)
[package]
name = "my-project"
edition = "2024"
```

### Version compatibility quick reference

| Rust version | Key feature |
|---|---|
| 1.65 | `let-else` statements |
| 1.70 | `OnceCell`, `OnceLock` in std |
| 1.74 | `[lints]` table in Cargo.toml |
| 1.80 | `LazyCell`, `LazyLock` in std |
| 1.82 | `&raw` pointers, `impl Trait` in closures |
| 1.85 | edition 2024 stable |
| 1.87 | `if let` chains stable |

---

## 2 -- Naming Conventions

### Variables and functions -- snake_case

```rust
// correct
let player_health_points: u32 = 100;
let maximum_retry_count: usize = 3;

fn calculate_total_damage(base: f32, multiplier: f32) -> f32 { ... }
fn is_player_outside_bounds(x: f32, y: f32) -> bool { ... }
fn parse_config_from_file(path: &str) -> Result<Config, ConfigError> { ... }

// wrong -- never do this
let x = 100;
let tmp = get_data();
fn do_thing(a: f32, b: f32) -> f32 { ... }
```

### Types, structs, enums, traits -- PascalCase

```rust
struct PlayerConfig { ... }
struct HttpClient { ... }
enum ConnectionState { Connecting, Connected, Disconnected, Failed(String) }
trait Serializable { ... }
trait EventHandler { ... }
```

### Constants and statics -- SCREAMING_SNAKE_CASE

```rust
const MAX_RETRY_ATTEMPTS: u32 = 3;
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();
```

### Type aliases -- PascalCase

```rust
type UserId = u64;
type Timestamp = u64;
type Result<T> = std::result::Result<T, AppError>;
```

### Booleans -- always prefixed

```rust
let is_connected: bool = false;
let has_pending_requests: bool = true;
let can_retry: bool = false;
let should_flush_buffer: bool = true;
```

### Modules and files -- snake_case

```
src/
  user_profile.rs
  http_client.rs
  config_parser.rs
```

---

## 3 -- Project Structure

### Single crate (small tool or library)

```
my-project/
├── src/
│   ├── lib.rs          # library root (if lib)
│   ├── main.rs         # binary root (if bin)
│   ├── config/
│   │   ├── mod.rs
│   │   └── parser.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   └── models.rs
│   └── errors.rs
├── tests/
│   └── integration_tests.rs
├── benches/
│   └── throughput.rs
├── Cargo.toml
├── Cargo.lock
├── CLAUDE.md
└── QUICKSTART.md
```

### Workspace (multi-crate, production scale)

```
my-project/
├── crates/
│   ├── core/           # pure domain logic -- no IO, no framework deps
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── domain/
│   │   │   └── errors.rs
│   │   └── Cargo.toml
│   ├── cli/            # thin CLI wrapper around core
│   │   ├── src/main.rs
│   │   └── Cargo.toml
│   ├── server/         # thin HTTP/gRPC wrapper around core
│   │   ├── src/main.rs
│   │   └── Cargo.toml
│   └── storage/        # persistence layer -- depends on core
│       ├── src/lib.rs
│       └── Cargo.toml
├── tests/
├── Cargo.toml          # workspace root
├── Cargo.lock
├── CLAUDE.md
└── QUICKSTART.md
```

### Module boundary law

- `core` has zero knowledge of IO, HTTP, CLI, or any framework.
- outer crates (`cli`, `server`) are thin wrappers. they call core, never the other way.
- no circular dependencies between crates. ever.

---

## 4 -- Cargo.toml Rules

### Workspace root

```toml
[workspace]
members = ["crates/core", "crates/cli", "crates/server"]
resolver = "2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 1

[workspace.lints.rust]
unsafe_code = "forbid"
unused = "warn"
dead_code = "warn"
```

### Crate Cargo.toml

```toml
[package]
name = "core"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[lints]
workspace = true

[dependencies]
serde = { version = "1", features = ["derive"] }
thiserror = "2"
log = "0.4"

[dev-dependencies]
pretty_assertions = "1"
```

### Dependency rules

- always pin major versions: `serde = "1"` not `serde = "*"`
- justify every dependency with a comment if it is not obvious
- check `cargo tree` before adding anything -- do not pull in a crate for one function
- prefer std over external crates where reasonable

---

## 5 -- SOLID in Rust

### S -- Single Responsibility

```rust
// correct -- each struct does one thing
struct ConfigParser { ... }
struct DatabaseConnection { ... }
struct RequestValidator { ... }

// wrong -- god struct
struct AppManager {
    fn parse_config(&self) { ... }
    fn connect_to_db(&self) { ... }
    fn validate_request(&self) { ... }
    fn send_email(&self) { ... }
    fn generate_report(&self) { ... }
}
```

### O -- Open/Closed (via traits)

```rust
// adding a new storage backend never touches existing ones
trait Storage {
    fn read(&self, key: &str) -> Result<Vec<u8>, StorageError>;
    fn write(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;
}

struct FileStorage { ... }
struct RedisStorage { ... }
struct MemoryStorage { ... }

impl Storage for FileStorage { ... }
impl Storage for RedisStorage { ... }
impl Storage for MemoryStorage { ... }
```

### L -- Liskov Substitution

```rust
// any impl of Storage must behave correctly everywhere Storage is used
fn backup_data<S: Storage>(storage: &S, data: &[u8]) -> Result<(), StorageError> {
    storage.write("backup", data)
}
// FileStorage, RedisStorage, MemoryStorage -- all must work here without surprises
```

### I -- Interface Segregation

```rust
// correct -- lean focused traits
trait Readable { fn read(&self, key: &str) -> Result<Vec<u8>, StorageError>; }
trait Writable { fn write(&self, key: &str, value: &[u8]) -> Result<(), StorageError>; }
trait Deletable { fn delete(&self, key: &str) -> Result<(), StorageError>; }

// wrong -- fat trait forces implementors to provide things they dont need
trait Storage {
    fn read(&self, key: &str) -> Result<Vec<u8>, StorageError>;
    fn write(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;
    fn delete(&self, key: &str) -> Result<(), StorageError>;
    fn list_all_keys(&self) -> Result<Vec<String>, StorageError>;
    fn get_storage_stats(&self) -> StorageStats;
    fn flush_all(&self) -> Result<(), StorageError>;
}
```

### D -- Dependency Inversion

```rust
// correct -- depend on the trait, not the concrete type
struct UserService<R: UserRepository> {
    repository: R,
}

// wrong -- hardcoded to one implementation forever
struct UserService {
    repository: PostgresUserRepository,
}
```

---

## 6 -- Error Handling

No panics in production. No `unwrap`. No `expect`. No exceptions.

### Define domain errors

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("user {user_id} not found")]
    UserNotFound { user_id: u64 },

    #[error("database connection failed: {reason}")]
    DatabaseConnectionFailed { reason: String },

    #[error("config file missing at {path}")]
    ConfigFileMissing { path: String },

    #[error("invalid input: {message}")]
    InvalidInput { message: String },
}
```

### Propagate with ?

```rust
pub fn load_user(user_id: u64) -> Result<User, AppError> {
    let row = db.query_one(user_id)
        .map_err(|e| AppError::DatabaseConnectionFailed { reason: e.to_string() })?;

    let user = parse_user_row(row)
        .map_err(|e| AppError::InvalidInput { message: e.to_string() })?;

    Ok(user)
}
```

### At binary entry point -- handle and exit cleanly

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    // all application logic here
    Ok(())
}
```

### What is banned

```rust
// banned
let user = db.get(id).unwrap();
let config = load_config().expect("config must exist");
panic!("this should never happen");

// also banned -- silently swallowing errors
let _ = cleanup_temp_files();

// allowed -- only in tests
#[test]
fn test_something() {
    let result = parse("valid input").unwrap();  // fine in tests
}
```

---

## 7 -- Memory Safety

### Ownership and borrowing

```rust
// prefer borrowing over cloning
fn print_username(user: &User) {  // borrow, not move or clone
    println!("{}", user.name);
}

// clone only at boundaries where you genuinely need ownership
fn spawn_background_task(config: Config) {
    let owned_config = config.clone();  // needed -- thread takes ownership
    std::thread::spawn(move || process(owned_config));
}
```

### Integer arithmetic -- always explicit

```rust
// correct -- never silent overflow
let new_health = current_health.saturating_sub(damage);
let new_score = current_score.checked_add(points).unwrap_or(u64::MAX);
let bounded = value.clamp(0, MAX_VALUE);

// wrong -- silent wrapping on overflow in debug, UB in release
let result = a - b;
```

### Collections -- pre-allocate when size is known

```rust
// correct
let mut results = Vec::with_capacity(input.len());

// wrong -- grows and reallocates repeatedly
let mut results = Vec::new();
```

### No raw pointer manipulation

```rust
// correct -- use Box, Arc, Rc for heap allocation
let boxed = Box::new(LargeStruct::new());
let shared = Arc::new(SharedData::new());

// banned
let raw = Box::into_raw(Box::new(data));  // memory leak risk
```

---

## 8 -- Concurrency

```rust
// use Arc<Mutex<T>> for shared mutable state across threads
use std::sync::{Arc, Mutex};

let shared_state = Arc::new(Mutex::new(AppState::new()));

let state_clone = Arc::clone(&shared_state);
std::thread::spawn(move || {
    let mut state = state_clone.lock().expect("mutex poisoned");
    state.increment_counter();
});

// use channels for message passing -- prefer over shared state
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel::<WorkItem>();

std::thread::spawn(move || {
    for item in receiver {
        process_work_item(item);
    }
});

// use OnceLock for global init
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::load_from_env())
}
```

---

## 9 -- Testing

Write tests unless the developer explicitly says to skip them.

If a codebase has no tests, write tests for existing code before adding anything new.

### Unit tests -- same file

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_does_not_underflow_below_zero() {
        let mut player = Player::new(100);
        player.apply_damage(999);
        assert_eq!(player.health, 0);
    }

    #[test]
    fn parse_config_fails_on_missing_required_field() {
        let input = r#"{ "port": 8080 }"#;  // missing "host"
        let result = parse_config(input);
        assert!(result.is_err());
    }

    #[test]
    fn user_id_zero_returns_not_found_error() {
        let result = find_user(0);
        assert!(matches!(result, Err(AppError::UserNotFound { .. })));
    }
}
```

### Integration tests -- tests/ folder

```rust
// tests/integration_tests.rs
use my_project::*;

#[test]
fn full_request_pipeline_returns_200_for_valid_input() {
    let app = App::new_for_testing();
    let response = app.handle_request(valid_test_request());
    assert_eq!(response.status_code, 200);
}
```

### Coverage targets

- happy path: normal input, expected output.
- edge cases: zero, max, empty string, boundary values.
- error cases: what should fail and how it should fail.
- do not test implementation details. test observable behavior.

### Test naming -- describe what it does

```rust
// correct -- reads like a sentence
fn user_with_expired_token_gets_401_response()
fn empty_input_returns_invalid_input_error()
fn retry_stops_after_maximum_attempt_count()

// wrong -- meaningless
fn test1()
fn test_user()
fn it_works()
```

---

## 10 -- Debugging

### Before touching anything

Read the full error output. All of it. Rust errors are actually helpful.

```bash
# full output
cargo build 2>&1 | less

# in WSL
wsl cargo build 2>&1 | less
```

### Debug checklist -- run in this order

```bash
# 1. type and borrow check fast
cargo check

# 2. lint -- warnings are errors
cargo clippy --all-targets -- -D warnings

# 3. tests
cargo test --workspace

# 4. tests with output
cargo test --workspace -- --nocapture

# 5. release build check
cargo build --release
```

### Common patterns

**borrow checker fight:**
```rust
// clone once to unblock yourself, leave a note
// TODO: 2024-xx-xx -- restructure ownership to remove this clone
let snapshot = self.state.clone();
```

**tracking down a silent bug:**
```rust
// add temporary debug assertions to narrow it down
debug_assert!(index < self.items.len(), "index {index} out of bounds");
debug_assert!(!name.is_empty(), "name must not be empty at this point");
// debug_assert! compiles to nothing in release -- safe to leave
```

**checking what a type looks like at runtime:**
```rust
dbg!(&my_value);  // prints file, line, and value to stderr
eprintln!("state at this point: {:#?}", self.state);  // pretty-printed
```

**performance regression:**
```bash
# add a bench in benches/ and run
cargo bench

# or use cargo-flamegraph for a flame graph
cargo install flamegraph
cargo flamegraph --bin my-binary
```

**surprising behavior in release that works in dev:**
```toml
# temporarily drop optimization to isolate the bug
[profile.release]
opt-level = 1   # temp -- revert after debugging
debug = true    # temp -- adds debug symbols to release
```

### Logging -- use log crate, not println

```rust
// Cargo.toml
[dependencies]
log = "0.4"
env_logger = "0.11"   # or tracing if you need structured logs

// main.rs
fn main() {
    env_logger::init();
    run().unwrap_or_else(|e| {
        log::error!("fatal: {e}");
        std::process::exit(1);
    });
}

// anywhere
log::debug!("processing item: {:?}", item);
log::info!("server started on port {}", port);
log::warn!("retry attempt {} of {}", attempt, max_attempts);
log::error!("connection failed: {}", error);
```

Run with:

```bash
RUST_LOG=debug cargo run
RUST_LOG=my_crate=debug cargo run   # only your crate
```

---

## 11 -- Project Understanding Before Writing Code

Do not write a single line of code until these steps are done.

### Step 1 -- scan the structure

Look at the layout first:

```
src/
Cargo.toml
Cargo.lock
tests/
```

Ask:
- what is this project doing?
- what crates are in the workspace?
- where is the entry point?
- what does the existing public API look like?

### Step 2 -- read the key files

At minimum:

- `Cargo.toml` (workspace + members)
- `src/lib.rs` or `src/main.rs`
- top-level `mod.rs` files
- `tests/` folder

### Step 3 -- check for existing tests

```bash
find . -name "*.rs" | xargs grep -l "#\[test\]"
cargo test --workspace
```

- tests exist: run them, understand coverage, do not break them.
- no tests: write tests for the existing code first, then add the new feature.
- dev said skip tests: skip and move on, no argument.

### Step 4 -- confirm understanding before coding

Summarize in 2-3 sentences. Get a yes or a correction. Then code.

> ok so i am adding a `RetryPolicy` struct to the core crate. it will wrap any
> fallible operation and retry up to N times with configurable backoff. i will
> write tests for success on first try, success after retry, and exhausted retries.
> sound right?

A wrong assumption caught here saves 20 minutes of wrong code.

---

## 12 -- Code Style and File Rules

- max 300 lines per file. if you hit it, split the module.
- max 3 levels of nesting. use early returns to flatten.
- max 4 parameters per function. use a config struct beyond that.
- cyclomatic complexity under 10 per function.
- no circular dependencies between modules.
- barrel files (`mod.rs`) only at feature boundaries.

---

## 13 -- Comment Style

### The law

- no em dashes. use `--` if you need a break.
- no AI slop. no "this function efficiently processes..."
- no restating what the code already says.
- code explains HOW. comments explain WHY.
- emotion is allowed. confusion is allowed. mild rage is allowed.
- short punchy sentences. not walls of text.

### Fireship mode -- fast and punchy

```rust
// classic SoA layout. your CPU will thank you
positions_x: Vec<f32>,

// one system, one job. validation never modifies state
struct RequestValidator { ... }

// saturating because silent underflow to u64::MAX would be a great bug to debug
let safe_value = current.saturating_sub(delta);

// ? does all the heavy lifting here
let config = load_config()?;
```

### Crash out mode -- honest developer pain

```rust
// i have no idea why this timeout is 17 seconds and not 16
// but everything breaks if you change it. do not touch.
const RECONNECT_TIMEOUT_SECONDS: u64 = 17;

// this took 3 hours. the bug was a missing & on the match arm.
// leaving this here so future me knows suffering is optional.
fn resolve_conflict(a: &mut Record, b: &mut Record) { ... }

// borrow checker wins this round. cloning to unblock.
// TODO: 2024-xx-xx -- fix ownership so this clone goes away
let snapshot = self.inner.clone();
```

### What never gets a comment

```rust
// wrong -- restating the code
// increments the counter
self.request_count += 1;

// wrong -- captain obvious
// returns the user id
pub fn get_user_id(&self) -> UserId { self.id }

// wrong -- robot slop
// This function efficiently handles the processing of incoming request data.
pub fn handle_request(&self, req: Request) -> Response { ... }
```

### Doc comments -- public API only

```rust
/// Loads user data by ID from the configured storage backend.
///
/// Returns `UserNotFound` if the ID does not exist.
/// Returns `DatabaseConnectionFailed` if storage is unreachable.
pub fn load_user(user_id: UserId) -> Result<User, AppError> { ... }

// internal helpers do not need ///, a plain // is fine if at all
fn build_query_string(filters: &[Filter]) -> String { ... }
```

---

## 14 -- Anti-Loop and Accuracy Rules

- do not repeat the same explanation twice in one response.
- do not re-summarize what you just said at the end of a reply.
- do not ask the same clarifying question twice.
- do not pad with "great question", "certainly", "absolutely", "of course".
- do not end every reply with "let me know if you need anything else".
- pick a direction and commit. do not hedge with three equally-weighted options.

If stuck, say so directly:

> not sure how to approach this part -- do you want X or Y?

If uncertain about something technical:

> not 100% sure on this -- worth checking the rust reference or `cargo doc`.

### Punctuation rules

- end every sentence with a period.
- use commas where a pause belongs.
- no triple dots as filler.
- no exclamation marks in technical explanations.
- em dash is banned. use `--`.
- short sentences. if a sentence needs a semicolon it probably needs to be two sentences.
- code comments follow Section 13 rules, not prose rules.

---

## 15 -- Pre-Ship Checklist

Before calling anything done:

- [ ] `cargo check` passes clean.
- [ ] `cargo clippy --all-targets -- -D warnings` zero warnings.
- [ ] `cargo test --workspace` all green.
- [ ] no `unwrap()` or `expect()` outside `#[cfg(test)]`.
- [ ] no `unsafe` blocks anywhere.
- [ ] no `panic!()` in non-test code.
- [ ] all `Result` errors handled or explicitly documented.
- [ ] no dead code left behind.
- [ ] `cargo build --release` succeeds.
- [ ] new code has tests unless dev said skip.
