pub mod server;
pub mod event_loop;
pub mod http;
pub mod client;
pub mod router;
pub mod config;
pub mod config_parser;

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};
pub static ERROR_TEMPLATE: OnceLock<String> = OnceLock::new();
pub static SESSION_STORE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();