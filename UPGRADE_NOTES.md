# Upgrade Notes: Dioxus 0.4.0 → 0.7.0-rc.0

This document summarizes the changes made to upgrade the shopping list application from Dioxus 0.4.0 to Dioxus 0.7.0-rc.0.

## Major Changes

### 1. Dependency Updates

**Cargo.toml:**
- Updated `dioxus` from `0.4.0` to `0.7.0-rc.0` with consolidated features
- Removed separate `dioxus-web`, `dioxus-router`, `dioxus-fullstack`, and `axum` dependencies
- Updated `dioxus-logger` from `0.4.1` to `0.5.1`
- Updated `dioxus-free-icons` from `0.7.0` to `0.10.0-alpha.1`
- Simplified features: `web = ["dioxus/web"]`, `server = ["sqlx", "tokio"]`

### 2. Configuration

**New Dioxus.toml:**
- Added Dioxus configuration file for project settings
- Configured web app title and resource handling

### 3. Project Structure Reorganization

**Assets:**
- Moved `src/style.css` to `assets/styling/main.css`
- CSS now loaded via `asset!()` macro

**Module Organization:**
- Created `src/views/` directory for page components
- Created `src/components/` directory for shared components
- Added proper `mod.rs` files with exports

### 4. API Changes

**Component Syntax:**
- Replaced `#[inline_props]` with `#[component]`
- Updated function signatures: `fn(cx: Scope, ...)` → `fn(...)`
- Removed `cx` parameter from all components
- Changed `render!` to `rsx!`

**State Management:**
- `use_state(cx, || value)` → `use_signal(|| value)`
- `UseState<T>` → `Signal<T>`
- `.get()` → `.read()`
- `.set(value)` → `*signal.write() = value`

**Effects:**
- `use_effect(&cx, deps, closure)` → `use_effect(move || { spawn(async move { ... }); })`
- Removed dependency tracking parameters

**Event Handlers:**
- Updated closure syntax to use `move |event|`
- Async operations now use `spawn(async move { ... })`

### 5. Server Functions (Dioxus 0.7 Modernization)

**Key Change: Removed `#[cfg(feature = "server")]` guards from server functions**

In Dioxus 0.7, the `#[server]` attribute automatically handles client/server separation, eliminating the need for manual feature guards on server functions themselves.

**common_types.rs:**
- Removed `#[cfg(feature = "server")]` guards from all server function bodies
- The `#[server]` attribute now handles conditional compilation automatically
- Server-only imports (like `use crate::postgres;`) are placed directly in server functions
- Module-level code with server-only dependencies still requires `#[cfg(feature = "server")]`
- This creates cleaner, more maintainable server function code

### 6. Component Props

**DialogWrapper:**
- Updated props structure: `#[derive(Props, Clone, PartialEq)]`
- Changed `&UseState<bool>` to `Signal<bool>`
- Updated element passing: `children: Element`

### 7. Router Changes

**main.rs:**
- Simplified app structure with new `App` component
- Router now uses `Router::<Route> {}` syntax
- CSS loaded via `document::Link` with `asset!()` macro

**Route Definitions:**
- Added `#[rustfmt::skip]` attribute for better formatting
- Updated route struct to include empty braces: `ShoppingListsPage {}`

### 8. Data Flow

**Async Operations:**
- All async operations wrapped in `spawn(async move { ... })`
- Cloned signals before moving into closures
- Fixed lifetime issues by restructuring match statements

**List/Item Rendering:**
- Changed from iterator-based rendering to explicit cloning
- Used `items.iter().map()` pattern for dynamic content
- Properly handled Signal borrowing in render contexts

## Key Migration Patterns

### State Usage
```rust
// Old (0.4.0)
let state = use_state(cx, || initial_value);
state.set(new_value);
let value = state.get();

// New (0.7.0)
let mut state = use_signal(|| initial_value);
*state.write() = new_value;
let value = state.read();
```

### Components
```rust
// Old (0.4.0)
#[inline_props]
fn MyComponent(cx: Scope, prop: String) -> Element {
    render! { div { "{prop}" } }
}

// New (0.7.0)
#[component]
fn MyComponent(prop: String) -> Element {
    rsx! { div { "{prop}" } }
}
```

### Effects
```rust
// Old (0.4.0)
use_effect(&cx, (), |_| async move {
    // async code
});

// New (0.7.0)
use_effect(move || {
    spawn(async move {
        // async code
    });
});
```

## Testing

The application successfully:
- Compiles without errors
- Runs with `dx serve`
- Handles API requests (get_lists, insert_new_list, etc.)
- Maintains all original functionality

## Dioxus 0.7 Server Function Modernization

The most significant improvement in Dioxus 0.7 is the elimination of manual `#[cfg(feature = "server")]` guards in server functions:

**Old Pattern (0.4.x):**
```rust
#[server(GetLists)]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    #[cfg(feature = "server")]
    use crate::postgres;
    match postgres::get_shopping_lists().await {
        Ok(lists) => Ok(lists),
        Err(e) => Err(ServerFnError::ServerError(
            "Could not retrieve Lists from Database".to_string(),
        ))
    }
}
```

**New Pattern (0.7.0):**
```rust
#[server(GetLists)]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    use crate::postgres;  // No cfg guard needed!
    match postgres::get_shopping_lists().await {
        Ok(lists) => Ok(lists),
        Err(e) => Err(ServerFnError::ServerError(
            "Could not retrieve Lists from Database".to_string(),
        ))
    }
}
```

### What Changed:
- **Removed all `#[cfg(feature = "server")]` guards from within server functions**
- The `#[server]` attribute now automatically handles server/client compilation
- Server-only imports can be used directly inside server functions
- Modules containing server-only dependencies still need `#[cfg(feature = "server")]`
- This eliminates boilerplate and reduces the chance of configuration errors

### Architecture:
- **postgres.rs module**: Still requires `#[cfg(feature = "server")]` because it contains `sqlx` dependencies
- **Server functions**: No longer need feature guards - the `#[server]` macro handles everything
- **main.rs**: Postgres module import still conditional: `#[cfg(feature = "server")] mod postgres;`

This modernization makes the code cleaner and follows Dioxus 0.7's "convention over configuration" approach.

## Database Schema Fix

**Issue Found During Upgrade:**
The database migration created a table called `items`, but the Rust code was referencing `shopping_list_items`. This mismatch caused runtime errors.

**Fix Applied:**
Updated all SQL queries in `postgres.rs` to use the correct table name `items` instead of `shopping_list_items`:
- `get_list_items()`: Changed `SELECT * FROM shopping_list_items` to `SELECT * FROM items`
- `create_list_item()`: Changed `INSERT INTO shopping_list_items` to `INSERT INTO items`
- `update_item_crossed()`: Changed `UPDATE shopping_list_items` to `UPDATE items`
- `clear_all_crossed()`: Changed `DELETE FROM shopping_list_items` to `DELETE FROM items`
- `delete_shopping_list()`: Changed `DELETE FROM shopping_list_items` to `DELETE FROM items`

**Database Troubleshooting:**
If you encounter table-related errors:
1. Verify tables exist: `\dt` in PostgreSQL
2. Check table structure: `\d items` 
3. Ensure migrations ran: The app calls `initialize_database()` on startup
4. Reset database if needed: Drop and recreate the database, then restart the app

## Notes

- Logger initialization updated to use `dioxus_logger::tracing::Level::INFO`
- Removed unused imports to clean up warnings
- Fixed all lifetime and borrowing issues with new Signal API
- Updated from `ssr` to `server` feature for modern Dioxus conventions
- Maintained backward compatibility for database operations