pub mod greeting;
pub mod calculator;
pub mod timer;
pub mod debug;

// Re-export command functions for macro expansion
pub use greeting::greet;
pub use calculator::calculate_total;
pub use timer::{save_timer_session, get_timer_sessions, add_break, get_sessions_with_breaks};
pub use debug::test_db;
